// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    
    for word in magazine {
        map.entry(word)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    for word in note {
        if !map.contains_key(word) || map.get(word) == Some(&0) {
            return false;
        }
        map.entry(word).and_modify(|e| { *e -= 1 });
    }
    true
}
