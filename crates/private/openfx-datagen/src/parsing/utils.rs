use std::sync::LazyLock;

use regex::Regex;

use crate::parsing::types::{DefineValue, TypedIntegerLiteralCType};

pub struct LinesEx<'a> {
    text: &'a str,
    offset: usize,
}

pub struct LineEx<'a> {
    pub content: &'a str,
    pub start_offset: usize,
}

impl<'a> LinesEx<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            text: content,
            offset: 0,
        }
    }
}

impl<'a> Iterator for LinesEx<'a> {
    type Item = LineEx<'a>;

    /// Author: GitHub Copilot's tab completion
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.text.len() {
            return None;
        }
        let remaining_text = &self.text[self.offset..];
        if let Some(pos) = remaining_text.find('\n') {
            let mut line = &remaining_text[..pos];
            line = line.strip_suffix('\r').unwrap_or(line);
            let start_offset = self.offset;
            self.offset += pos + 1;
            Some(LineEx {
                content: line,
                start_offset,
            })
        } else {
            let start_offset = self.offset;
            self.offset = self.text.len();
            Some(LineEx {
                content: remaining_text,
                start_offset,
            })
        }
    }
}

fn parse_c_integer_literal_expecting_u32(s: &str) -> Result<u32, ()> {
    if s.starts_with("-") {
        return Err(());
    }
    if let Some(s) = s.strip_prefix("0x") {
        u32::from_str_radix(s, 16).map_err(|_| ())
    } else if s.starts_with("0") && s.len() > 1 {
        Err(())
    } else {
        s.parse::<u32>().map_err(|_| ())
    }
}

pub fn parse_define_value(value: &str) -> Result<DefineValue, ()> {
    static RE_STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^"(.*)"$"#).unwrap());
    static RE_INTEGER: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^([1-9][0-9]*|0[1-7]*|0[xX][\da-fA-F]+)$"#).unwrap());
    static RE_BOOLEAN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^(false|true)$"#).unwrap());
    static RE_TYPED_INTEGER: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^\(\((int)\)\s*([1-9][0-9]*|0[1-7]*|0[xX][\da-fA-F]+)\)$"#).unwrap()
    });
    static RE_SYMBOL: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^[a-zA-Z_][a-zA-Z0-9_]*$"#).unwrap());

    if let Some(g) = RE_STRING.captures(value) {
        let Some(value) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        if value.find("\\").is_some() {
            return Err(());
        }
        Ok(DefineValue::StringLiteral {
            value: value.to_owned(),
        })
    } else if let Some(g) = RE_INTEGER.captures(value) {
        let Some(value) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        Ok(DefineValue::IntegerLiteral {
            value: parse_c_integer_literal_expecting_u32(value)?,
        })
    } else if let Some(g) = RE_BOOLEAN.captures(value) {
        let Some(value) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        Ok(match value {
            "true" => DefineValue::BooleanLiteral { value: true },
            "false" => DefineValue::BooleanLiteral { value: false },
            _ => unreachable!(),
        })
    } else if let Some(g) = RE_TYPED_INTEGER.captures(value) {
        let Some(ty) = g.get(1).map(|m| m.as_str()) else {
            unreachable!();
        };
        let Some(value) = g.get(2).map(|m| m.as_str()) else {
            unreachable!();
        };
        Ok(DefineValue::TypedIntegerLiteral {
            c_type: TypedIntegerLiteralCType::try_from(ty)?,
            value: parse_c_integer_literal_expecting_u32(value)?,
        })
    } else if let Some(g) = RE_SYMBOL.captures(value) {
        let Some(sym) = g.get(0).map(|m| m.as_str()) else {
            unreachable!();
        };
        Ok(DefineValue::Symbol {
            value: sym.to_owned(),
        })
    } else {
        Err(())
    }
}

