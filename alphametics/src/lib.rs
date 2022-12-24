use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools; // https://docs.rs/itertools/0.10.5/itertools/trait.Itertools.html

pub fn to_number(map: &HashMap<char, u8>, s: &str) -> usize {
    let mut ret: usize = 0;
    for c in s.chars() {
        ret = ret * 10 + (*map.get(&c).unwrap() as usize);
    }
    ret
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let lhs = input.split("==").next().unwrap();
    if !lhs.contains('+') { return None }; // A == B -> None
    // get leading chars and operands
    let mut leading_char_set: HashSet<char> = HashSet::new();
    let mut operands: Vec<String> = Vec::new();
    for c in input.split(' ').filter(|x| x.chars().next().unwrap().is_alphabetic() ) {
        leading_char_set.insert(c.chars().next().unwrap());
        operands.push(c.to_owned());
    }
    // setup hashmap and hashset
    let mut map: HashMap<char, u8> = HashMap::new();
    let mut char_set: HashSet<char> = HashSet::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            map.entry(c).or_insert(0);
            char_set.insert(c);
        }
    }
    // brute force
    for p in (0..10).permutations(char_set.len()) {
        let mut skip = false;
        // get one of the permutations and set the value in map
        for (k, v) in char_set.iter().zip(p.iter()) {
            if leading_char_set.contains(k) && *v == 0 { // leading 0, skip
                skip = true;
                break;
            }
            map.entry(*k).and_modify(|c| *c = *v);
        }
        if skip { continue; }
        let mut sum: usize = 0;
        let mut record: HashMap<String, usize> = HashMap::new();
        for (idx, op) in operands.iter().enumerate() {
            if idx == operands.len()-1 { // reach rhs
                if sum == to_number(&map, op) {
                    return Some(map);
                }
            }
            match record.get(op) {
                None => {
                    let num = to_number(&map, op);
                    sum += num;
                    record.entry(op.to_owned()).or_insert(num);
                }
                Some(num) => {
                    sum += num;
                }
            }
        }
    }
    None
}
