use std::fmt::{Display, Formatter};

use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_black() {
        assert_eq!(color_to_value(ResistorColor::Black), 0);
    }

    #[test]
    fn test_is_brown() {
        assert_eq!(color_to_value(ResistorColor::Brown), 1);
    }

    #[test]
    fn test_is_red() {
        assert_eq!(color_to_value(ResistorColor::Red), 2);
    }

    #[test]
    fn test_is_orange() {
        assert_eq!(color_to_value(ResistorColor::Orange), 3);
    }

    #[test]
    fn test_is_yellow() {
        assert_eq!(color_to_value(ResistorColor::Yellow), 4);
    }

    #[test]
    fn test_is_green() {
        assert_eq!(color_to_value(ResistorColor::Green), 5);
    }

    #[test]
    fn test_is_blue() {
        assert_eq!(color_to_value(ResistorColor::Blue), 6);
    }

    #[test]
    fn test_is_violet() {
        assert_eq!(color_to_value(ResistorColor::Violet), 7);
    }

    #[test]
    fn test_is_grey() {
        assert_eq!(color_to_value(ResistorColor::Grey), 8);
    }

    #[test]
    fn test_is_white() {
        assert_eq!(color_to_value(ResistorColor::White), 9);
    }

    #[test]
    fn test_0_is_black() {
        assert_eq!(value_to_color_string(0), String::from("Black"));
    }

    #[test]
    fn test_1_is_brown() {
        assert_eq!(value_to_color_string(1), String::from("Brown"));
    }

    #[test]
    fn test_2_is_red() {
        assert_eq!(value_to_color_string(2), String::from("Red"));
    }

    #[test]
    fn test_3_is_orange() {
        assert_eq!(value_to_color_string(3), String::from("Orange"));
    }

    #[test]
    fn test_4_is_yellow() {
        assert_eq!(value_to_color_string(4), String::from("Yellow"));
    }

    #[test]
    fn test_5_is_green() {
        assert_eq!(value_to_color_string(5), String::from("Green"));
    }

    #[test]
    fn test_6_is_blue() {
        assert_eq!(value_to_color_string(6), String::from("Blue"));
    }

    #[test]
    fn test_7_is_violet() {
        assert_eq!(value_to_color_string(7), String::from("Violet"));
    }

    #[test]
    fn test_8_is_grey() {
        assert_eq!(value_to_color_string(8), String::from("Grey"));
    }

    #[test]
    fn test_9_is_white() {
        assert_eq!(value_to_color_string(9), String::from("White"));
    }

    #[test]
    fn test_out_range() {
        assert_eq!(
            value_to_color_string(11),
            String::from("value out of range")
        );
    }

    #[test]
    fn test_colors() {
        assert!(colors().contains(&ResistorColor::Green));
        assert!(colors().contains(&ResistorColor::Grey));
        assert!(colors().contains(&ResistorColor::Red));
        assert!(colors().contains(&ResistorColor::Yellow));
        assert!(colors().contains(&ResistorColor::Black));
        assert!(colors().contains(&ResistorColor::Brown));
        assert!(colors().contains(&ResistorColor::Blue));
        assert!(colors().contains(&ResistorColor::Violet));
        assert!(colors().contains(&ResistorColor::White));
        assert!(colors().contains(&ResistorColor::Orange));
        assert_eq!(colors().len(), 10);
    }
}

#[repr(usize)]
#[derive(Debug, IntoEnumIterator, Eq, PartialEq, Copy, Clone, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    let result = ResistorColor::from_int(value);
    if result.is_ok() {
        result.unwrap().to_string()
    } else {
        String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
