pub fn is_leap_year(year: u64) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, _, _) => true,
        (_, 0, _) => false,
        (_, _, 0) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2015() {
        assert!(!is_leap_year(2015));
    }

    #[test]
    fn test_2016() {
        assert!(is_leap_year(2016));
    }

    #[test]
    fn test_1900() {
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_2000() {
        assert!(is_leap_year(2000));
    }
}
