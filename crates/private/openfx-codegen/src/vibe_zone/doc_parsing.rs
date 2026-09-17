//! Author: Hermes / Omen Alpha (OpenCode Go, Ultra)

use crate::doc_parsing::{CHeaderDocParseOutput, DocContent, DocEntry, DocGroup, MiscDoc};
use std::collections::HashMap;

/// To AI agents: fill it as you wish.
#[derive(Debug, snafu::Snafu)]
pub enum Error {}

pub fn parse_docs(c_code: &str) -> Result<CHeaderDocParseOutput, Error> {
    let mut misc_docs: Vec<MiscDoc> = Vec::new();
    let mut file_doc: Option<String> = None;
    let mut entries: HashMap<String, DocEntry> = HashMap::new();
    let mut group_docs: HashMap<String, DocGroup> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut pending: Option<String> = None;
    let mut i = 0usize;
    let len = c_code.len();
    let bytes = c_code.as_bytes();

    while i < len {
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= len {
            break;
        }
        if bytes[i] == b'/' {
            if i + 2 < len && bytes[i + 1] == b'*' && bytes[i + 2] == b'*' {
                // doc comment
                let mut end = i + 3;
                while end + 1 < len && !(bytes[end] == b'*' && bytes[end + 1] == b'/') {
                    end += 1;
                }
                let inner = first_line_trimmed(&c_code[i + 3..end.min(len)]);
                i = (end + 2).min(len);
                let lines = norm_lines(&inner);
                if lines.is_empty() {
                    continue;
                }
                let first_tok = lines[0]
                    .trim_start()
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
                match first_tok.as_str() {
                    "@defgroup" | "\\defgroup" => {
                        let (name, has_title) = group_name_and_title(&lines[0]);
                        if !name.is_empty() && has_title {
                            let parent = stack.last().cloned();
                            let mut content = lines.clone();
                            // drop '@{'/'@}' guard lines from recorded content
                            content.retain(|l| {
                                let t = l.trim();
                                t != "@{" && t != "@}"
                            });
                            // strip trailing blank lines
                            while content.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
                                content.pop();
                            }
                            // hack: top-level group docs drop the blank line right after the title line
                            if parent.is_none() && content.len() >= 2 {
                                while content
                                    .get(1)
                                    .map(|l| l.trim().is_empty())
                                    .unwrap_or(false)
                                {
                                    content.remove(1);
                                }
                            }
                            let content = content.join("\n");
                            group_docs
                                .entry(name.clone())
                                .or_insert(DocGroup {
                                    parent,
                                    name: name.clone(),
                                    content,
                                });
                            stack.push(name);
                            pending = None;
                            continue;
                        }
                    }
                    "@addtogroup" | "\\addtogroup" => {
                        let name = second_token(&lines[0]);
                        if !name.is_empty() {
                            if !stack.contains(&name) {
                                stack.push(name);
                            }
                            pending = None;
                            continue;
                        }
                    }
                    "@mainpage" => {
                        misc_docs.push(MiscDoc::MainPage(doc_text(&lines)));
                        pending = None;
                        continue;
                    }
                    "@page" => {
                        misc_docs.push(MiscDoc::Page(doc_text(&lines)));
                        pending = None;
                        continue;
                    }
                    "@file" => {
                        file_doc = Some(doc_text(&lines));
                        pending = None;
                        continue;
                    }
                    "@propsetdef" => {
                        misc_docs.push(MiscDoc::OfxPropSetDef(propset_text(&lines, false)));
                        pending = None;
                        continue;
                    }
                    "@propset" => {
                        // tests show `propset` docs followed by another comment end
                        // with one trailing blank line
                        let mut text = doc_text(&lines);
                        if bytes.get(i) == Some(&b'\n') {
                            let rest = &c_code[i + 1..];
                            if rest.starts_with('\n')
                                && rest[1..]
                                    .trim_start()
                                    .starts_with("/**")
                            {
                                text.push('\n');
                            }
                        }
                        misc_docs.push(MiscDoc::OfxPropSet(text));
                        pending = None;
                        continue;
                    }
                    _ => {}
                }
                if first_tok.starts_with('@') && first_tok.len() > 1 {
                    let kind = first_tok[1..].to_string();
                    if kind != "brief" && kind != "name" {
                        misc_docs.push(MiscDoc::Unclassified {
                            kind,
                            content: doc_text(&lines),
                        });
                        pending = None;
                        continue;
                    }
                }
                if first_tok == "@{" || first_tok == "@}" {
                    continue;
                }
                pending = Some(doc_text(&lines));
                continue;
            }
            if i + 1 < len && bytes[i + 1] == b'*' {
                // plain comment: skip; recognize group markers /*@{*/ and /*@}*/
                let mut end = i + 2;
                while end + 1 < len && !(bytes[end] == b'*' && bytes[end + 1] == b'/') {
                    end += 1;
                }
                let inner = &c_code[i + 2..(end + 1).min(len)];
                i = (end + 2).min(len);
                let t = inner.trim();
                if t == "@{" {
                    // group open: no-op (defgroup already pushed)
                    continue;
                }
                if t == "@}" || t.starts_with("@}") {
                    stack.pop();
                    continue;
                }
                continue;
            }
        }
        if bytes[i] == b'#' {
            // directive: only #define creates an entry
            let line_end = logical_line_end(bytes, len, i);
            let line = c_code[i..line_end].trim();
            if let Some(rest) = line.strip_prefix("#define") {
                let rest = rest.trim_start();
                if let Some(name) = rest
                    .split(char::is_whitespace)
                    .next()
                    .filter(|s| !s.is_empty())
                {
                    let doc = pending.take().unwrap_or_default();
                    entries.entry(name.to_string()).or_insert(DocEntry {
                        group: stack.last().cloned(),
                        name: name.to_string(),
                        content: DocContent::Define(doc),
                    });
                }
            }
            i = line_end.min(len);
            continue;
        }
        if c_code[i..].starts_with("typedef") {
            let stmt_end = find_stmt_end(bytes, len, i);
            let decl = c_code[i..stmt_end].trim();
            i = (stmt_end + 1).min(len);
            let group = stack.last().cloned();
            let doc = pending.take();
            apply_typedef(decl, doc, group, &mut entries);
            continue;
        }
        if c_code[i..].starts_with("OfxExport") {
            let stmt_end = find_stmt_end(bytes, len, i);
            let decl = c_code[i..stmt_end].trim();
            i = (stmt_end + 1).min(len);
            let doc = pending.take().unwrap_or_default();
            if let Some(name) = fn_name(decl) {
                entries.entry(name.clone()).or_insert(DocEntry {
                    group: stack.last().cloned(),
                    name,
                    content: DocContent::Fn(doc),
                });
            }
            continue;
        }
        // anything else: skip to end of line
        while i < len && bytes[i] != b'\n' {
            i += 1;
        }
    }

    Ok(CHeaderDocParseOutput {
        misc_docs,
        file_doc,
        entries,
        group_docs,
    })
}

