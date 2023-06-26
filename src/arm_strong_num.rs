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
}

fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    let mul = num
        .to_string()
        .chars()
        .map(|v| v.to_digit(10).unwrap())
        .fold(0, |acc, cur| acc + cur.pow(len));
    num == mul
}
