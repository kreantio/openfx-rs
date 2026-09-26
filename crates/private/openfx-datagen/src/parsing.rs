use treesitter_types_c::{FromNode, TranslationUnitChildren};

use crate::parsing::{preprocessing::preprocess_for_tree_sitter, utils::parse_define_value};

mod preprocessing;
mod utils;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// This syntax does not occur in the official OpenFX C headers, so we do
    /// not handle it yet.
    UnaddressedSyntax { code: String },
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct Bindings {
    copyright_comments: Vec<String>,
    items: Vec<RootItemWithCommentAbove>,
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
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

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum RootItem {
    Define {
        name: String,
        value: DefineValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },

    Todo {
        code: String,
    },
}

/// The value of a `#define` directive that appears in the C headers of the
/// OpenFX standard.
#[derive(Debug, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
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
        ($raw_node:ident) => {
            $raw_node
                .utf8_text(code.as_bytes())
                .expect("`utf8_text` should not fail.")
                .to_owned()
        };
    }
    macro_rules! text_fromspan {
        ($span:expr) => {
            code[$span.start_byte..$span.end_byte].to_owned()
        };
    }

    for raw_node in root_node.named_children(&mut cursor) {
        if raw_node.kind() == "comment" {
            let text = text!(raw_node);
            if text.contains("copyright")
                || text.contains("Copyright")
                || text.contains("SPDX-License-Identifier")
            {
                copyright_comments.push(text);
            } else {
                if let Some(comment) = last_comment.take() {
                    items.push(RootItemWithCommentAbove::StandaloneComment { comment });
                }
                last_comment = Some(text!(raw_node));
            }

            continue;
        }

        let Ok(node) = TranslationUnitChildren::from_node(raw_node, code.as_bytes()) else {
            tracing::warn!("not `TranslationUnitChildren`: {}", raw_node.kind());
            continue;
        };

        let item = match node {
            TranslationUnitChildren::AttributedStatement(attributed_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::BreakStatement(break_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::CaseStatement(case_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::CompoundStatement(compound_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::ContinueStatement(continue_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::Declaration(declaration) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::DoStatement(do_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::ExpressionStatement(expression_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::ForStatement(for_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::FunctionDefinition(function_definition) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::GotoStatement(goto_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::IfStatement(if_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::LabeledStatement(labeled_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::LinkageSpecification(linkage_specification) => {
                RootItem::Todo {
                    code: text!(raw_node),
                }
            }
            TranslationUnitChildren::PreprocCall(preproc_call) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::PreprocDef(preproc_def) => {
                let mut cursor = raw_node.walk();
                let comments = raw_node
                    .named_children(&mut cursor)
                    .filter(|node| node.kind() == "comment")
                    .collect::<Vec<_>>();

                let Some(value) = preproc_def.value.as_ref() else {
                    return Err(Error::UnaddressedSyntax {
                        code: text!(raw_node),
                    });
                };
                if comments.len() > 1 {
                    return Err(Error::UnaddressedSyntax {
                        code: text!(raw_node),
                    });
                }

                let value_str = text_fromspan!(value.span);
                let Ok(value) = parse_define_value(value_str.trim()) else {
                    return Err(Error::UnaddressedSyntax {
                        code: value_str.to_owned(),
                    });
                };

                RootItem::Define {
                    name: text_fromspan!(preproc_def.name.span),
                    value,
                    comment: comments.first().map(|node| text!(node)),
                }
            }
            TranslationUnitChildren::PreprocFunctionDef(preproc_function_def) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::PreprocIf(preproc_if) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::PreprocIfdef(preproc_ifdef) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::PreprocInclude(preproc_include) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::ReturnStatement(return_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::SwitchStatement(switch_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::TypeDefinition(type_definition) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::TypeSpecifier(type_specifier) => RootItem::Todo {
                code: text!(raw_node),
            },
            TranslationUnitChildren::WhileStatement(while_statement) => RootItem::Todo {
                code: text!(raw_node),
            },
        };

        items.push(RootItemWithCommentAbove::Item {
            comment_above: last_comment.take(),
            item,
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
