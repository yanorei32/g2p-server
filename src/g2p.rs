use std::sync::Arc;

use lingua::{Language, LanguageDetector};
use vibrato::Tokenizer;

/// Perform G2P conversion
pub fn convert_text(
    text: &str,
    tokenizer: &Arc<Tokenizer>,
    detector: &Arc<LanguageDetector>,
) -> String {
    let mut result = String::new();
    let blocks = split_by_script(text);

    for block in blocks {
        let detected = detector.detect_language_of(&block);

        let is_japanese = if let Some(lang) = detected {
            lang == Language::Japanese
        } else {
            block.chars().next().map(is_japanese_char).unwrap_or(false)
        };

        if is_japanese {
            let mut worker = tokenizer.new_worker();
            worker.reset_sentence(&block);
            worker.tokenize();

            let num_tokens = worker.num_tokens();
            for i in 0..num_tokens {
                let token = worker.token(i);
                let features = token.feature();
                let parts: Vec<&str> = features.split(',').collect();

                let reading = if parts.len() > 9 {
                    parts[9] // Pronunciation (UniDic: pron field)
                } else if parts.len() > 6 {
                    parts[6] // Reading (UniDic: lForm field)
                } else {
                    token.surface()
                };

                let hiragana = crate::utils::katakana_to_hiragana(reading);
                result.push_str(&hiragana);
            }
        } else {
            result.push_str(&block);
        }
    }

    result
}

/// Check if character is Japanese
fn is_japanese_char(c: char) -> bool {
    matches!(c,
        '\u{3040}'..='\u{309F}' | // Hiragana
        '\u{30A0}'..='\u{30FF}' | // Katakana
        '\u{4E00}'..='\u{9FFF}' | // Kanji
        '\u{3000}'..='\u{303F}' | // CJK Symbols and Punctuation
        '\u{FF00}'..='\u{FFEF}'   // Halfwidth and Fullwidth Forms
    )
}

/// Split text into blocks by script
fn split_by_script(text: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current_block = String::new();
    let mut current_is_jp = false;

    for (i, c) in text.chars().enumerate() {
        let is_jp = is_japanese_char(c);

        if i == 0 {
            current_block.push(c);
            current_is_jp = is_jp;
        } else if is_jp == current_is_jp {
            current_block.push(c);
        } else {
            blocks.push(current_block);
            current_block = String::new();
            current_block.push(c);
            current_is_jp = is_jp;
        }
    }
    if !current_block.is_empty() {
        blocks.push(current_block);
    }
    blocks
}
