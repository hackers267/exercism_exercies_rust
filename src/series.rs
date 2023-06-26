#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_zero_length() {
        let expected = vec!["".to_string(); 6];
        assert_eq!(series("92017", 0), expected)
    }

    #[test]
    fn test_with_length_2() {
        let expected = vec![
            "92".to_string(),
            "20".to_string(),
            "01".to_string(),
            "17".to_string(),
        ];
        assert_eq!(series("92017", 2), expected)
    }

    #[test]
    fn test_with_numbers_length() {
        let expected = vec!["92017".to_string()];
        assert_eq!(series("92017", 5), expected)
    }

    #[test]
    fn test_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 99), expected)
    }
}

fn series(digits: &str, len: usize) -> Vec<String> {
    let digit_len = digits.len();
    if len == 0 {
        return vec![String::default(); digit_len + 1];
    }
    if len > digit_len {
        return vec![];
    }
    let count = digit_len - len;
    (0..=count)
        .map(|index| {
            digits
                .chars()
                .map(|v| v.to_string())
                .skip(index)
                .take(len)
                .collect()
        })
        .collect()
}
