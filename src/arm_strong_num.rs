#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_9_is() {
        assert!(is_armstrong_number(9));
    }

    #[test]
    fn num_10_not() {
        assert!(!is_armstrong_number(10));
    }

    #[test]
    fn num_153_is() {
        assert!(is_armstrong_number(153))
    }

    #[test]
    fn num_154_not() {
        assert!(!is_armstrong_number(154))
    }

    #[test]
    fn num_4_106_098_957_not() {
        assert!(!is_armstrong_number(4_106_098_957))
    }
}

fn is_armstrong_number(num: u32) -> bool {
    let digit_count: u32 = num.ilog10() + 1;
    (0..digit_count)
        .map(|index| ((num / 10_u32.pow(index)) % 10))
        .try_fold(0_u32, |acc, cur| acc.checked_add(cur.pow(digit_count)))
        .is_some_and(|sum| sum == num)
}
