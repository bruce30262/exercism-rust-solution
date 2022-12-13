// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    data: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { data: HashMap::new() }
    }   

    pub fn add(&mut self, grade: u32, student: &str) {
        let cur_grade = self.data.entry(grade).or_insert(Vec::new());
        cur_grade.push(student.to_owned());
    }   

    pub fn grades(&self) -> Vec<u32> {
        // copied(): An iterator that copies the elements of an underlying iterator.
        let mut v = Vec::from_iter(self.data.keys().copied());
        v.sort();
        v
    }   

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.data.get(&grade) {
            None => Vec::new(),
            Some(cur_grade) => {
                let mut v = cur_grade.clone();
                v.sort();
                v
            }
        }
    }
}
