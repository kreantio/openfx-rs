mod enum_body_comment_collector;
mod struct_body_comment_collector;
mod utils;

use std::{collections::HashMap, iter::Peekable, sync::LazyLock};

use regex::Regex;

use crate::parsing::{
    not_so_c_preprocessor::messy_chunks::{
        enum_body_comment_collector::collect_comments_in_enum_body,
        struct_body_comment_collector::collect_comments_in_struct_body,
        utils::until_multi_line_comment_close_exclusive_but_consuming_last_line_and_wanting_first_line,
    },
    welp::{welp, welp_assert, welp_assert_eq, welp_text},
};
use utils::{
    LinesEx, parse_c_integer_literal_expecting_u32, until_endif_exclusive_but_consuming_last_line,
    until_typedef_close_exclusive_but_consuming_last_line,
};

pub use struct_body_comment_collector::StructBodyComments;

#[derive(Debug, PartialEq, Eq)]
pub enum MessyChunk<'a> {
    //======== Special Cases That will be Discarded ========//
    /// `#ifndef …` and `#define …` at the root of the header file.
    ///
    /// If `#pragma once` appears above it, that `#pragma` will be taken into
    /// account.
    RootIfndefDefine {
        /// `#pragma once` appears above it.
        has_pragma_once_above: bool,
    },
    /// `#endif` corresponding to the root `#ifndef …`.
    RootEndif,
    /// It represents the following code in `ofxColour.h`:
    /// ```c
    /// #ifndef OFX_NO_DEFAULT_COLORSPACE_HEADER
    /// #include "ofx-native-v1.5_aces-v1.3_ocio-v2.3.h"
    /// #endif
    /// ```
    ///
    /// Note: the multi-line comment chunk before it should be discarded as
    /// well.
    IfndefOfxNoDefaultColorspaceHeaderIncludeEndif,
    /// It represents the following code in `ofxGPURender.h`:
    ///
    /// ```c
    /// #ifndef kOfxBitDepthHalf
    /// /** … */
    ///   #define kOfxBitDepthHalf "OfxBitDepthHalf"
    /// #endif
    /// ```
    ///
    /// `/** … */` might span multiple lines.
    IfndefKOfxBitDepthHalfDefineEndif,
    /// It represents:
    /// ```c
    /// #ifdef __cplusplus
    /// extern "C" {
    /// #endif
    /// ```
    IfDefCPlusPlusExternCOpenEndif,
    /// It represents:
    /// ```c
    /// #ifdef __cplusplus
    /// }
    /// #endif
    /// ```
    IfDefCPlusPlusCloseEndIf,
    /// It represents the following code in `ofxCore.h`:
    /// ```c
    /// #if defined(_WIN32)
    ///     #define OfxExport extern __declspec(dllexport)
    /// #else
    ///     #define OfxExport extern
    /// #endif
    /// ```
    ///
    /// Note: the multi-line comment chunk before it should be discarded as
    /// well.
    IfDefinedWin32DefineElseDefineEndif,

    //======== (Top Level) C Comments ========//
    SingleLineComments(&'a str),
    MultiLineComment(&'a str),

    //======== Preprocessor Directives ========//
    Include(&'a str),
    Define {
        name: &'a str,
        value: DefineValue<'a>,
        comment: Option<&'a str>,
    },

    //======== C Code ========//
    CCodeStruct {
        code: &'a str,
        comments: StructBodyComments<'a>,
    },
    CCodeEnum {
        code: &'a str,
        variant_comments: HashMap<&'a str, &'a str>,
    },
    CCodeLine(&'a str),
}

/// The value of a `#define` directive that appears in the C headers of the
/// OpenFX standard.
#[derive(Debug, PartialEq, Eq)]
pub enum DefineValue<'a> {
    /// String literal inside the quotes. Its contents are guaranteed to be
    /// unescaped by panicking if the string contains `\` characters.
    StringLiteral(&'a str),
    /// Integer literal.
    ///
    /// Currently, in the source code:
    /// - values can have the `0x` & `0X` prefix (but not the `0` prefix);
    /// - values are always not negative;
    /// - values can always be held by a `u32`.
    IntegerLiteral(u32),
    /// `"false"` or `"true"`.
    BooleanLiteral(bool),
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
    Symbol(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedIntegerLiteralType {
    Int,
}

pub struct MessyChunkParseStream<'a> {
    code: &'a str,
    lines: Peekable<LinesEx<'a>>,
    is_at_beginning: bool,
}

impl<'a> MessyChunkParseStream<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            lines: LinesEx::new(code).peekable(),
            is_at_beginning: true,
        }
    }
}

impl<'a> Iterator for MessyChunkParseStream<'a> {
    type Item = MessyChunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self
            .lines
            .peek()
            .is_some_and(|l| l.content.trim().is_empty())
        {
            self.lines.next();
        }

        let mut line = self.lines.next()?;

        if self.is_at_beginning {
            let mut has_pragma_once_above = false;
            if line.content == "#pragma once" {
                has_pragma_once_above = true;
                line = self.lines.next().expect(welp_text!());
            }
            welp_assert!(line.content.starts_with("#ifndef "));
            let Some(next_line) = self.lines.next() else {
                welp!();
            };
            welp_assert!(next_line.content.starts_with("#define "));
            welp_assert_eq!(
                &line.content["#ifndef ".len()..],
                &next_line.content["#define ".len()..]
            );
            self.is_at_beginning = false;
            Some(MessyChunk::RootIfndefDefine {
                has_pragma_once_above,
            })
        } else
        // Use `starts_with` because there might be comments after the
        // directive.
        if line.content.starts_with("#endif") {
            while let Some(next_line) = self.lines.peek() {
                welp_assert!(next_line.content.trim().is_empty());
                self.lines.next();
            }
            Some(MessyChunk::RootEndif)
        } else if line.content == "#ifndef OFX_NO_DEFAULT_COLORSPACE_HEADER"
            || line.content == "#ifndef kOfxBitDepthHalf"
        {
            let content_range = until_endif_exclusive_but_consuming_last_line(&mut self.lines);

            Some(match line.content {
                "#ifndef OFX_NO_DEFAULT_COLORSPACE_HEADER" => {
                    static CONTENT_RE: LazyLock<Regex> =
                        LazyLock::new(|| regex::Regex::new(r#"\A#include ".+"\z"#).unwrap());
                    if !CONTENT_RE.is_match(&self.code[content_range]) {
                        welp!();
                    }
                    MessyChunk::IfndefOfxNoDefaultColorspaceHeaderIncludeEndif
                }
                "#ifndef kOfxBitDepthHalf" => {
                    static CONTENT_RE: LazyLock<Regex> = LazyLock::new(|| {
                        regex::Regex::new(r#"(?m)\A/\*(.|\n)+\*/\s*#define kOfxBitDepthHalf.+\z"#)
                            .unwrap()
                    });
                    if !CONTENT_RE.is_match(&self.code[content_range]) {
                        welp!();
                    }
                    MessyChunk::IfndefKOfxBitDepthHalfDefineEndif
                }
                _ => unreachable!(),
            })
        } else if line.content == "#ifdef __cplusplus" {
            let Some(next_line) = self.lines.next() else {
                welp!();
            };
            let Some(next_next_line) = self.lines.next() else {
                welp!();
            };
            welp_assert_eq!(next_next_line.content, "#endif");
            Some(match next_line.content {
                r#"extern "C" {"# => MessyChunk::IfDefCPlusPlusExternCOpenEndif,
                r#"}"# => MessyChunk::IfDefCPlusPlusCloseEndIf,
                _ => welp!(),
            })
        } else if line.content == r#"#if defined(_WIN32)"# {
            let content_range: std::ops::Range<usize> =
                until_endif_exclusive_but_consuming_last_line(&mut self.lines);

            static CONTENT_RE: LazyLock<Regex> = LazyLock::new(|| {
                regex::Regex::new(
                    r#"(?m)\A\s*#define OfxExport .+\n#else\n\s*#define OfxExport .+\z"#,
                )
                .unwrap()
            });
            if !CONTENT_RE.is_match(&self.code[content_range]) {
                welp!();
            }
            Some(MessyChunk::IfDefinedWin32DefineElseDefineEndif)
        } else if line.content.starts_with("//") {
            let mut content_range = line.start_offset..line.start_offset + line.content.len();
            while self
                .lines
                .peek()
                .is_some_and(|line| line.content.starts_with("//"))
            {
                let next_line = self.lines.next().unwrap();
                content_range.end = next_line.start_offset + next_line.content.len();
            }
            Some(MessyChunk::SingleLineComments(&self.code[content_range]))
        } else if line.content.starts_with("/*") {
            let content_range =
                until_multi_line_comment_close_exclusive_but_consuming_last_line_and_wanting_first_line(
                    self.code,
                    &mut self.lines,
                    line,
                );
            Some(MessyChunk::MultiLineComment(&self.code[content_range]))
        } else if line.content.starts_with("#include ") {
            if line.content.contains("/*") {
                welp!();
            }
            let content = if let Some(comment_start) = line.content.find("//") {
                &line.content[..comment_start]
            } else {
                line.content
            };

            if let Some(l) = content.find("<")
                && let Some(r) = content.rfind(">")
            {
                Some(MessyChunk::Include(&content[l + 1..r]))
            } else if let Some(l) = content.find("\"")
                && let Some(r) = content.rfind("\"")
            {
                Some(MessyChunk::Include(&content[l + 1..r]))
            } else {
                welp!();
            }
        } else if line.content.starts_with("#define ") {
            let Some((name, value)) =
                line.content["#define ".len()..].split_once(char::is_whitespace)
            else {
                welp!();
            };
            let value = value.trim_start();

            let (comment, value) = if let Some(i) = value.find("//") {
                let comment = &value[i + "//".len()..];
                (Some(comment), value[..i].trim())
            } else if let Some(l) = value.find("/*")
                && let Some(r) = value.rfind("*/")
            {
                let comment = value[l + "/*".len()..r].trim_start_matches("*").trim();
                (Some(comment), value[..l].trim())
            } else {
                (None, value)
            };

            let value = parse_define_value(value);

            Some(MessyChunk::Define {
                name,
                value,
                comment,
            })
        } else if line.content.trim_start().starts_with("#") {
            welp!();
        } else if line.content.starts_with("typedef struct")
            && (line.content.rfind("{").is_some()
                || self.lines.peek().is_some_and(|l| l.content.trim() == "{"))
        {
            let start_offset = line.start_offset;

            let Some(g) = RE_SYMBOL_LOSSY.captures(&line.content["typedef struct".len()..]) else {
                welp!();
            };
            if line.content.rfind("{").is_none() {
                self.lines.next();
            }
            let Some(sym) = g.get(0).map(|m| m.as_str()) else {
                unreachable!();
            };
            let body_range =
                until_typedef_close_exclusive_but_consuming_last_line(&mut self.lines, sym);

            let body_code = &self.code[body_range];
            let code = &self.code[start_offset
                ..self
                    .lines
                    .peek()
                    .map_or(self.code.len(), |l| l.start_offset)]
                .trim();

            Some(MessyChunk::CCodeStruct {
                code,
                comments: collect_comments_in_struct_body(body_code),
            })
        } else if line.content.starts_with("typedef enum")
            && (line.content.rfind("{").is_some()
                || self.lines.peek().is_some_and(|l| l.content.trim() == "{"))
        {
            let start_offset = line.start_offset;

            let Some(g) = RE_SYMBOL_LOSSY.captures(&line.content["typedef enum".len()..]) else {
                welp!();
            };
            if line.content.rfind("{").is_none() {
                self.lines.next();
            }
            let Some(sym) = g.get(0).map(|m| m.as_str()) else {
                unreachable!();
            };
            let body_range =
                until_typedef_close_exclusive_but_consuming_last_line(&mut self.lines, sym);

            let body_code = &self.code[body_range];
            let code = &self.code[start_offset
                ..self
                    .lines
                    .peek()
                    .map_or(self.code.len(), |l| l.start_offset)]
                .trim();

            Some(MessyChunk::CCodeEnum {
                code,
                variant_comments: collect_comments_in_enum_body(body_code),
            })
        } else {
            Some(MessyChunk::CCodeLine(line.content))
        }
    }
}

static RE_SYMBOL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-zA-Z_][a-zA-Z0-9_]*$"#).unwrap());
static RE_SYMBOL_LOSSY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s+[a-zA-Z_][a-zA-Z0-9_]*"#).unwrap());

fn parse_define_value(value: &str) -> DefineValue<'_> {
    static RE_STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^"(.*)"$"#).unwrap());
    static RE_INTEGER: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^([1-9][0-9]*|0[1-7]*|0[xX][\da-fA-F]+)$"#).unwrap());
    static RE_BOOLEAN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^(false|true)$"#).unwrap());
    static RE_TYPED_INTEGER: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^\(\((int)\)\s*([1-9][0-9]*|0[1-7]*|0[xX][\da-fA-F]+)\)$"#).unwrap()
    });

    if let Some(g) = RE_STRING.captures(value) {
        let Some(value) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        if value.find("\\").is_some() {
            welp!();
        }
        DefineValue::StringLiteral(value)
    } else if let Some(g) = RE_INTEGER.captures(value) {
        let Some(value) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        DefineValue::IntegerLiteral(parse_c_integer_literal_expecting_u32(value))
    } else if let Some(g) = RE_BOOLEAN.captures(value) {
        let Some(value) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        match value {
            "true" => DefineValue::BooleanLiteral(true),
            "false" => DefineValue::BooleanLiteral(false),
            _ => unreachable!(),
        }
    } else if let Some(g) = RE_TYPED_INTEGER.captures(value) {
        let Some(ty) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        let Some(value) = g.get(2).map(|m| m.as_str()) else {
            unreachable!();
        };
        let ty = match ty {
            "int" => TypedIntegerLiteralType::Int,
            _ => welp!(),
        };
        DefineValue::TypedIntegerLiteral {
            ty,
            value: parse_c_integer_literal_expecting_u32(value),
        }
    } else if let Some(g) = RE_SYMBOL.captures(value) {
        let Some(sym) = g.get(0).map(|m| m.as_str()) else {
            unreachable!();
        };
        DefineValue::Symbol(sym)
    } else {
        welp!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn wrap_input(input: &str) -> String {
        [r#"#ifndef TEST_H"#, r#"#define TEST_H"#, input, r#"#endif"#].join("\n")
    }

    fn wrap_expected<'a>(expected: Vec<MessyChunk<'a>>) -> Vec<MessyChunk<'a>> {
        let mut result = Vec::new();
        result.push(MessyChunk::RootIfndefDefine {
            has_pragma_once_above: false,
        });
        result.extend(expected);
        result.push(MessyChunk::RootEndif);
        result
    }

    #[test]
    fn test_root_ifndef_define_and_endif() {
        pretty_assertions::assert_eq!(
            MessyChunkParseStream::new(&wrap_input("")).collect::<Vec<_>>(),
            wrap_expected(vec![]),
        );

        let cases = [(
            r#"#pragma once
#ifndef _ofxOpenGLRender_h_
#define _ofxOpenGLRender_h_

// Copyright OpenFX and Contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause

#endif  // _ofxOpenGLRender_h_
"#,
            vec![
                MessyChunk::RootIfndefDefine {
                    has_pragma_once_above: true,
                },
                MessyChunk::SingleLineComments(
                    r#"// Copyright OpenFX and Contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause"#,
                ),
                MessyChunk::RootEndif,
            ],
        )];

        for (input, expected) in cases {
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(input).collect::<Vec<_>>(),
                expected,
            );
        }
    }

    #[test]
    fn test_include() {
        let cases = [
            (r#"#include "foo""#, vec![MessyChunk::Include("foo")]),
            (
                r#"#include <foo>
#include "bar"

#include <baz>"#,
                vec![
                    MessyChunk::Include("foo"),
                    MessyChunk::Include("bar"),
                    MessyChunk::Include("baz"),
                ],
            ),
            (
                r#"#include "stddef.h" // for size_t"#,
                vec![MessyChunk::Include("stddef.h")],
            ),
            (
                r#"#include <limits.h> // for INT_MIN & INT_MAX"#,
                vec![MessyChunk::Include("limits.h")],
            ),
        ];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    #[test]
    fn test_define() {
        let cases = [
            (
                r#"#define kOfxDrawSuite "OfxDrawSuite""#,
                vec![MessyChunk::Define {
                    name: "kOfxDrawSuite",
                    value: DefineValue::StringLiteral("OfxDrawSuite"),
                    comment: None,
                }],
            ),
            (
                r#"#define kOfxColourspaceLinRec709SrgbLabel "Linear Rec.709 (sRGB)""#,
                vec![MessyChunk::Define {
                    name: "kOfxColourspaceLinRec709SrgbLabel",
                    value: DefineValue::StringLiteral("Linear Rec.709 (sRGB)"),
                    comment: None,
                }],
            ),
            (
                r#"#define kOfxStatOK 0"#,
                vec![MessyChunk::Define {
                    name: "kOfxStatOK",
                    value: DefineValue::IntegerLiteral(0),
                    comment: None,
                }],
            ),
            (
                r#"#define kOfxKey_BackSpace 0xFF08"#,
                vec![MessyChunk::Define {
                    name: "kOfxKey_BackSpace",
                    value: DefineValue::IntegerLiteral(0xFF08),
                    comment: None,
                }],
            ),
            (
                r#"#define kOfxKey_Multi_key		0xFF20  /* Multi-key character compose */"#,
                vec![MessyChunk::Define {
                    name: "kOfxKey_Multi_key",
                    value: DefineValue::IntegerLiteral(0xFF20),
                    comment: Some("Multi-key character compose"),
                }],
            ),
            (
                r#"#define kOfxColourspaceRawIsData true"#,
                vec![MessyChunk::Define {
                    name: "kOfxColourspaceRawIsData",
                    value: DefineValue::BooleanLiteral(true),
                    comment: None,
                }],
            ),
            (
                r#"#define kOfxStatFailed  ((int)1)"#,
                vec![MessyChunk::Define {
                    name: "kOfxStatFailed",
                    value: DefineValue::TypedIntegerLiteral {
                        ty: TypedIntegerLiteralType::Int,
                        value: 1,
                    },
                    comment: None,
                }],
            ),
            (
                r#"#define kOfxStatGPUOutOfMemory  ((int) 1001)"#,
                vec![MessyChunk::Define {
                    name: "kOfxStatGPUOutOfMemory",
                    value: DefineValue::TypedIntegerLiteral {
                        ty: TypedIntegerLiteralType::Int,
                        value: 1001,
                    },
                    comment: None,
                }],
            ),
            (
                r#"#define kOfxStatGLRenderFailed ((int) 1002) /* for backward compatibility */"#,
                vec![MessyChunk::Define {
                    name: "kOfxStatGLRenderFailed",
                    value: DefineValue::TypedIntegerLiteral {
                        ty: TypedIntegerLiteralType::Int,
                        value: 1002,
                    },
                    comment: Some("for backward compatibility"),
                }],
            ),
        ];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    #[test]
    fn test_comments() {
        let cases = [
            (
                r#"// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause"#,
                vec![MessyChunk::SingleLineComments(
                    r#"// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause"#,
                )],
            ),
            (
                r#"/** @file ofxDrawSuite.h
API for host- and GPU API-independent drawing.
@version Added in OpenFX 1.5
*/


/** @brief the string that names the DrawSuite, passed to OfxHost::fetchSuite */
#define kOfxDrawSuite "OfxDrawSuite""#,
                vec![
                    MessyChunk::MultiLineComment(
                        r#"@file ofxDrawSuite.h
API for host- and GPU API-independent drawing.
@version Added in OpenFX 1.5"#,
                    ),
                    MessyChunk::MultiLineComment(
                        r#"@brief the string that names the DrawSuite, passed to OfxHost::fetchSuite"#,
                    ),
                    MessyChunk::Define {
                        name: "kOfxDrawSuite",
                        value: DefineValue::StringLiteral("OfxDrawSuite"),
                        comment: None,
                    },
                ],
            ),
        ];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    #[test]
    fn test_special_cases() {
        let cases = [(
            r#"// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause


#include "stddef.h" // for size_t
#include <limits.h> // for INT_MIN & INT_MAX

#ifdef __cplusplus
extern "C" {
#endif

/** @file ofxCore.h
Contains the core OFX architectural struct and function definitions. For more details on the basic OFX architecture, see \ref Architecture.
*/


/** @brief Platform independent export macro.
 *
 * This macro is to be used before any symbol that is to be
 * exported from a plug-in. This is OS/compiler dependent.
 */
#if defined(_WIN32)
	#define OfxExport extern __declspec(dllexport)
#else
	#define OfxExport extern
#endif

#ifdef __cplusplus
}
#endif
"#,
            vec![
                MessyChunk::SingleLineComments(
                    r#"// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause"#,
                ),
                MessyChunk::Include("stddef.h"),
                MessyChunk::Include("limits.h"),
                MessyChunk::IfDefCPlusPlusExternCOpenEndif,
                MessyChunk::MultiLineComment(
                    r#"@file ofxCore.h
Contains the core OFX architectural struct and function definitions. For more details on the basic OFX architecture, see \ref Architecture."#,
                ),
                MessyChunk::MultiLineComment(
                    r#"@brief Platform independent export macro.
 *
 * This macro is to be used before any symbol that is to be
 * exported from a plug-in. This is OS/compiler dependent."#,
                ),
                MessyChunk::IfDefinedWin32DefineElseDefineEndif,
                MessyChunk::IfDefCPlusPlusCloseEndIf,
            ],
        )];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    #[test]
    fn test_c_code_struct() {
        const CASE_1_CODE: &str = r#"typedef struct OfxHost {
  /** @brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded.
   */
  OfxPropertySetHandle host;

  /** @brief The function which the plug-in uses to fetch suites from the host.
…
  */
  const void *(*fetchSuite)(OfxPropertySetHandle host, const char *suiteName, int suiteVersion);
} OfxHost;"#;
        const CASE_2_CODE: &str = r#"typedef struct OfxOpenCLProgramSuiteV1 {
    /** @brief Compiles the OpenCL program */
    OfxStatus(*compileProgram)(const char   *pszProgramSource,
        int           fOptional,          // if non-zero, host may skip compiling on this call
        void         *pResult);           // cast to cl_program*
} OfxOpenCLProgramSuiteV1;"#;
        const CASE_3_CODE: &str = r#"typedef struct OfxInteractSuiteV1 {	
  /** @brief Requests an openGL buffer swap on the interact instance */
  OfxStatus (*interactSwapBuffers)(OfxInteractHandle interactInstance);

  /** @brief Requests a redraw of the interact instance */
  OfxStatus (*interactRedraw)(OfxInteractHandle interactInstance);

  /** @brief Gets the property set handle for this interact handle */
  OfxStatus (*interactGetPropertySet)(OfxInteractHandle interactInstance,
				      OfxPropertySetHandle *property);
} OfxInteractSuiteV1;"#;

        let cases = [
            (
                CASE_1_CODE,
                vec![MessyChunk::CCodeStruct {
                    code: CASE_1_CODE,
                    comments: StructBodyComments {
                        field_comments: HashMap::from([
                            (
                                "host",
                                r#"@brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded."#,
                            ),
                            (
                                "fetchSuite",
                                r#"@brief The function which the plug-in uses to fetch suites from the host.
…"#,
                            ),
                        ]),
                        fn_field_parameter_comments: Default::default(),
                    },
                }],
            ),
            (
                CASE_2_CODE,
                vec![MessyChunk::CCodeStruct {
                    code: CASE_2_CODE,
                    comments: StructBodyComments {
                        field_comments: HashMap::from([(
                            "compileProgram",
                            r#"@brief Compiles the OpenCL program"#,
                        )]),
                        fn_field_parameter_comments: HashMap::from([
                            (
                                ("compileProgram", "fOptional"),
                                "if non-zero, host may skip compiling on this call",
                            ),
                            (("compileProgram", "pResult"), "cast to cl_program*"),
                        ]),
                    },
                }],
            ),
            (
                CASE_3_CODE,
                vec![MessyChunk::CCodeStruct {
                    code: CASE_3_CODE,
                    comments: StructBodyComments {
                        field_comments: HashMap::from([
                            (
                                "interactSwapBuffers",
                                r#"@brief Requests an openGL buffer swap on the interact instance"#,
                            ),
                            (
                                "interactRedraw",
                                r#"@brief Requests a redraw of the interact instance"#,
                            ),
                            (
                                "interactGetPropertySet",
                                r#"@brief Gets the property set handle for this interact handle"#,
                            ),
                        ]),
                        fn_field_parameter_comments: Default::default(),
                    },
                }],
            ),
        ];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    #[test]
    fn test_c_code_enum() {
        const CASE_1_CODE: &str = r#"typedef enum OfxStandardColour
        {
        	kOfxStandardColourOverlayBackground,
        	kOfxStandardColourOverlayActive,
        } OfxStandardColour;"#;

        const CASE_2_CODE: &str = r#"typedef enum OfxDrawLineStipplePattern
{
	kOfxDrawLineStipplePatternSolid,	// -----
	kOfxDrawLineStipplePatternDot,		// .....
} OfxDrawLineStipplePattern;"#;

        let cases = vec![
            (
                CASE_1_CODE,
                vec![MessyChunk::CCodeEnum {
                    code: CASE_1_CODE,
                    variant_comments: Default::default(),
                }],
            ),
            (
                CASE_2_CODE,
                vec![MessyChunk::CCodeEnum {
                    code: CASE_2_CODE,
                    variant_comments: HashMap::from([
                        ("kOfxDrawLineStipplePatternSolid", "-----"),
                        ("kOfxDrawLineStipplePatternDot", "....."),
                    ]),
                }],
            ),
        ];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    #[test]
    fn test_c_code_line() {
        let cases = vec![(
            r#"/** @brief Blind data structure to manipulate sets of properties through */
typedef struct OfxPropertySetStruct *OfxPropertySetHandle;

/** @brief OFX status return type */
typedef int OfxStatus;"#,
            vec![
                MessyChunk::MultiLineComment(
                    "@brief Blind data structure to manipulate sets of properties through",
                ),
                MessyChunk::CCodeLine("typedef struct OfxPropertySetStruct *OfxPropertySetHandle;"),
                MessyChunk::MultiLineComment("@brief OFX status return type"),
                MessyChunk::CCodeLine("typedef int OfxStatus;"),
            ],
        )];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    #[test]
    fn test_combined() {
        let cases = [(
            r#"// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause


#include "stddef.h" // for size_t
#include <limits.h> // for INT_MIN & INT_MAX

#ifdef __cplusplus
extern "C" {
#endif

/** @file ofxCore.h
Contains the core OFX architectural struct and function definitions. For more details on the basic OFX architecture, see \ref Architecture.
*/


/** @brief Platform independent export macro.
 *
 * This macro is to be used before any symbol that is to be
 * exported from a plug-in. This is OS/compiler dependent.
 */
#if defined(_WIN32)
	#define OfxExport extern __declspec(dllexport)
#else
	#define OfxExport extern
#endif

/** @brief Blind data structure to manipulate sets of properties through */
typedef struct OfxPropertySetStruct *OfxPropertySetHandle;

/** @brief OFX status return type */
typedef int OfxStatus;

/** @brief Generic host structure passed to OfxPlugin::setHost function

    This structure contains what is needed by a plug-in to bootstrap its connection
    to the host.
*/
typedef struct OfxHost {
  /** @brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded.
   */
  OfxPropertySetHandle host;

  /** @brief The function which the plug-in uses to fetch suites from the host.

      \arg \c host          the host the suite is being fetched from this \em must be the \e host member of the OfxHost struct containing fetchSuite.
      \arg \c suiteName     ASCII string labelling the host supplied API
      \arg \c suiteVersion  version of that suite to fetch

      Any API fetched will be valid while the binary containing the plug-in is loaded.

      Repeated calls to fetchSuite with the same parameters will return the same pointer.

      It is recommended that hosts should return the same host and suite pointers to all plugins
      in the same shared lib or bundle.

      returns
         - NULL if the API is unknown (either the api or the version requested),
	 - pointer to the relevant API if it was found
  */
  const void *(*fetchSuite)(OfxPropertySetHandle host, const char *suiteName, int suiteVersion);
} OfxHost;
 
/** @brief String used to label signed 32 bit floating point samples */
#define kOfxBitDepthFloat "OfxBitDepthFloat"

/**
   \defgroup StatusCodes Status Codes

These strings are used to identify error states within ofx, they are returned
by various host suite functions, as well as plug-in functions. The valid return codes
for each function are documented with that function.
*/
/*@{*/

/**
   \defgroup StatusCodesGeneral General Status Codes

General status codes start at 1 and continue until 999

*/
/*@{*/

/** @brief Status code indicating all was fine */
#define kOfxStatOK 0

/** @brief Status error code for a failed operation. */
#define kOfxStatFailed  ((int)1)

/*@}*/

/*@}*/

#ifdef __cplusplus
}
#endif

/** @mainpage OFX : Open Plug-Ins For Special Effects

This page represents the automatically extracted HTML documentation of the source headers for the OFX Image Effect API.
The documentation was extracted by doxygen (http://www.doxygen.org).
A more complete reference manual is https://openfx.readthedocs.io .

*/"#,
            vec![
                MessyChunk::SingleLineComments(
                    r#"// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause"#,
                ),
                MessyChunk::Include("stddef.h"),
                MessyChunk::Include("limits.h"),
                MessyChunk::IfDefCPlusPlusExternCOpenEndif,
                MessyChunk::MultiLineComment(
                    r#"@file ofxCore.h
Contains the core OFX architectural struct and function definitions. For more details on the basic OFX architecture, see \ref Architecture."#,
                ),
                MessyChunk::MultiLineComment(
                    r#"@brief Platform independent export macro.
 *
 * This macro is to be used before any symbol that is to be
 * exported from a plug-in. This is OS/compiler dependent."#,
                ),
                MessyChunk::IfDefinedWin32DefineElseDefineEndif,
                MessyChunk::MultiLineComment(
                    r#"@brief Blind data structure to manipulate sets of properties through"#,
                ),
                MessyChunk::CCodeLine(
                    r#"typedef struct OfxPropertySetStruct *OfxPropertySetHandle;"#,
                ),
                MessyChunk::MultiLineComment(r#"@brief OFX status return type"#),
                MessyChunk::CCodeLine(r#"typedef int OfxStatus;"#),
                MessyChunk::MultiLineComment(
                    r#"@brief Generic host structure passed to OfxPlugin::setHost function

    This structure contains what is needed by a plug-in to bootstrap its connection
    to the host."#,
                ),
                MessyChunk::CCodeStruct {
                    code: r#"typedef struct OfxHost {
  /** @brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded.
   */
  OfxPropertySetHandle host;

  /** @brief The function which the plug-in uses to fetch suites from the host.

      \arg \c host          the host the suite is being fetched from this \em must be the \e host member of the OfxHost struct containing fetchSuite.
      \arg \c suiteName     ASCII string labelling the host supplied API
      \arg \c suiteVersion  version of that suite to fetch

      Any API fetched will be valid while the binary containing the plug-in is loaded.

      Repeated calls to fetchSuite with the same parameters will return the same pointer.

      It is recommended that hosts should return the same host and suite pointers to all plugins
      in the same shared lib or bundle.

      returns
         - NULL if the API is unknown (either the api or the version requested),
	 - pointer to the relevant API if it was found
  */
  const void *(*fetchSuite)(OfxPropertySetHandle host, const char *suiteName, int suiteVersion);
} OfxHost;"#,
                    comments: StructBodyComments {
                        field_comments: HashMap::from([
                            (
                                "host",
                                r#"@brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded."#,
                            ),
                            (
                                "fetchSuite",
                                r#"@brief The function which the plug-in uses to fetch suites from the host.

      \arg \c host          the host the suite is being fetched from this \em must be the \e host member of the OfxHost struct containing fetchSuite.
      \arg \c suiteName     ASCII string labelling the host supplied API
      \arg \c suiteVersion  version of that suite to fetch

      Any API fetched will be valid while the binary containing the plug-in is loaded.

      Repeated calls to fetchSuite with the same parameters will return the same pointer.

      It is recommended that hosts should return the same host and suite pointers to all plugins
      in the same shared lib or bundle.

      returns
         - NULL if the API is unknown (either the api or the version requested),
	 - pointer to the relevant API if it was found"#,
                            ),
                        ]),
                        fn_field_parameter_comments: Default::default(),
                    },
                },
                MessyChunk::MultiLineComment(
                    r#"@brief String used to label signed 32 bit floating point samples"#,
                ),
                MessyChunk::Define {
                    name: "kOfxBitDepthFloat",
                    value: DefineValue::StringLiteral("OfxBitDepthFloat"),
                    comment: None,
                },
                MessyChunk::MultiLineComment(
                    r#"\defgroup StatusCodes Status Codes

These strings are used to identify error states within ofx, they are returned
by various host suite functions, as well as plug-in functions. The valid return codes
for each function are documented with that function."#,
                ),
                MessyChunk::MultiLineComment(r#"@{"#),
                MessyChunk::MultiLineComment(
                    r#"\defgroup StatusCodesGeneral General Status Codes

General status codes start at 1 and continue until 999"#,
                ),
                MessyChunk::MultiLineComment(r#"@{"#),
                MessyChunk::MultiLineComment(r#"@brief Status code indicating all was fine"#),
                MessyChunk::Define {
                    name: "kOfxStatOK",
                    value: DefineValue::IntegerLiteral(0),
                    comment: None,
                },
                MessyChunk::MultiLineComment(r#"@brief Status error code for a failed operation."#),
                MessyChunk::Define {
                    name: "kOfxStatFailed",
                    value: DefineValue::TypedIntegerLiteral {
                        ty: TypedIntegerLiteralType::Int,
                        value: 1,
                    },
                    comment: None,
                },
                MessyChunk::MultiLineComment(r#"@}"#),
                MessyChunk::MultiLineComment(r#"@}"#),
                MessyChunk::IfDefCPlusPlusCloseEndIf,
                MessyChunk::MultiLineComment(
                    r#"@mainpage OFX : Open Plug-Ins For Special Effects

This page represents the automatically extracted HTML documentation of the source headers for the OFX Image Effect API.
The documentation was extracted by doxygen (http://www.doxygen.org).
A more complete reference manual is https://openfx.readthedocs.io ."#,
                ),
            ],
        )];

        for (input, expected) in cases {
            let input_wrapped = wrap_input(input);
            let expected_wrapped = wrap_expected(expected);
            pretty_assertions::assert_eq!(
                MessyChunkParseStream::new(&input_wrapped).collect::<Vec<_>>(),
                expected_wrapped,
            );
        }
    }

    fn test_real_file(content: &str) {
        let stream = MessyChunkParseStream::new(content);
        for _ in stream {}
    }

    crate::test_fixtures::real_c_headers::make_test_real_files!(
        crate::parsing::not_so_c_preprocessor::messy_chunks::test::test_real_file
    );
}
