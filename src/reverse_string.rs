#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_str_hello() {
        let result = reverse("hello");
        assert_eq!(&result, "olleh");
    }

    #[test]
    fn reverse_str_world() {
        let result = reverse("world");
        assert_eq!(&result, "dlrow");
    }
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
