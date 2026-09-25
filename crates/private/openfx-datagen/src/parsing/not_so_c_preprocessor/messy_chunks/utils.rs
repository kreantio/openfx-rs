use crate::parsing::welp::{welp, welp_text};

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

/// The returned range doesn't include the `#endif` line itself, but the
/// iterator will have consumed it.
pub fn until_endif_exclusive_but_consuming_last_line<'a>(
    lines: &mut impl Iterator<Item = LineEx<'a>>,
) -> std::ops::Range<usize> {
    let mut content_range: Option<std::ops::Range<usize>> = None;

    loop {
        let Some(next_line) = lines.next() else {
            welp!();
        };
        if next_line.content == "#endif" {
            let Some(content_range) = content_range else {
                welp!();
            };
            return content_range;
        } else {
            content_range = match content_range {
                Some(r) => Some(r.start..next_line.start_offset + next_line.content.len()),
                None => {
                    Some(next_line.start_offset..next_line.start_offset + next_line.content.len())
                }
            };
        }
    }
}

pub fn until_typedef_close_exclusive_but_consuming_last_line<'a>(
    lines: &mut impl Iterator<Item = LineEx<'a>>,
    sym: &str,
) -> std::ops::Range<usize> {
    let mut content_range: Option<std::ops::Range<usize>> = None;

    loop {
        let Some(next_line) = lines.next() else {
            welp!();
        };
        let content = next_line.content.trim();
        if let Some(i_close) = content.find("}")
            && let Some(i_sym) = content.find(sym)
            && let Some(i_colon) = content.find(";")
            && i_close < i_sym
            && i_sym < i_colon
            && content[i_close + "}".len()..i_sym].trim().is_empty()
            && content[i_sym + sym.len()..i_colon].trim().is_empty()
            && content[i_colon + ";".len()..].trim().is_empty()
        {
            let Some(content_range) = content_range else {
                welp!();
            };
            return content_range;
        } else {
            content_range = match content_range {
                Some(r) => Some(r.start..next_line.start_offset + next_line.content.len()),
                None => {
                    Some(next_line.start_offset..next_line.start_offset + next_line.content.len())
                }
            };
        }
    }
}

pub fn until_multi_line_comment_close_exclusive_but_consuming_last_line_and_wanting_first_line<
    'a,
>(
    code: &str,
    lines: &mut impl Iterator<Item = LineEx<'a>>,
    first_line: LineEx<'a>,
) -> std::ops::Range<usize> {
    let Some(offset) = first_line.content.find("/*") else {
        welp!();
    };

    let mut content_range = (first_line.start_offset + offset + "/*".len())
        ..first_line.start_offset + first_line.content.len();
    while code.as_bytes().get(content_range.start) == Some(&b'*') {
        content_range.start += 1;
    }

    let mut current_line = first_line;
    loop {
        if let Some(i) = current_line.content.find("*/") {
            if !current_line.content[(i + "*/".len())..].trim().is_empty() {
                welp!();
            }
            content_range.end = current_line.start_offset + i;
            return trim_range(code, content_range);
        }

        let Some(next_line) = lines.next() else {
            welp!();
        };
        current_line = next_line;
    }
}

fn trim_range(s: &str, range: std::ops::Range<usize>) -> std::ops::Range<usize> {
    let content = &s[range.clone()];
    let start_trimmed = content.trim_start();
    let start = range.start + content.len() - start_trimmed.len();
    let end = range.start + content.trim_end().len();
    start..end
}

pub fn parse_c_integer_literal_expecting_u32(s: &str) -> u32 {
    if s.starts_with("-") {
        welp!();
    }
    if let Some(s) = s.strip_prefix("0x") {
        u32::from_str_radix(s, 16).expect(welp_text!())
    } else if s.starts_with("0") && s.len() > 1 {
        welp!();
    } else {
        s.parse::<u32>().expect(welp_text!())
    }
}