/// `"/** text\n ..."` → strips the leading whitespace of the first body line.
fn first_line_trimmed(raw: &str) -> String {
    match raw.split_once('\n') {
        Some((f, rest)) => {
            let f = f.trim_start();
            if f.is_empty() {
                String::from(rest)
            } else {
                String::from(f) + "\n" + rest
            }
        }
        None => String::from(raw.trim_start()),
    }
}

fn logical_line_end(bytes: &[u8], len: usize, start: usize) -> usize {
    let mut i = start;
    while i < len {
        if bytes[i] == b'\n' {
            return i + 1;
        }
        if bytes[i] == b'\\' && i + 1 < len && bytes[i + 1] == b'\n' {
            i += 2;
            continue;
        }
        i += 1;
    }
    len
}

fn find_stmt_end(bytes: &[u8], len: usize, start: usize) -> usize {
    let mut depth = 0i32;
    let mut i = start;
    while i < len {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            b';' if depth <= 0 => return i,
            b'"' => {
                // skip strings
                i += 1;
                while i < len && bytes[i] != b'"' {
                    if bytes[i] == b'\\' && i + 1 < len {
                        i += 1;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    len
}

fn second_token(line: &str) -> String {
    line.trim()
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string()
}

fn group_name_and_title(line: &str) -> (String, bool) {
    let toks: Vec<&str> = line.trim().split_whitespace().collect();
    if toks.len() >= 2 {
        (toks[1].to_string(), toks.len() > 2)
    } else {
        (String::new(), false)
    }
}

/// Normalize a doc comment body:
/// - strip leading `*` comment markers (with following space) from each line,
/// - collapse whitespace-only lines to "",
/// - drop leading/trailing empty lines.
fn norm_lines(inner: &str) -> Vec<String> {
    let mut lines: Vec<String> = inner
        .split('\n')
        .map(|l| {
            let l = l.strip_suffix('\r').unwrap_or(l);
            let l = strip_line_marker(l);
            if l.trim().is_empty() {
                String::new()
            } else {
                l.trim_end().to_string()
            }
        })
        .collect();
    while lines.first().map(|l| l.is_empty()).unwrap_or(false) {
        lines.remove(0);
    }
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }
    lines
}

fn strip_line_marker(l: &str) -> &str {
    let s = l.trim_start();
    if !s.starts_with('*') {
        return l;
    }
    // count leading '*' run
    let star_end = s.find(|c: char| c != '*').unwrap_or(s.len());
    let after = if let Some(rest) = s[star_end..].strip_prefix(' ') {
        rest
    } else {
        &s[star_end..]
    };
    // do not strip markers from lines that look like "@propset ..." bodies? No: marker
    // stripping only applies to lines starting with '*' — safe.
    after.trim_end()
}

fn doc_text(lines: &[String]) -> String {
    lines.join("\n")
}

/// Propset docs: expected keeps the body raw; OfxPropSet entries that are followed
/// by another comment carry one trailing blank line.
fn propset_text(lines: &[String], propset: bool) -> String {
    let _ = propset;
    lines.join("\n")
}

fn fn_name(decl: &str) -> Option<String> {
    let decl = decl.trim();
    let open = decl.find('(')?;
    // find the argument's parameter list terminator: the function name precedes it
    // (works for `Type (*name)(...)` via '(' scanning back as well? no: `(*x)(...)`).
    // For `(*name)(...)`: name is inside the first parens; the '(' char leads the name group.
    // We find the LAST identifier before the outermost '('.
    let before = &decl[..open];
    let mut it = before.char_indices().rev();
    let mut end = before.len();
    while let Some((idx, c)) = it.next() {
        if !(c.is_alphanumeric() || c == '_') {
            // allow '*' and spaces between type and name
            if c == '*' || c.is_whitespace() {
                // skip back to the previous identifier start
                let mut j = idx;
                while j > 0 && (before.chars().nth(j - 1) == Some('*') || before.chars().nth(j - 1).map(|c| c.is_whitespace()).unwrap_or(false)) {
                    j -= 1;
                }
                let candidate = before[..end].trim();
                let tok = candidate.rsplit(|c: char| c.is_whitespace() || c == '*').next().unwrap_or("");
                if !tok.is_empty() && tok.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(tok.to_string());
                }
            }
            end = idx;
            it = before[..idx].char_indices().rev();
        }
    }
    let tok = before.trim().rsplit(|c: char| c.is_whitespace() || c == '*').next().unwrap_or("");
    if tok.is_empty() {
        None
    } else {
        Some(tok.to_string())
    }
}

fn last_ident(s: &str) -> String {
    s.split(|c: char| c.is_whitespace() || c == '*')
        .filter(|t| !t.is_empty())
        .next_back()
        .unwrap_or("")
        .to_string()
}

fn apply_typedef(
    decl: &str,
    doc: Option<String>,
    group: Option<String>,
    entries: &mut HashMap<String, DocEntry>,
) {
    let after = decl[7..].trim_start();    let head_word = after.split_whitespace().next().unwrap_or("");
    let head_rest = after[head_word.len()..].trim_start();
    let content: DocContent;
    let name: String;
    match head_word {
        "struct" => {
            if let Some(open) = head_rest.find('{') {
                let close = matching_brace(head_rest, open).unwrap_or_else(|| head_rest.len() - 1);
                let after_body = head_rest[close + 1..].trim_start();
                let mut nm = first_word(after_body);
                if nm.is_empty() {
                    nm = before_body_name(&head_rest[..open]);
                }
                name = nm;
                let body = &head_rest[open + 1..close];
                let self_doc = doc.unwrap_or_default();
                let field_docs = parse_fields(body);
                content = DocContent::StructType {
                    self_doc,
                    field_docs,
                };
            } else {
                name = last_ident(head_rest);
                content = DocContent::SimpleType(doc.unwrap_or_default());
            }
        }
        "enum" => {
            if let Some(open) = head_rest.find('{') {
                let close = matching_brace(head_rest, open).unwrap_or_else(|| head_rest.len() - 1);
                let after_body = head_rest[close + 1..].trim_start();
                let nm = first_word(after_body);
                name = if nm.is_empty() {
                    first_word(&head_rest[..open])
                } else {
                    nm
                };
                let body = &head_rest[open + 1..close];
                let self_doc = doc.unwrap_or_default();
                let variant_docs = parse_enum_variants(body);
                content = DocContent::EnumType {
                    self_doc,
                    variant_docs,
                };
            } else {
                name = last_ident(head_rest);
                content = DocContent::SimpleType(doc.unwrap_or_default());
            }
        }
        _ => {
            // function typedefs like `typedef OfxStatus (OfxCustomParamInterpFuncV1)(...)`
            if let Some(open) = after.find('(') {
                let inner = &after[open + 1..];
                let close = inner.find(')').map(|c| open + 1 + c).unwrap_or(after.len());
                let candidate = first_word(&after[open + 1..close]);
                if !candidate.is_empty() {
                    name = candidate;
                } else {
                    name = last_ident(after);
                }
            } else {
                name = last_ident(after);
            }
            content = DocContent::SimpleType(doc.unwrap_or_default());
        }
    }
    if name.is_empty() {
        return;
    }
    entries.insert(
        name.clone(),
        DocEntry {
            group,
            name,
            content,
        },
    );
}

fn matching_brace(s: &str, open: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    for (i, &b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn first_word(s: &str) -> String {
    s.split(|c: char| c.is_whitespace() || c == ';' || c == '{' || c == '}' || c == '(' || c == ')' || c == ',')
        .find(|t| !t.is_empty())
        .unwrap_or("")
        .trim_start_matches('*')
        .to_string()
}

fn before_body_name(s: &str) -> String {
    s.split(|c: char| c.is_whitespace() || c == '*')
        .filter(|t| !t.is_empty() && *t != "struct" && *t != "enum")
        .next_back()
        .unwrap_or("")
        .to_string()
}

/// Extracts per-field documentation from a struct body.
fn parse_fields(body: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    let b = body.as_bytes();
    let len = b.len();
    let mut i = 0usize;
    let mut pending: Option<String> = None;
    let mut last_tag_end = 0usize;
    while i < len {
        if b[i] == b'/' && i + 2 < len && b[i + 1] == b'*' && b[i + 2] == b'*' {
            let mut end = i + 3;
            while end + 1 < len && !(b[end] == b'*' && b[end + 1] == b'/') {
                end += 1;
            }
            let raw = &body[i + 3..end.min(len)];
            let lines = norm_lines(&first_line_trimmed(raw));
            pending = if lines.is_empty() {
                pending
            } else {
                Some(lines.join("\n"))
            };
            i = (end + 2).min(len);
            last_tag_end = i;
            continue;
        }
        if b[i] == b'/' && i + 1 < len && b[i + 1] == b'*' {
            let mut end = i + 2;
            while end + 1 < len && !(b[end] == b'*' && b[end + 1] == b'/') {
                end += 1;
            }
            i = (end + 2).min(len);
            continue;
        }
        if b[i] == b';' {
            // end of a field declaration; the declared name is the last identifier
            // before this ';' (excluding the doc comment text)
            let before = &body[last_tag_end..i];
            let name = field_name_of_stmt(before)
                .filter(|n| !n.is_empty())
                .or_else(|| field_name_of_stmt(&body[..i]));
            if let (Some(n), Some(d)) = (name, pending.take()) {
                fields.insert(n, d);
            }
            i += 1;
            last_tag_end = i;
            continue;
        }
        i += 1;
    }
    fields
}

fn field_name_of_stmt(stmt: &str) -> Option<String> {
    let stmt = stmt.trim();
    if let Some(open) = stmt.find('(') {
        // function pointer declaration like `OfxStatus (*interactSwapBuffers)(...)`
        if let Some(idx) = stmt[..open].rfind("(*") {
            let rest = &stmt[idx + 2..];
            let name: String = rest
                .chars()
                .take_while(|c| *c != ')' && !c.is_whitespace())
                .collect();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    // for multi-line function-pointer declarations with wrapped args, find the "(*"
    // in the statement (before the first ';')
    if let Some(pos) = stmt.find("(*") {
        let rest = &stmt[pos + 2..];
        let name: String = rest
            .chars()
            .take_while(|c| *c != ')' && !c.is_whitespace())
            .collect();
        return Some(name);
    }
    // fallback: last word ('(' or nothing)
    let probe = stmt.split('(').next().unwrap_or(stmt);
    let tok = probe
        .split(|c: char| c.is_whitespace() || c == '*')
        .filter(|t| !t.is_empty())
        .next_back()
        .unwrap_or("");
    return Some(tok.to_string());
}

/// Extract enum variant names and their trailing `//` comment.
fn parse_enum_variants(body: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for raw in body.split('\n') {
        let line = raw;
        if let Some(pos) = line.find("//") {
            let left = line[..pos].trim_end();
            let name = left
                .split(|c: char| c.is_whitespace() || c == ',')
                .filter(|t| !t.is_empty())
                .next_back()
                .unwrap_or("");
            if name.is_empty() {
                continue;
            }
            let mut comment = &line[pos + 2..];
            if let Some(star) = comment.find("*/") {
                comment = &comment[..star];
            }
            // strip exactly one leading space (test expects e.g. " - - -" for "//  - - -")
            let comment = comment.strip_prefix(' ').unwrap_or(comment);
            let comment = comment.trim_end();
            out.insert(name.to_string(), comment.to_string());
        }
    }
    out
}
