// Define kana mapping table
static KANA_TABLE: &[&str] = &[
    "あ",
    "い",
    "う",
    "え",
    "お",
    "や",
    "ゆ",
    "いぇ",
    "よ",
    "わ",
    "うぃ",
    "うぇ",
    "うぉ",
    "か",
    "き",
    "く",
    "け",
    "こ",
    "きゃ",
    "きゅ",
    "きぇ",
    "きょ",
    "くぁ",
    "くぃ",
    "くぇ",
    "くぉ",
    "が",
    "ぎ",
    "ぐ",
    "げ",
    "ご",
    "ぎゃ",
    "ぎゅ",
    "ぎぇ",
    "ぎょ",
    "ぐぁ",
    "ぐぃ",
    "ぐぇ",
    "ぐぉ",
    "さ",
    "し",
    "す",
    "せ",
    "そ",
    "しゃ",
    "しゅ",
    "しぇ",
    "しょ",
    "すぁ",
    "すぃ",
    "すぇ",
    "すぉ",
    "ざ",
    "じ",
    "ず",
    "ぜ",
    "ぞ",
    "じゃ",
    "じゅ",
    "じぇ",
    "じょ",
    "ずぁ",
    "ずぃ",
    "ずぇ",
    "ずぉ",
    "た",
    "てぃ",
    "とぅ",
    "て",
    "と",
    "てゃ",
    "てゅ",
    "てぇ",
    "てょ",
    "とぁ",
    "とぃ",
    "とぇ",
    "とぉ",
    "だ",
    "でぃ",
    "とぅ",
    "で",
    "ど",
    "でゃ",
    "でゅ",
    "でぇ",
    "でょ",
    "どぁ",
    "どぃ",
    "どぇ",
    "どぉ",
    "な",
    "に",
    "ぬ",
    "ね",
    "の",
    "にゃ",
    "にゅ",
    "にぇ",
    "にょ",
    "ぬぁ",
    "ぬぃ",
    "ぬぇ",
    "ぬぉ",
    "は",
    "ひ",
    "ふ",
    "へ",
    "ほ",
    "ひゃ",
    "ひゅ",
    "ひぇ",
    "ひょ",
    "ふぁ",
    "ふぃ",
    "ふぇ",
    "ふぉ",
    "ば",
    "び",
    "ぶ",
    "べ",
    "ぼ",
    "びゃ",
    "びゅ",
    "びぇ",
    "びょ",
    "ぶぁ",
    "ぶぃ",
    "ぶぇ",
    "ぶぉ",
    "ぱ",
    "ぴ",
    "ぷ",
    "ぺ",
    "ぽ",
    "ぴゃ",
    "ぴゅ",
    "ぴぇ",
    "ぴょ",
    "ぷぁ",
    "ぷぃ",
    "ぷぇ",
    "ぷぉ",
    "ま",
    "み",
    "む",
    "め",
    "も",
    "みゃ",
    "みゅ",
    "みぇ",
    "みょ",
    "むぁ",
    "むぃ",
    "むぇ",
    "むぉ",
    "ら",
    "り",
    "る",
    "れ",
    "ろ",
    "りゃ",
    "りゅ",
    "りぇ",
    "りょ",
    "るぁ",
    "るぃ",
    "るぇ",
    "るぉ",
    "",
    "ち",
    "",
    "",
    "",
    "ちゃ",
    "ちゅ",
    "ちぇ",
    "ちょ",
    "ちゅぁ",
    "ちゅぃ",
    "ちゅぇ",
    "ちゅぉ",
    "",
    "ぢ",
    "",
    "",
    "",
    "ぢゃ",
    "ぢゅ",
    "ぢぇ",
    "ぢょ",
    "ぢゅぁ",
    "ぢゅぃ",
    "ぢゅぇ",
    "ぢゅぉ",
    "つぁ",
    "つぃ",
    "つ",
    "つぇ",
    "つぉ",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "ゔ",
    "",
    "",
    "",
    "",
    "",
    "",
    "ゔぁ",
    "ゔぃ",
    "ゔぇ",
    "ゔぉ",
    "ぁ",
    "ぃ",
    "ぅ",
    "ぇ",
    "ぉ",
    "ゃ",
    "ゅ",
    "",
    "ょ",
    "ゎ",
    "",
    "",
    "",
];

