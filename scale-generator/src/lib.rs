// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

use std::fmt;

const SCALE1:&[&str] = &["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
const SCALE2:&[&str] = &["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];
const SHARPS:&[&str] = &["G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#", "C", "a"];
const FLATS:&[&str] = &["F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb"];

// https://doggor.github.io/posts/2020/custom-errors-in-rust/
#[derive(Debug)]
pub enum Error {
    NoteNotFoundError(String),
    IntervalNotFoundError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoteNotFoundError(ref note) =>
                write!(f, "Note {} not found", note),
            
            Error::IntervalNotFoundError(ref interval) =>
                write!(f, "Interval {} not found", interval),
        }
    }
}

pub fn first_letter_to_uppercase(s: &str) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v[0] = v[0].to_uppercase().next().unwrap();
    v.into_iter().collect()
}

pub fn pick_scale(tonic: &str) -> Result<&[&str], Error> {
    if SHARPS.contains(&tonic) {
        Ok(SCALE1)
    } else if FLATS.contains(&tonic) {
        Ok(SCALE2)
    } else {
        Err(Error::NoteNotFoundError(tonic.to_owned()))
    }
}

#[derive(Debug)]
pub struct Scale {
    data: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        // pick a scale
        let scale:&[&str] = match pick_scale(tonic) {
            Ok(s) => s,
            Err(err) => return Err(err),
        };
        // push first note after convert its first letter to uppercase
        let mut res:Vec<String> = Vec::new();
        let tonic_real:String = first_letter_to_uppercase(tonic);
        res.push(tonic_real.to_owned());
        // gen notes
        let size = scale.len();
        let mut index = scale.iter().position(|&x| x == tonic_real).unwrap();
        for i in intervals.chars() {
            match i {
                'M' => index = (index + 2) % size,
                'm' => index = (index + 1) % size,
                'A' => index = (index + 3) % size,
                _ => return Err(Error::IntervalNotFoundError(i.to_string())),
            }
            res.push(scale[index].to_owned())
        }
        Ok(Scale{ data: res})
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.data.clone()
    }
}

