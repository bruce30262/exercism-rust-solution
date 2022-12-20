use std::collections::BTreeSet;
use std::collections::HashMap;
use std::cmp::{max, min};
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Point {
    x: usize,
    y: isize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn check_rec_ok(src: &Point, dst: &Point, art_str: &str, line_len:usize) -> bool {
    let Point {x: x1, y: y1} = *src;
    let Point {x: x2, y: y2} = *dst;
    // check if we can walk from x1 to x2 from both y1 & y2
    for x in min(x1, x2)..max(x1, x2)+1 {
        match art_str.chars().nth((y1.abs() as usize) * line_len + x).unwrap() {
            '-' | '+' => {
                match art_str.chars().nth((y2.abs() as usize) * line_len + x).unwrap() {
                    '-' | '+' => continue,
                    _ => return false,
                }
            }
            _ => return false,
        }
    }
    // check if we can walk from y1 to y2 from both x1 & x2
    for y in min(y1, y2)..max(y1, y2)+1 {
        match art_str.chars().nth((y.abs() as usize) * line_len + x2).unwrap() {
            '|' | '+' => {
                match art_str.chars().nth((y.abs() as usize) * line_len + x1).unwrap() {
                    '|' | '+' => continue,
                    _ => return false,
                }
            }
            _ => return false,
        }
    }
    true
}

pub fn count(lines: &[&str]) -> u32 {
    // read and sort Point ( location of '+' )
    let mut y = 0;
    let mut line_len:usize = 0;
    let mut art_str = "".to_owned();
    let mut pv: Vec<Point> = Vec::new();
    for line in lines {
        line_len = line.len();
        art_str.push_str(line);
        for (x, c) in line.chars().enumerate() {
            if c == '+' { pv.push(Point{x,  y}); }
        }
        y -= 1;
    }
    if pv.len() == 0 { return 0; }
    pv.sort();
    // calculate rectangle count
    let mut rec_map:HashMap<String, bool> = HashMap::new();
    for i in 0..pv.len()-1 {
        let Point {x: x1, y: y1} = pv[i]; // src
        for j in i+1..pv.len()-1 {
            let Point {x: x2, y: y2} = pv[j]; // dst
            if x1 == x2 || y1 == y2 || !check_rec_ok(&pv[i], &pv[j], &art_str, line_len){ continue; }
            // if reach here, we got a rectangle
            let mut point_set: BTreeSet<Point> = BTreeSet::new(); 
            point_set.insert(pv[i]);
            point_set.insert(pv[j]);
            for mid in pv.iter().filter(|&x| *x != pv[i] && *x != pv[j]) {
                let Point {x: x3, y: y3} = mid; // mid point
                if ( *x3 == x1 && *y3 == y2) || ( *x3 == x2 && *y3 == y1 ) {
                    point_set.insert(*mid);
                }
            }
            assert_eq!(point_set.len(), 4); // should only contain 4 points
            let mut rec_str = "".to_owned();
            for p in point_set.iter() { // construct string of rectangle ( 4 points )
                rec_str.push_str(&p.to_string());
            }
            if !rec_map.contains_key(&rec_str) { // add new rectangle only
                rec_map.insert(rec_str, true);
            }
        }
    }
    rec_map.keys().len() as u32
}
