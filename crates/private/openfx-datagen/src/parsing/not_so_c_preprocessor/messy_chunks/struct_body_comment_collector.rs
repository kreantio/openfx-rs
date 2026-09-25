use std::collections::HashMap;

use lang_c::ast::{DeclaratorKind, ExternalDeclaration};

use crate::parsing::{
    c_parser::parse_c_code_tolerating_unknown_types,
    not_so_c_preprocessor::messy_chunks::utils::{
        LinesEx, rfind_possible_symbol,
        until_multi_line_comment_close_exclusive_but_consuming_last_line_and_wanting_first_line,
    },
    welp::welp,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct StructBodyComments<'a> {
    pub field_comments: HashMap<&'a str, &'a str>,
    pub fn_field_parameter_comments: HashMap<(&'a str, &'a str), &'a str>,
}

pub fn collect_comments_in_struct_body<'a>(body_code: &'a str) -> StructBodyComments<'a> {
    let mut lines = LinesEx::new(body_code).peekable();

    let mut field_comments: HashMap<&'a str, &'a str> = HashMap::new();
    let mut fn_field_parameter_comments: HashMap<(&'a str, &'a str), &'a str> = HashMap::new();

    let mut last_comment: Option<&'a str> = None;

    while let Some(mut line) = lines.next() {
        if line.content.trim().is_empty() {
            continue;
        }
        if line.content.trim_start().starts_with("/*") {
            let comment_content_range =
                until_multi_line_comment_close_exclusive_but_consuming_last_line_and_wanting_first_line(
                    body_code, &mut lines, line,
                );
            if let Some(comment) = last_comment {
                // TODO: Those overwritten comments should be related to
                // grouping, and we should support extracting them as well
                // later.
                tracing::warn!("Overwriting previous comment: {}", comment);
            }
            last_comment = Some(&body_code[comment_content_range]);
        } else {
            let mut parameter_comments: HashMap<&str, &str> = HashMap::new();

            let mut content_range = line.start_offset..(line.start_offset + line.content.len());
            let mut has_single_line_comments = false;
            loop {
                if line.content.contains("/*") {
                    welp!();
                }
                if let Some(comment_start) = line.content.find("//") {
                    let Some(sym) = rfind_possible_symbol(&line.content[..comment_start]) else {
                        welp!();
                    };
                    parameter_comments
                        .insert(sym, line.content[comment_start + "//".len()..].trim());
                    has_single_line_comments = true;
                }

                content_range.end = line.start_offset + line.content.len();

                if line.content.contains(";") {
                    break;
                }

                if lines
                    .peek()
                    .is_some_and(|l| !l.content.trim_start().starts_with("/*"))
                {
                    line = lines.next().unwrap();
                } else {
                    break;
                }
            }

            let idents: Vec<&str> = if has_single_line_comments {
                let code = &body_code[content_range];
                let code_with_single_line_comments_stripped = code
                    .lines()
                    .map(|l| {
                        if let Some(comment_start) = l.find("//") {
                            &l[..comment_start]
                        } else {
                            l
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("\n");
                let result = extract_identifiers(&code_with_single_line_comments_stripped);
                result
                    .into_iter()
                    .map(|ident| {
                        let ident_start_in_code = code.find(ident).unwrap();
                        &code[ident_start_in_code..ident_start_in_code + ident.len()]
                    })
                    .collect()
            } else {
                extract_identifiers(&body_code[content_range])
            };

            if let Some(comment) = last_comment.take() {
                for ident in &idents {
                    field_comments.insert(ident, comment);
                }
            }

            for ident in idents {
                for (&param, comment) in &parameter_comments {
                    if ident == param {
                        welp!();
                    }
                    fn_field_parameter_comments.insert((ident, param), comment);
                }
            }
        }
    }

    if let Some(comment) = last_comment {
        tracing::warn!("Unassociated comment found: {}", comment);
    }

    StructBodyComments {
        field_comments,
        fn_field_parameter_comments,
    }
}

/// Author: GitHub Copilot / Kimi K3 (High)
/// Reviewed-by: Umaĵo
fn extract_identifiers(stmt: &str) -> Vec<&str> {
    let parsed = parse_c_code_tolerating_unknown_types(stmt).expect("The parse should succeed.");

    // The spans in `parsed.unit` refer to `stmt` prefixed with dummy typedefs;
    // subtract `parsed.prefix_len` to map them back onto `stmt`.
    let mut identifiers = Vec::new();
    assert!(parsed.unit.0.len() <= 1);
    for external_declaration in parsed.unit.0 {
        let ExternalDeclaration::Declaration(declaration) = external_declaration.node else {
            welp!();
        };
        for init_declarator in declaration.node.declarators {
            let mut kind = &init_declarator.node.declarator.node.kind;
            loop {
                match &kind.node {
                    // E.g. `host` in `OfxPropertySetHandle host;`.
                    DeclaratorKind::Identifier(identifier) => {
                        identifiers.push(
                            &stmt[identifier.span.start - parsed.prefix_len
                                ..identifier.span.end - parsed.prefix_len],
                        );
                        break;
                    }
                    // E.g. the parenthesized `*fetchSuite` in `void *(*fetchSuite)(...);`.
                    DeclaratorKind::Declarator(inner) => kind = &inner.node.kind,
                    DeclaratorKind::Abstract => break,
                }
            }
        }
    }
    identifiers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_comments_in_struct_body() {
        let cases = &[
            (
                r#"  /** @brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded.
   */
  OfxPropertySetHandle host;

  /** @brief The function which the plug-in uses to fetch suites from the host.
…
  */
  const void *(*fetchSuite)(OfxPropertySetHandle host, const char *suiteName, int suiteVersion);"#,
                StructBodyComments {
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
            ),
            (
                r#"  /** Defines the type of the plug-in, this will tell the host what the plug-in does. e.g.: an image
      effects plug-in would be a "OfxImageEffectPlugin"
   */
  const char		*pluginApi;

  /** Defines the version of the pluginApi that this plug-in implements */
  int            apiVersion;"#,
                StructBodyComments {
                    field_comments: HashMap::from([
                        (
                            "pluginApi",
                            r#"Defines the type of the plug-in, this will tell the host what the plug-in does. e.g.: an image
      effects plug-in would be a "OfxImageEffectPlugin""#,
                        ),
                        (
                            "apiVersion",
                            r#"Defines the version of the pluginApi that this plug-in implements"#,
                        ),
                    ]),
                    fn_field_parameter_comments: Default::default(),
                },
            ),
            (r#"int min, max;"#, Default::default()),
            (
                r#"/** @name Keyframe Handling
…
 */
/** @{ */

  /** @brief Returns the number of keyframes in the parameter
…
  */
  OfxStatus (*paramGetNumKeys)(OfxParamHandle  paramHandle,
			       unsigned int  *numberOfKeys);"#,
                StructBodyComments {
                    field_comments: HashMap::from([(
                        "paramGetNumKeys",
                        r#"@brief Returns the number of keyframes in the parameter
…"#,
                    )]),
                    fn_field_parameter_comments: Default::default(),
                },
            ),
            (
                r#"    /** @brief Compiles the OpenCL program */
    OfxStatus(*compileProgram)(const char   *pszProgramSource,
        int           fOptional,          // if non-zero, host may skip compiling on this call
        void         *pResult);           // cast to cl_program*"#,
                StructBodyComments {
                    field_comments: HashMap::from([(
                        "compileProgram",
                        r#"@brief Compiles the OpenCL program"#,
                    )]),
                    fn_field_parameter_comments: HashMap::from([
                        (
                            ("compileProgram", "fOptional"),
                            r#"if non-zero, host may skip compiling on this call"#,
                        ),
                        (("compileProgram", "pResult"), r#"cast to cl_program*"#),
                    ]),
                },
            ),
        ];

        for (input, expected) in cases {
            pretty_assertions::assert_eq!(&collect_comments_in_struct_body(input), expected);
        }
    }

    #[test]
    fn test_extract_identifiers() {
        let cases: &[(&str, &[&str])] = &[
            (r#"OfxPropertySetHandle host;"#, &["host"]),
            (
                r#"const void *(*fetchSuite)(OfxPropertySetHandle host, const char *suiteName, int suiteVersion);"#,
                &["fetchSuite"],
            ),
            (r#"const char		*pluginApi;"#, &["pluginApi"]),
            (r#"int            apiVersion;"#, &["apiVersion"]),
            (r#"void     (*setHost)(OfxHost *host);"#, &["setHost"]),
            (r#"int min, max;"#, &["min", "max"]),
        ];

        for (input, expected) in cases {
            pretty_assertions::assert_eq!(extract_identifiers(input), Vec::from(*expected));
        }
    }
}
