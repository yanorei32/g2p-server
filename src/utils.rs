/// Converts Katakana to Hiragana
pub fn katakana_to_hiragana(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c >= '\u{30A1}' && c <= '\u{30F6}' {
                std::char::from_u32(c as u32 - 0x60).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}
