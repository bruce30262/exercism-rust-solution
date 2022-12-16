/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome {
    value:u64,
}

fn is_palindrome(v: u64) -> bool {
    // filter numbers with simple check
    let (mut h, l) = (v, v % 10);
    while h/10 != 0 {
        h /= 10;
    }
    if h != l { return false; }
    // do check
    if v.to_string().chars().rev().collect::<String>() == v.to_string().chars().collect::<String>() {
        true
    } else {
        false
    }
}

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome{value})
        } else {
            None
        }
    }
    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.value
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max { return None; }
    let mut pmin = Palindrome::new(10);
    let mut pmax = Palindrome::new(10);
    for i in min..max+1 {
        for j in i..max+1 {
            let v:u64 = j*i;
            if is_palindrome(v) {
                if pmin == None {
                    pmin = Palindrome::new(v);
                }
                if pmax == None || pmax.unwrap().into_inner() < v {
                    pmax = Palindrome::new(v);
                }
            }
        }
    }
    match pmin == None {
        true => None,
        false => Some((pmin.unwrap(), pmax.unwrap())),
    }
}

