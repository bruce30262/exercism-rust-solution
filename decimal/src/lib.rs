use num_bigint::{BigInt, Sign};
use std::cmp::{max, Ordering};
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
pub struct Decimal {
    left: BigInt,
    right: BigInt,
    after_dot: isize,
}

fn ten_pow_bigint(p: isize) -> BigInt {
    BigInt::from(10).pow(p.try_into().unwrap())
}

fn get_times_val(dot_self: isize, dot_other: isize) -> BigInt {
    ten_pow_bigint((dot_self - dot_other).abs())
}

impl Decimal {
    fn plus_and_minus(&self) -> bool {
        if (self.left.sign() == Sign::Plus && self.right.sign() == Sign::Minus) ||   
           (self.left.sign() == Sign::Minus && self.right.sign() == Sign::Plus) {
            return true;
        }
        false
    }

    pub fn to_integer_bigint(&self, after_dot_max: isize) -> BigInt {
        self.left.clone()*ten_pow_bigint(after_dot_max) + self.right.clone()*get_times_val(self.after_dot, after_dot_max)
    }

    pub fn carry(&mut self) {
        let mut tmp = self.right.clone();
        for _ in 0..self.after_dot {
            tmp /= 10;
        }

        if tmp != BigInt::from(0) {
            match tmp.sign() {
                Sign::Minus => {
                    self.left -= 1;
                    self.right += ten_pow_bigint(self.after_dot);
                }
                _ => { 
                    self.left += 1;
                    self.right -= ten_pow_bigint(self.after_dot);
                }
            }
        }

        if self.plus_and_minus() {
            match self.left.sign() {
                Sign::Minus => {
                    self.left += 1;
                    self.right -= ten_pow_bigint(self.after_dot);
                }
                _ => {
                    self.left -= 1;
                    self.right += ten_pow_bigint(self.after_dot);
                }
            }
        }
    
        while self.right.clone() % 10 == BigInt::from(0) && self.after_dot != 0 {
            self.right /= 10;
            self.after_dot -= 1;
        }
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let (mut left, mut right, mut after_dot) = (BigInt::from(0), BigInt::from(0), 0);
        for (idx, data) in input.split('.').filter(|x| !x.is_empty()).enumerate() {
            match idx {
                0 => left = BigInt::parse_bytes(data.as_bytes(), 10).unwrap(),
                1 => {
                    let tmp = data.trim_end_matches('0').as_bytes();
                    if !tmp.is_empty() {
                        right = BigInt::parse_bytes(tmp, 10).unwrap();
                        if input.starts_with('-') { right = -right; }
                    }
                    after_dot = tmp.len() as isize;
                }
                _ => { return None; }
            }
        }
        Some(Decimal{left, right, after_dot})
    }
}
// PartialEq
impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left &&
            self.right == other.right &&
            self.after_dot == other.after_dot
    }
}
//PartialOrd
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let after_dot = max(self.after_dot, other.after_dot);
        let tmp1 = self.to_integer_bigint(after_dot);
        let tmp2 = other.to_integer_bigint(after_dot);
        Some(tmp1.cmp(&tmp2))
    }
}
//Add
impl Add for Decimal {
    // https://stackoverflow.com/questions/39115363/why-do-rusts-operators-have-the-type-output-variable
    type Output = Decimal;
    fn add(self, other: Self) -> Self {
        let after_dot = max(self.after_dot, other.after_dot);
        let tmp1 = self.to_integer_bigint(after_dot);
        let tmp2 = other.to_integer_bigint(after_dot);
        let left = (tmp1.clone() + tmp2.clone()) / ten_pow_bigint(after_dot);
        let right = (tmp1 + tmp2) % ten_pow_bigint(after_dot);

        let mut ret = Self { left, right, after_dot };
        ret.carry();
        ret
    }
}
// Sub
impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Self) -> Self {
        let after_dot = max(self.after_dot, other.after_dot);
        let tmp1 = self.to_integer_bigint(after_dot);
        let tmp2 = other.to_integer_bigint(after_dot);
        let left = (tmp1.clone() - tmp2.clone()) / ten_pow_bigint(after_dot);
        let right = (tmp1 - tmp2) % ten_pow_bigint(after_dot);

        let mut ret = Self { left, right, after_dot };
        ret.carry();
        ret
    }
}
// Mul
impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Self) -> Self {
        let after_dot = self.after_dot + other.after_dot;
        let tmp1 = self.to_integer_bigint(self.after_dot);
        let tmp2 = other.to_integer_bigint(other.after_dot);
        let left = tmp1.clone() * tmp2.clone() / ten_pow_bigint(after_dot);
        let right = tmp1 * tmp2 % ten_pow_bigint(after_dot);

        let mut ret = Self { left, right, after_dot };
        ret.carry();
        ret
    }
}
