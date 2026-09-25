use lang_c::ast::TranslationUnit;
use lang_c::driver::{Config, SyntaxError, parse_preprocessed};

/// Author: GitHub Copilot / Kimi K3 (High)
/// Reviewed-by: Umaĵo
pub struct ParseResultToleratingUnknownTypes {
    /// The translation unit of `source`, without the dummy typedefs.
    pub unit: TranslationUnit,
    /// Byte length of the dummy-typedef prefix that was prepended to `source`
    /// during parsing.
    ///
    /// FOOTGUN: all spans in `unit` refer to `source` *prefixed with the
    /// dummy typedefs*, so a span `s` maps back onto `source` as
    /// `&source[s.start - prefix_len..s.end - prefix_len]`. (Only valid for
    /// spans that fall outside the prefix; all spans in `unit` do.)
    pub prefix_len: usize,
}

/// Author: GitHub Copilot / Kimi K3 (High)
/// Reviewed-by: Umaĵo
///
/// Parses `source` as a C translation unit, tolerating references to type
/// names that are not declared anywhere (e.g. a chunk extracted from a header
/// without the typedefs and `#include`s that precede it).
///
/// Every time the parser chokes on an unknown type name, a dummy
/// `typedef int Name;` is prepended and parsing is retried. The dummy
/// declarations are removed from the returned unit, so it only contains what
/// `source` itself declares.
///
/// NOTE(umajho): This is hacky and not so efficient, but on the other hand I
/// don't want to maintain a fork of a C parser.
///
/// TODO: fork lang_c?
pub fn parse_c_code_tolerating_unknown_types(
    source: &str,
) -> Result<ParseResultToleratingUnknownTypes, SyntaxError> {
    let config = Config::with_clang();

    let mut prefix = String::new();
    let mut typedefed: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut unit = loop {
        let prefixed_source = format!("{prefix}{source}");
        match parse_preprocessed(&config, prefixed_source.clone()) {
            Ok(parse) => break parse.unit,
            Err(err) => {
                let Some(unknown) = unknown_type_name_at(&prefixed_source, &err) else {
                    return Err(err);
                };
                if !typedefed.insert(unknown.to_owned()) {
                    // Typedef-ing this name did not make the parse succeed.
                    return Err(err);
                }
                prefix.push_str(&format!("typedef int {unknown};\n"));
            }
        }
    };

    // Remove the dummy typedefs: they are exactly the declarations located
    // within the prefix.
    let prefix_len = prefix.len();
    unit.0.retain(|node| node.span.start >= prefix_len);

    Ok(ParseResultToleratingUnknownTypes { unit, prefix_len })
}

/// Author: GitHub Copilot / Kimi K3 (High)
/// Reviewed-by: Umaĵo
///
/// Infers the name of the unknown type that made the parser choke at
/// `err.offset`, without any external knowledge about the types involved.
fn unknown_type_name_at<'a>(source: &'a str, err: &SyntaxError) -> Option<&'a str> {
    fn is_ident_start(b: u8) -> bool {
        b == b'_' || b.is_ascii_alphabetic()
    }
    fn is_ident_char(b: u8) -> bool {
        b == b'_' || b.is_ascii_alphanumeric()
    }

    let bytes = source.as_bytes();

    // Case 1: the parser wanted a type name and found an unknown identifier,
    // e.g. `OfxPropertySetHandle host;` failing at `OfxPropertySetHandle`.
    if err.expected.contains("<typedef_name>") {
        let start = err.offset;
        if start < bytes.len() && is_ident_start(bytes[start]) {
            let end = bytes[start..]
                .iter()
                .position(|&b| !is_ident_char(b))
                .map_or(bytes.len(), |len| start + len);
            return Some(&source[start..end]);
        }
    }

    // Case 2: the parser accepted an unknown type name as e.g. a parameter
    // name and only choked on the token after it, e.g. failing at `host` in
    // `void (*setHost)(OfxHost *host);`. The type name is the identifier
    // immediately preceding the error offset.
    let mut end = err.offset;
    while end > 0 && !is_ident_char(bytes[end - 1]) {
        end -= 1;
    }
    let mut start = end;
    while start > 0 && is_ident_char(bytes[start - 1]) {
        start -= 1;
    }
    (start < end && is_ident_start(bytes[start])).then(|| &source[start..end])
}
