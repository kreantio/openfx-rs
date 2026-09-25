use std::collections::HashMap;

use crate::parsing::{
    not_so_c_preprocessor::messy_chunks::utils::{LinesEx, rfind_possible_symbol},
    welp::welp,
};

pub fn collect_comments_in_enum_body(body_code: &str) -> HashMap<&str, &str> {
    let mut lines = LinesEx::new(body_code);

    let mut variant_comments: HashMap<&str, &str> = HashMap::new();

    while let Some(line) = lines.next() {
        if line.content.contains("/*") {
            welp!();
        }
        let Some(comment_start) = line.content.find("//") else {
            continue;
        };
        let Some(sym) = rfind_possible_symbol(&line.content[..comment_start]) else {
            continue;
        };
        variant_comments.insert(sym, &line.content[comment_start + "//".len()..].trim());
    }

    variant_comments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_comments_in_enum_body() {
        let cases = [
            (
                r#"kOfxStandardColourOverlayBackground,
	kOfxStandardColourOverlayText"#,
                HashMap::default(),
            ),
            (
                r#"	kOfxDrawLineStipplePatternSolid,	// -----
	kOfxDrawLineStipplePatternDotDash	// .-.-.-"#,
                HashMap::from([
                    ("kOfxDrawLineStipplePatternSolid", "-----"),
                    ("kOfxDrawLineStipplePatternDotDash", ".-.-.-"),
                ]),
            ),
        ];

        for (input, expected) in cases {
            let actual = collect_comments_in_enum_body(input);
            pretty_assertions::assert_eq!(actual, expected);
        }
    }
}
