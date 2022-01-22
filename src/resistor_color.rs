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
}

#[derive(Debug, PartialEq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9,
    }
}
