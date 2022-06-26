// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let N:u64 = 221;
    let S:u64 = speed as u64;
    if speed >= 1 && speed <= 4 {
        (S * N) as f64
    } else if speed >= 5 && speed <= 8 {
        (S * N) as f64 * 0.9
    } else if speed >= 9 && speed <= 10 {
        (S * N) as f64 * 0.77
    } else {
        0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
