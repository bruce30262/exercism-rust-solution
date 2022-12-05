use std::collections::HashSet;

// https://stackoverflow.com/questions/47640550/what-is-a-in-rust-language
// https://doc.rust-lang.org/rust-by-example/scope/lifetime/explicit.html
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word_vec = word.to_lowercase().chars().collect::<Vec<_>>();
    word_vec.sort_unstable();
    
    let mut ans = HashSet::new();    
    for s in possible_anagrams {
        if word.to_lowercase() == s.to_lowercase() { continue; }
        if word.len() != s.len() { continue; }
        
        let mut s_vec = s.to_lowercase().chars().collect::<Vec<_>>();
        s_vec.sort_unstable();
        
        if s_vec == word_vec {
            ans.insert(s.to_owned());
        }
    }
    ans
}
