use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;
use treesitter_types_c::{FromNode as _, Spanned as _, TranslationUnitChildren};

pub use crate::parsing::types::*;
use crate::parsing::{
    preprocessing::preprocess_for_tree_sitter,
    treesitter_utils::{extract_name_from_declaration, parse_type_definition},
    utils::{clean_comment, parse_define_value},
};

mod preprocessing;
mod treesitter_utils;
mod types;
mod utils;

#[derive(Debug, snafu::Snafu, Default)]
pub struct Error {
    /// There are nodes with unaddressed syntax that we do not handle yet. This
    /// generally means that the official OpenFX C headers have been updated
    /// and now contain syntax that was not previously used.
    unaddressed_nodes: Vec<UnadressedNode>,

    unexpected_includes: HashSet<String>,
}

impl Error {
    fn is_empty(&self) -> bool {
        self.unaddressed_nodes.is_empty() && self.unexpected_includes.is_empty()
    }
}

#[derive(Debug)]
pub struct UnadressedNode {
    pub comment_above: Option<String>,
    pub code: String,
    pub details: Option<String>,
}

static UNINTERESTING_INCLUDES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["limits.h", "stddef.h"]));

pub fn parse(code: &str) -> Result<BindingsUnprocessed, Error> {
    let code = preprocess_for_tree_sitter(code);

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("`set_language` should not fail.");
    let tree = parser.parse(&code, None).expect(
        "OpenFX official C headers should be parseable by tree-sitter (after preprocessing).",
    );
    let root_node = tree.root_node();
    if root_node.has_error() {
        panic!("OpenFX official C headers should not have syntax errors (after preprocessing).");
    }

    let mut cursor = root_node.walk();

    let mut unprocessed_includes: HashSet<String> = HashSet::new();
    let mut copyright_comments: Vec<String> = vec![];
    let mut items: Vec<RootItemWithCommentAbove> = vec![];
    let mut last_comment: Option<String> = None;

    macro_rules! text {
        ($raw_node:expr) => {
            $raw_node
                .utf8_text(code.as_bytes())
                .expect("`utf8_text` should not fail.")
                .to_owned()
        };
    }
    macro_rules! text_from_span {
        ($span:expr) => {
            code[$span.start_byte..$span.end_byte].to_owned()
        };
    }

    let mut unadressed_nodes: Vec<UnadressedNode> = vec![];
    macro_rules! continue_unaddressed {
        ($comment_above:ident, $raw_node:ident) => {
            unadressed_nodes.push(UnadressedNode {
                comment_above: $comment_above,
                code: text!($raw_node),
                details: None,
            });
            continue;
        };
        ($comment_above:ident, $raw_node:ident, $details:expr) => {
            unadressed_nodes.push(UnadressedNode {
                comment_above: $comment_above,
                code: text!($raw_node),
                details: Some($details),
            });
            continue;
        };
    }

    for raw_node in root_node.named_children(&mut cursor) {
        if raw_node.kind() == "comment" {
            let text = text!(raw_node);
            if text.contains("copyright")
                || text.contains("Copyright")
                || text.contains("SPDX-License-Identifier")
            {
                copyright_comments.push(clean_comment(&text));
            } else if let text = text.trim_start()
                && text.starts_with("/**")
            {
                static SPECIAL_RE: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r#"(@(mainpage|page|file)|\\(defgroup|addtogroup))\b"#).unwrap()
                });

                if let Some(comment) = last_comment.take() {
                    items.push(RootItemWithCommentAbove::StandaloneComment { comment });
                }

                if SPECIAL_RE.is_match(text) {
                    items.push(RootItemWithCommentAbove::StandaloneComment {
                        comment: clean_comment(text),
                    });
                } else {
                    last_comment = Some(clean_comment(text));
                }
            }

            continue;
        }

        let comment_above = last_comment.take();

        let Ok(node) = TranslationUnitChildren::from_node(raw_node, code.as_bytes()) else {
            continue_unaddressed!(
                comment_above,
                raw_node,
                format!("not `TranslationUnitChildren`: {}", raw_node.kind())
            );
        };

        let item = match node {
            TranslationUnitChildren::Declaration(declaration) => {
                let Ok(name_span) = extract_name_from_declaration(&declaration) else {
                    continue_unaddressed!(comment_above, raw_node);
                };
                let name = text_from_span!(name_span);
                if ["OfxGetPlugin", "OfxGetNumberOfPlugins", "OfxSetHost"].contains(&name.as_str())
                {
                    // No idea what to do with them, so just skip them.
                    // TODO: don't skip them?
                    continue;
                } else {
                    continue_unaddressed!(comment_above, raw_node);
                }
            }
            TranslationUnitChildren::PreprocCall(preproc_call) => {
                if text_from_span!(preproc_call.directive.span).trim() == "#pragma"
                    && preproc_call
                        .argument
                        .is_some_and(|a| text_from_span!(a.span).trim() == "once")
                {
                    continue;
                } else {
                    continue_unaddressed!(comment_above, raw_node);
                }
            }
            TranslationUnitChildren::PreprocDef(preproc_def) => {
                let mut cursor = raw_node.walk();
                let comments = raw_node
                    .named_children(&mut cursor)
                    .filter(|node| node.kind() == "comment")
                    .collect::<Vec<_>>();

                let Some(value) = preproc_def.value.as_ref() else {
                    continue_unaddressed!(comment_above, raw_node);
                };
                if comments.len() > 1 {
                    continue_unaddressed!(comment_above, raw_node);
                }

                let value_str = text_from_span!(value.span);
                let Ok(value) = parse_define_value(value_str.trim()) else {
                    continue_unaddressed!(
                        comment_above,
                        raw_node,
                        format!("failed to parse define value: `{value_str}`")
                    );
                };

                RootItem::Define {
                    name: text_from_span!(preproc_def.name.span),
                    value,
                    comment: comments.first().map(|node| text!(node)),
                }
            }
            TranslationUnitChildren::PreprocIf(preproc_if) => {
                if text_from_span!(preproc_if.condition.span()).trim() == "defined(_WIN32)" {
                    continue;
                } else {
                    continue_unaddressed!(comment_above, raw_node);
                }
            }
            TranslationUnitChildren::PreprocIfdef(preproc_ifdef) => {
                if text_from_span!(preproc_ifdef.span)
                    .split_whitespace()
                    .next()
                    .is_some_and(|dir| dir == "#ifndef")
                    && ["OFX_NO_DEFAULT_COLORSPACE_HEADER", "kOfxBitDepthHalf"]
                        .contains(&text_from_span!(preproc_ifdef.name.span).trim())
                {
                    // `OFX_NO_DEFAULT_COLORSPACE_HEADER` (in `ofxColour.h`): It
                    // just includes `ofx-native-v1.5_aces-v1.3_ocio-v2.3.h` and
                    // doesn't use anything from that header afterwards. In our
                    // bindings, we can just generate bindings for that header
                    // and let users import it if they need it.

                    // `kOfxBitDepthHalf` (in `ofxGPURender.h`): It already
                    // exists in `ofxCore.h`.
                    continue;
                } else {
                    continue_unaddressed!(comment_above, raw_node);
                }
            }
            TranslationUnitChildren::PreprocInclude(preproc_include) => {
                let path = text_from_span!(preproc_include.path.span())
                    .trim()
                    .to_owned();
                let name = if let Some(stripped) = path
                    .strip_prefix("\"")
                    .and_then(|s| s.strip_suffix("\""))
                    .or_else(|| path.strip_prefix("<").and_then(|s| s.strip_suffix(">")))
                {
                    Some(stripped.to_string())
                } else {
                    path.strip_prefix("<")
                        .and_then(|s| s.strip_suffix(">"))
                        .map(|stripped| stripped.to_string())
                };
                let Some(name) = name else {
                    continue_unaddressed!(comment_above, raw_node);
                };
                if !UNINTERESTING_INCLUDES.contains(name.as_str()) {
                    unprocessed_includes.insert(name);
                }
                continue;
            }
            TranslationUnitChildren::TypeDefinition(type_definition) => {
                let Ok(item) = parse_type_definition(&raw_node, &code, &type_definition) else {
                    continue_unaddressed!(comment_above, raw_node);
                };
                items.push(RootItemWithCommentAbove::Item {
                    comment_above,
                    item,
                });
                continue;
            }
            TranslationUnitChildren::AttributedStatement(_)
            | TranslationUnitChildren::BreakStatement(_)
            | TranslationUnitChildren::CaseStatement(_)
            | TranslationUnitChildren::CompoundStatement(_)
            | TranslationUnitChildren::ContinueStatement(_)
            | TranslationUnitChildren::DoStatement(_)
            | TranslationUnitChildren::ExpressionStatement(_)
            | TranslationUnitChildren::ForStatement(_)
            | TranslationUnitChildren::FunctionDefinition(_)
            | TranslationUnitChildren::GotoStatement(_)
            | TranslationUnitChildren::IfStatement(_)
            | TranslationUnitChildren::LabeledStatement(_)
            | TranslationUnitChildren::LinkageSpecification(_)
            | TranslationUnitChildren::PreprocFunctionDef(_)
            | TranslationUnitChildren::ReturnStatement(_)
            | TranslationUnitChildren::SwitchStatement(_)
            | TranslationUnitChildren::TypeSpecifier(_)
            | TranslationUnitChildren::WhileStatement(_) => {
                continue_unaddressed!(comment_above, raw_node);
            }
        };

        items.push(RootItemWithCommentAbove::Item {
            comment_above,
            item,
        });
    }

    {
        let mut error = Error::default();

        if !unadressed_nodes.is_empty() {
            error.unaddressed_nodes = unadressed_nodes;
        }
        for include in &unprocessed_includes {
            if !include.starts_with("ofx") {
                error.unexpected_includes.insert(include.clone());
            }
        }

        if !error.is_empty() {
            return Err(error);
        }
    }

    {
        // dev

        let mut should_include_unfinished_items = true;
        should_include_unfinished_items = false;
        if !should_include_unfinished_items {
            items.retain(|i| match i {
                RootItemWithCommentAbove::Item { item, .. } => {
                    !matches!(item, RootItem::Todo { .. })
                }
                RootItemWithCommentAbove::StandaloneComment { .. } => true,
            })
        }
    }

    Ok(BindingsUnprocessed {
        unprocessed_includes,
        copyright_comments,
        items,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_real_file(content: &str) {
        if let Err(err) = parse(content) {
            panic!("{:?}", err)
        }
    }

    crate::test_fixtures::real_c_headers::make_test_real_files!(
        crate::parsing::tests::test_real_file
    );
}
