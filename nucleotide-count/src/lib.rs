use std::collections::HashMap;

pub fn new_map() -> HashMap<char, usize> {
    let mut map:HashMap<char, usize> = HashMap::new();
    for c in "ACGT".chars() {
        map.entry(c).or_insert(0);
    }
    map
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide_counts(dna) {
        Err(err) => Err(err),
        Ok(map) => {
            match map.get(&nucleotide) {
                None => Err(nucleotide),
                Some(cnt) => Ok(*cnt),
            }
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = new_map();
    for c in dna.chars() {
        match map.get(&c) {
            None => return Err(c),
            Some(_) => {
                map.entry(c).and_modify(|n| *n += 1);
            }
        }
    }
    Ok(map)
}
