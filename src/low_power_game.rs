#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_mod() {
        assert_eq!(div_mod(10, 3), (3, 1))
    }

    #[test]
    fn test_evens_int() {
        let mut even_ints = evens(0_u8..);
        assert_eq!(even_ints.next(), Some(0));
        assert_eq!(even_ints.next(), Some(2));
        assert_eq!(even_ints.next(), Some(4));
        assert_eq!(even_ints.next(), Some(6));
    }

    #[test]
    fn test_evens_from_odds() {
        let mut evens_from_odds = evens(1_i16..);
        assert_eq!(evens_from_odds.next(), Some(1));
        assert_eq!(evens_from_odds.next(), Some(3));
        assert_eq!(evens_from_odds.next(), Some(5));
        assert_eq!(evens_from_odds.next(), Some(7));
    }
}

fn div_mod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

fn evens<T>(iter: impl Iterator<Item=T>) -> impl Iterator<Item=T> {
    iter.step_by(2)
}
