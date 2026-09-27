use std::sync::LazyLock;

use regex::Regex;
use treesitter_types_c::{FromNode, Spanned, TranslationUnitChildren};

use crate::parsing::{
    preprocessing::preprocess_for_tree_sitter,
    treesitter_utils::extract_name_from_declaration,
    utils::{clean_comment, parse_define_value},
};

mod preprocessing;
mod treesitter_utils;
mod utils;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// There are nodes with unaddressed syntax that we do not handle yet. This
    /// generally means that the official OpenFX C headers have been updated
    /// and now contain syntax that was not previously used.
    HasUnaddressedNodes { nodes: Vec<UnadressedNode> },
}

#[derive(Debug)]
pub struct UnadressedNode {
    pub comment_above: Option<String>,
    pub code: String,
    pub details: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Bindings {
    pub copyright_comments: Vec<String>,
    pub items: Vec<RootItemWithCommentAbove>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum RootItemWithCommentAbove {
    Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        comment_above: Option<String>,
        item: RootItem,
    },
    StandaloneComment {
        comment: String,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum RootItem {
    Define {
        name: String,
        value: DefineValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },

    Todo {
        kind: String,
        code: String,
    },
}

impl RootItem {
    pub fn name(&self) -> &str {
        match self {
            RootItem::Define { name, .. } => name,
            _ => todo!(),
        }
    }
}

/// The value of a `#define` directive that appears in the C headers of the
/// OpenFX standard.
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum DefineValue {
    /// String literal inside the quotes. Its contents are guaranteed to be
    /// unescaped by panicking if the string contains `\` characters.
    StringLiteral { value: String },
    /// Integer literal.
    ///
    /// Currently, in the source code:
    /// - values can have the `0x` & `0X` prefix (but not the `0` prefix);
    /// - values are always not negative;
    /// - values can always be held by a `u32`.
    IntegerLiteral { value: u32 },
    /// `"false"` or `"true"`.
    BooleanLiteral { value: bool },
    /// Things like `#define kOfxStatFailed  ((int)1)` and
    /// `#define kOfxStatGPUOutOfMemory  ((int) 1001)`
    ///
    /// Currently, in the source code:
    /// - values do not use a prefix like `0x`;
    /// - values are always not negative;
    /// - values can always be held by a `u32`.
    TypedIntegerLiteral {
        ty: TypedIntegerLiteralType,
        value: u32,
    },
    /// e.g., `#define kOfxActionDescribeInteract kOfxActionDescribe`.
    Symbol { value: String },
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum TypedIntegerLiteralType {
    Int,
}

pub fn parse(code: &str) -> Result<Bindings, Error> {
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
            TranslationUnitChildren::PreprocInclude(preproc_include) => RootItem::Todo {
                kind: "PreprocInclude".to_owned(),
                code: text!(raw_node),
            },
            TranslationUnitChildren::TypeDefinition(type_definition) => RootItem::Todo {
                kind: "TypeDefinition".to_owned(),
                code: text!(raw_node),
            },
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

    if !unadressed_nodes.is_empty() {
        return Err(Error::HasUnaddressedNodes {
            nodes: unadressed_nodes,
        });
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

    Ok(Bindings {
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
