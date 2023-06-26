#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_of_sum_1() {
        assert_eq!(square_of_sum(1), 1)
    }

    #[test]
    fn test_square_of_sum_5() {
        assert_eq!(square_of_sum(5), 225)
    }

    #[test]
    fn test_square_of_sum_100() {
        assert_eq!(square_of_sum(100), 25_502_500)
    }

    #[test]
    fn test_sum_of_square_1() {
        assert_eq!(sum_of_square(1), 1)
    }

    #[test]
    fn test_sum_of_square_5() {
        assert_eq!(sum_of_square(5), 55)
    }

    #[test]
    fn test_sum_of_square_100() {
        assert_eq!(sum_of_square(100), 338_350)
    }

    #[test]
    fn test_diff_1() {
        assert_eq!(difference(1), 0)
    }

    #[test]
    fn test_diff_5() {
        assert_eq!(difference(5), 170)
    }

    #[test]
    fn test_diff_100() {
        assert_eq!(difference(100), 25_164_150)
    }
}

fn square_of_sum(n: u32) -> u32 {
    let sum = (0..=n).fold(0, |acc, cur| acc + cur);
    sum.pow(2)
}

fn sum_of_square(n: u32) -> u32 {
    (0..=n).map(|v| v.pow(2)).fold(0, |acc, cur| acc + cur)
}

fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_square(n)
}
