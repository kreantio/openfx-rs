/// Author: GitHub Copilot's tab completion | Reviewed-by: Umaĵo
///
/// Removes the longest prefix shared by every string.
pub fn strip_common_prefix(strings: &[String]) -> Vec<String> {
    if strings.is_empty() {
        return Vec::new();
    }

    let mut character_iterators: Vec<_> = strings.iter().map(|string| string.chars()).collect();
    let mut prefix_length = 0;

    while let Some(character) = character_iterators[0].next() {
        if character_iterators[1..]
            .iter_mut()
            .all(|iterator| iterator.next() == Some(character))
        {
            prefix_length += character.len_utf8();
        } else {
            break;
        }
    }

    strings
        .iter()
        .map(|string| string[prefix_length..].to_string())
        .collect()
}

pub fn new_ident(string: impl AsRef<str>) -> syn::Ident {
    syn::Ident::new(string.as_ref(), proc_macro2::Span::call_site())
}

/// Unlike [`::convert_case`]:
/// - strings such as `V1` are preserved as `v1` rather than `v_1`.
/// - strings such as `3D` are preserved as `3d` rather than `3_d`.
pub fn to_snake_case(name: &str) -> String {
    let mut snake_case = String::new();

    let mut last: Option<char> = None;
    let mut iter = name.chars().peekable();
    while let Some(c) = iter.next() {
        if let Some(last) = last {
            let is_word_start = match () {
                _ if c.is_uppercase() && last.is_numeric() => false,
                _ if c.is_uppercase()
                    && (last.is_lowercase() || !iter.peek().is_some_and(|c| c.is_uppercase())) =>
                {
                    true
                }
                _ if c.is_numeric() && last.is_lowercase() => true,
                _ => false,
            };
            if is_word_start {
                snake_case.push('_');
            }
        }

        snake_case.push(c.to_ascii_lowercase());
        last = Some(c);
    }

    snake_case
}

#[cfg(test)]
#[test]
fn test_to_snake_case() {
    assert_eq!(to_snake_case("Hello"), "hello");
    assert_eq!(to_snake_case("hello"), "hello");
    assert_eq!(to_snake_case("helloWorld"), "hello_world");
    assert_eq!(to_snake_case("hello_world"), "hello_world");

    assert_eq!(to_snake_case("ParamInteractV1"), "param_interact_v1");
    assert_eq!(to_snake_case("Double2D"), "double_2d");
    assert_eq!(
        to_snake_case("ImageEffectOpenGLRenderSuiteV1"),
        "image_effect_open_gl_render_suite_v1"
    );
}
