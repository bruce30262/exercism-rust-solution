use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match value {
        0..=9 => {
            let res = format!("{:?}", ResistorColor::from_int(value));
            res[3..res.len()-1].to_string()
        }
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut res = all::<ResistorColor>().collect::<Vec<_>>();
    res.sort_by_key(|a| a.int_value());
    res
}
