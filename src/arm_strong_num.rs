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
    let mul: (u32, bool) = num
        .to_string()
        .chars()
        .map(|v| v.to_digit(10).unwrap())
        .fold((0, false), |acc: (u32, bool), cur: u32| {
            let is_over = acc.1;
            if is_over {
                (0, true)
            } else {
                acc.0.overflowing_add(cur.pow(digit_count))
            }
        });
    println!("mul {:?}", mul);
    !mul.1 && mul.0 == num
}
