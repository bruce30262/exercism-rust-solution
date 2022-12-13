use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    words.split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches('\'').to_lowercase())
        .fold(HashMap::new(), |mut acc, w| {
            acc.entry(w).and_modify(|cnt| *cnt += 1).or_insert(1);
            acc
        })
}
