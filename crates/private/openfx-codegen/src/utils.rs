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