/// Find the final substring that could be a C symbol in the given line.
///
/// Author: GitHub Copilot / Kimi K3 (High)
/// Reviewed-by: Umaĵo
pub fn rfind_possible_symbol(line: &str) -> Option<&str> {
    static SYMBOL: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap());
    SYMBOL.find_iter(line).last().map(|m| m.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_until_endif_exclusive_but_consuming_last_line() {
        let cases = [
            (
                r#"#ifndef OFX_NO_DEFAULT_COLORSPACE_HEADER
#include "ofx-native-v1.5_aces-v1.3_ocio-v2.3.h"
#endif"#,
                r#"#include "ofx-native-v1.5_aces-v1.3_ocio-v2.3.h""#,
            ),
            (
                r#"#ifndef kOfxBitDepthHalf
/** @brief String used to label the OpenGL half float (16 bit floating
point) sample format */
  #define kOfxBitDepthHalf "OfxBitDepthHalf"
#endif"#,
                r#"/** @brief String used to label the OpenGL half float (16 bit floating
point) sample format */
  #define kOfxBitDepthHalf "OfxBitDepthHalf""#,
            ),
            // NOTE: `"\t"` before `#define`.
            (
                r#"#if defined(_WIN32)
	#define OfxExport extern __declspec(dllexport)
#else
	#define OfxExport extern
#endif"#,
                r#"	#define OfxExport extern __declspec(dllexport)
#else
	#define OfxExport extern"#,
            ),
        ];

        for (input, expected) in cases {
            let mut lines = LinesEx::new(input);
            lines.next();
            let content_range = until_endif_exclusive_but_consuming_last_line(&mut lines);
            pretty_assertions::assert_eq!(&input[content_range], expected);
            assert!(lines.next().is_none());
        }
    }

    #[test]
    fn test_until_typedef_close_exclusive_but_consuming_last_line() {
        let cases = [
            (
                r#"typedef struct OfxMultiThreadSuiteV1 {
  /**@brief Function to spawn SMP threads
…
  */
  OfxStatus (*multiThread)(OfxThreadFunctionV1 func,
			   unsigned int nThreads,
			   void *customArg);
//…
 } OfxMultiThreadSuiteV1;"#,
                "OfxMultiThreadSuiteV1",
                1,
                r#"  /**@brief Function to spawn SMP threads
…
  */
  OfxStatus (*multiThread)(OfxThreadFunctionV1 func,
			   unsigned int nThreads,
			   void *customArg);
//…"#,
            ),
            (
                r#"typedef struct OfxYUVAColourB {
  unsigned char y, u, v, a;
}OfxYUVAColourB;"#,
                "OfxYUVAColourB",
                1,
                r#"  unsigned char y, u, v, a;"#,
            ),
            (
                r#"typedef enum OfxDrawTextAlignment
{
	kOfxDrawTextAlignmentLeft     = 0x0001,
//…
	kOfxDrawTextAlignmentCenterV  = (kOfxDrawTextAlignmentTop | kOfxDrawTextAlignmentBaseline)
} OfxDrawTextAlignment;"#,
                "OfxDrawTextAlignment",
                2,
                r#"	kOfxDrawTextAlignmentLeft     = 0x0001,
//…
	kOfxDrawTextAlignmentCenterV  = (kOfxDrawTextAlignmentTop | kOfxDrawTextAlignmentBaseline)"#,
            ),
        ];

        for (input, sym, lines_skipped, expected) in cases {
            let mut lines = LinesEx::new(input);
            for _ in 0..lines_skipped {
                lines.next();
            }
            let content_range =
                until_typedef_close_exclusive_but_consuming_last_line(&mut lines, sym);
            pretty_assertions::assert_eq!(&input[content_range], expected);
            assert!(lines.next().is_none());
        }
    }

    #[test]
    fn test_trim_range() {
        let input = "prefix\u{2003}  cafe\u{301}\u{00a0}suffix";
        let range = "prefix".len()..input.len() - "suffix".len();

        pretty_assertions::assert_eq!(&input[trim_range(input, range)], "cafe\u{301}");
    }

    #[test]
    fn test_until_multi_line_comment_close_exclusive_but_consuming_last_line_and_wanting_first_line()
     {
        let cases = [
            (r#"/*hello world*/"#, "hello world"),
            (r#"/* hello world */"#, "hello world"),
            (r#"  /* hello world */  "#, "hello world"),
            (
                r#"/*
    hello world
*/"#,
                "hello world",
            ),
            (
                r#"/*hello
world*/"#,
                "hello\nworld",
            ),
            (
                r#"/* good
    day
    world */"#,
                "good\n    day\n    world",
            ),
        ];

        for (input, expected) in cases {
            let mut lines = LinesEx::new(input);
            let first_line = lines.next().unwrap();
            let content_range =
                until_multi_line_comment_close_exclusive_but_consuming_last_line_and_wanting_first_line(
                    input, &mut lines, first_line,
                );
            pretty_assertions::assert_eq!(&input[content_range], expected);
            assert!(lines.next().is_none());
        }
    }

    #[test]
    fn test_rfind_symbol() {
        let cases = [
            (
                r#"OfxStatus(*compileProgram)(const char   *pszProgramSource,"#,
                "pszProgramSource",
            ),
            (r#"int           fOptional,"#, "fOptional"),
            (r#"void         *pResult);"#, "pResult"),
        ];

        for (input, expected) in cases {
            pretty_assertions::assert_eq!(rfind_possible_symbol(input), Some(expected));
        }
    }
}
