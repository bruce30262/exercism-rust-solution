use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    words.split(|c: char| !c.is_alphanumeric() && c != '\'') // split with chars that matches the condition
        .filter(|w| !w.is_empty()) // strip empty string. filter for iter, retain for vec
        .map(|w| w.trim_matches('\'').to_lowercase()) // map for iter. Do trim & to_lowercase
        .fold(HashMap::new(), |mut acc, w| { // fold for iter. Fold with HashMap::new() ( acc )
            acc.entry(w).and_modify(|cnt| *cnt += 1).or_insert(1);
            acc
        })
}
