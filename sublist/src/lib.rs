use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sol<T: PartialEq>(big: &[T], big_sz: usize, small: &[T], small_sz: usize) -> bool {
    if (big_sz == 0) || (big_sz > 0 && small_sz == 0) { // both empty, or empty small list is sublist of big list
        return true;
    }

    big.windows(small_sz).any(|v| v == small)
}

// Any type that has implement PartialEq can be passed into this function
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (len1, len2) = (_first_list.len(), _second_list.len());
    
    match len1.cmp(&len2) {
        Ordering::Greater => {
            // super list or unequal
            match sol(_first_list, len1, _second_list, len2) {
                true => Comparison::Superlist,
                false => Comparison::Unequal
            }
        }
        Ordering::Equal => {
            // equal or unequal
            match sol(_first_list, len1, _second_list, len2) {
                true => Comparison::Equal,
                false => Comparison::Unequal
            }
        }
        Ordering::Less => {
            // sublist or unequal
            match sol(_second_list, len2, _first_list, len1) {
                true => Comparison::Sublist,
                false => Comparison::Unequal
            }
        }
    }   
}