pub fn clean_comment(comment: &str) -> String {
    let comment = comment.trim();

    if let Some(stripped) = comment.strip_prefix("//") {
        return stripped.trim_start().to_owned();
    }

    let comment = comment
        .strip_prefix("/*")
        .expect("`comment` should start with be a comment.");
    let comment = comment.trim_start_matches("*");
    let comment = comment
        .strip_suffix("*/")
        .expect("a multi-line comment should end with */");

    let mut lines: Vec<String> = comment.lines().map(String::from).collect();
    if lines.len() == 1 {
        return lines[0].trim().to_owned();
    }
    let range = 1..(lines.len() - 1);
    for line in &mut lines[range] {
        let l = line.trim_start().to_owned();
        if l.is_empty() {
        } else if l.trim_end() == "*" {
            *line = String::new();
        } else if let Some(stripped) = l.strip_prefix("* ") {
            *line = stripped.to_owned();
        }
    }

    lines.join("\n").trim().to_owned()
}

pub fn find_line_before(text: &str, cursor: usize) -> &str {
    text[..cursor]
        .rfind('\n')
        .map_or("", |pos| &text[pos + 1..cursor])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_define_value() {
        let cases = [
            (
                r#""OfxDrawSuite""#,
                DefineValue::StringLiteral {
                    value: "OfxDrawSuite".to_owned(),
                },
            ),
            (
                r#""Linear Rec.709 (sRGB)""#,
                DefineValue::StringLiteral {
                    value: "Linear Rec.709 (sRGB)".to_owned(),
                },
            ),
            (r#"0"#, DefineValue::IntegerLiteral { value: 0 }),
            (r#"0xFF08"#, DefineValue::IntegerLiteral { value: 0xFF08 }),
            (r#"true"#, DefineValue::BooleanLiteral { value: true }),
            (
                r#"((int)1)"#,
                DefineValue::TypedIntegerLiteral {
                    c_type: TypedIntegerLiteralCType::try_from("int").unwrap(),
                    value: 1,
                },
            ),
            (
                r#"((int) 1001)"#,
                DefineValue::TypedIntegerLiteral {
                    c_type: TypedIntegerLiteralCType::try_from("int").unwrap(),
                    value: 1001,
                },
            ),
        ];

        for (input, expected) in cases {
            pretty_assertions::assert_eq!(parse_define_value(input), Ok(expected));
        }
    }

    #[test]
    fn test_clean_comment() {
        let cases = [
            (
                r#"// Copyright OpenFX and contributors to the OpenFX project."#,
                r#"Copyright OpenFX and contributors to the OpenFX project."#,
            ),
            (r#"/*@{*/"#, r#"@{"#),
            (
                r#"/** @brief How time is specified within the OFX API */"#,
                r#"@brief How time is specified within the OFX API"#,
            ),
            (
                r#"/** @brief Blind declaration of an OFX image effect
*/"#,
                r#"@brief Blind declaration of an OFX image effect"#,
            ),
            (
                r#"/** @brief OFX suite that allows an effect to interact with an openGL window so as to provide custom interfaces.

*/"#,
                r#"@brief OFX suite that allows an effect to interact with an openGL window so as to provide custom interfaces."#,
            ),
            (
                r#"/** @brief Description of the plug-in to a user.

This is a string giving a potentially verbose description of the effect.
    
    - Valid Values - UTF8 string
    @propdef
    type: string
    dimension: 1
*/"#,
                r#"@brief Description of the plug-in to a user.

This is a string giving a potentially verbose description of the effect.
    
    - Valid Values - UTF8 string
    @propdef
    type: string
    dimension: 1"#,
            ),
            (
                r#"/** @brief Platform independent export macro.
 *
 * This macro is to be used before any symbol that is to be
 * exported from a plug-in. This is OS/compiler dependent.
 */"#,
                r#"@brief Platform independent export macro.

This macro is to be used before any symbol that is to be
exported from a plug-in. This is OS/compiler dependent."#,
            ),
            (
                r#"/**
   \addtogroup PropertiesGeneral
*/"#,
                r#"\addtogroup PropertiesGeneral"#,
            ),
        ];

        for (input, expected) in cases {
            pretty_assertions::assert_eq!(clean_comment(input), expected);
        }
    }
}