// Use an enum to represent the intermediate state.
#[derive(Debug, Clone, PartialEq)]
enum Item {
    Syllable(i32, i32, Vec<i32>),
    Char(String),
    Separator,
    Choonpu,
    SmallTsu,
    N,
}

/// Converts Japanese Kana (Hiragana/Katakana) to Korean Syllables.
///
/// This function handles:
/// - Hiragana and Katakana (Full-width and Half-width)
/// - Choonpu ('ー') -> Vowel extension
/// - Small Tsu ('っ') -> Patchim (Final consonant)
/// - N ('ん') -> Patchim or '응'
///
/// # Examples
///
/// ```
/// use g2p_server::korean::convert_to_korean;
///
/// assert_eq!(convert_to_korean("こんにちわ"), "꼰니찌와");
/// assert_eq!(convert_to_korean("ありがとう"), "아리가또우");
/// assert_eq!(convert_to_korean("あっん"), "아응");
/// ```
pub fn convert_to_korean(text: &str) -> String {
    // Normalize to Hiragana (handling Hankaku via NFKC)
    use unicode_normalization::UnicodeNormalization;
    let normalized_text: String = text.nfkc().collect();

    let text_chars: Vec<char> = normalized_text
        .chars()
        .map(|c| {
            if ('\u{30A1}'..='\u{30F6}').contains(&c) {
                std::char::from_u32(c as u32 - 0x60).unwrap_or(c)
            } else {
                c
            }
        })
        .collect();

    let mut res: Vec<Item> = Vec::new();
    let mut pos = 0;

    while pos < text_chars.len() {
        let mut matched_i = -1;
        let mut matched_len = 0;

        // Create a slice from current position
        let current_slice = &text_chars[pos..];
        let current_text: String = current_slice.iter().collect();

        for (i, s) in KANA_TABLE.iter().enumerate() {
            if !s.is_empty() && s.chars().count() > matched_len && current_text.starts_with(s) {
                matched_i = i as i32;
                matched_len = s.chars().count();
            }
        }

        if matched_i != -1 {
            let cons = matched_i / 13;
            let vowel = matched_i % 13;
            res.push(Item::Syllable(cons, vowel, vec![]));
            pos += matched_len;
        } else {
            let c = text_chars[pos].to_string();
            let item = match c.as_str() {
                "ー" => Item::Choonpu,
                "っ" => Item::SmallTsu,
                "ん" => Item::N,
                _ => Item::Char(c),
            };
            res.push(item);
            pos += 1;
        }
    }

    // Process 'ー' (Choonpu)
    let mut i = 0;
    while i < res.len() {
        if let Item::Choonpu = res[i] {
            if i > 0 {
                if let Item::Syllable(_, prev_v, _) = res[i - 1] {
                    const VOWEL_MAP: [i32; 13] = [0, 1, 2, 3, 4, 0, 2, 3, 4, 0, 1, 3, 4];
                    let new_v = VOWEL_MAP[prev_v as usize];
                    // Replace Choonpu with a new syllable (Consonant 0 = 'あ' row -> 'ㅇ', Vowel = new_v)
                    res[i] = Item::Syllable(0, new_v, vec![]);
                    i += 1;
                    continue;
                }
            }
            res.remove(i);
            continue;
        }
        i += 1;
    }

    // Process 'っ' (Little Tsu)
    let mut i = 0;
    while i < res.len() {
        if let Item::SmallTsu = res[i] {
            if i > 0 {
                if i + 1 < res.len() {
                    match res[i + 1] {
                        Item::N => {
                            res[i] = Item::Separator;
                            i += 1;
                            continue;
                        }
                        Item::Syllable(nc, _, _) => {
                            if let Item::Syllable(prev_c, prev_v, ref mut prev_f) = res[i - 1] {
                                if prev_f.is_empty() {
                                    let to_add = if [1, 2].contains(&nc) {
                                        1
                                    } else if [9, 10, 11].contains(&nc) {
                                        17
                                    } else if nc == 12 {
                                        8
                                    } else if nc == 0 {
                                        0
                                    } else {
                                        19
                                    };
                                    prev_f.push(to_add);
                                    res[i - 1] = Item::Syllable(prev_c, prev_v, prev_f.clone());
                                }
                            }
                        }
                        _ => {
                            if let Item::Syllable(prev_c, prev_v, ref mut prev_f) = res[i - 1] {
                                if prev_f.is_empty() {
                                    prev_f.push(19);
                                    res[i - 1] = Item::Syllable(prev_c, prev_v, prev_f.clone());
                                }
                            }
                        }
                    }
                } else if let Item::Syllable(prev_c, prev_v, ref mut prev_f) = res[i - 1] {
                    if prev_f.is_empty() {
                        prev_f.push(19);
                        res[i - 1] = Item::Syllable(prev_c, prev_v, prev_f.clone());
                    } else {
                        // Already has patchim, treat 'っ' as separator/ignore or maybe standalone?
                        // For now, let's just remove it to avoid breaking the previous char
                    }
                }
            }
            if let Item::Separator = res[i] {
                // Do not remove if it became separator
            } else {
                res.remove(i);
                continue;
            }
        }
        i += 1;
    }

    // Process 'ん' (N)
    let mut i = 0;
    while i < res.len() {
        if let Item::N = res[i] {
            if i > 0 {
                // Check if we can attach to previous
                let can_attach = if let Item::Syllable(_, _, ref f) = res[i - 1] {
                    f.is_empty()
                } else {
                    false
                };

                if can_attach {
                    if i + 1 < res.len() {
                        let next_cons = if let Item::Syllable(c, _, _) = res[i + 1] {
                            Some(c)
                        } else {
                            None
                        };

                        if let Some(nc) = next_cons {
                            if let Item::Syllable(prev_c, prev_v, ref mut prev_f) = res[i - 1] {
                                let to_add = if [9, 10, 11].contains(&nc) {
                                    16
                                } else if [0, 1, 2, 8].contains(&nc) {
                                    21
                                } else {
                                    4
                                };
                                prev_f.push(to_add);
                                res[i - 1] = Item::Syllable(prev_c, prev_v, prev_f.clone());
                            }
                        } else if let Item::Syllable(prev_c, prev_v, ref mut prev_f) = res[i - 1] {
                            prev_f.push(21);
                            res[i - 1] = Item::Syllable(prev_c, prev_v, prev_f.clone());
                        }
                        res.remove(i);
                        continue;
                    } else {
                        if let Item::Syllable(prev_c, prev_v, ref mut prev_f) = res[i - 1] {
                            prev_f.push(21);
                            res[i - 1] = Item::Syllable(prev_c, prev_v, prev_f.clone());
                        }
                        res.remove(i);
                        continue;
                    }
                } else {
                    // Cannot attach (e.g. prev is Separator or Char), treat as standalone
                    res[i] = Item::Char("응".to_string());
                }
            } else {
                // Treat as standalone
                res[i] = Item::Char("응".to_string());
            }
        }
        i += 1;
    }

    // Final Composition
    let mut final_string = String::new();
    for item in res {
        match item {
            Item::Separator | Item::Choonpu | Item::SmallTsu | Item::N => {} // Ignore
            Item::Char(s) => final_string.push_str(&s),
            Item::Syllable(c, v, mut f) => {
                if f.is_empty() {
                    f.push(0);
                }
                if f.len() == 1 {
                    const CONST_MAP: [i32; 18] =
                        [11, 1, 0, 9, 12, 4, 3, 2, 18, 7, 8, 6, 5, 13, 14, 13, 7, 11];
                    const VOWEL_MAP: [i32; 13] = [0, 20, 13, 5, 8, 2, 17, 7, 12, 9, 16, 15, 14];

                    if c < 0 || c >= CONST_MAP.len() as i32 || v < 0 || v >= VOWEL_MAP.len() as i32
                    {
                        continue;
                    }

                    let const_num = CONST_MAP[c as usize];
                    let mut vowel_num = VOWEL_MAP[v as usize];
                    let final_num = f[0];

                    if [3, 4, 15].contains(&c) && vowel_num == 13 {
                        vowel_num = 18;
                    }

                    let code_point = 0xAC00 + (const_num * 21 + vowel_num) * 28 + final_num;
                    if let Some(ch) = std::char::from_u32(code_point as u32) {
                        final_string.push(ch);
                    }
                }
            }
        }
    }
    final_string
}
