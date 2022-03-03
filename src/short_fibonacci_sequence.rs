#[cfg(test)]
mod test {
    use crate::short_fibonacci_sequence::{create_buffer, create_empty, fibonacci};

    #[test]
    fn create_empty_test() {
        let vec = create_empty();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn create_buffer_with_five() {
        let vec = create_buffer(5);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn fibonacci_test() {
        let vec = fibonacci();
        assert_eq!(vec.len(), 5);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[2], 2);
        assert_eq!(vec[3], 3);
        assert_eq!(vec[4], 5);
    }
}

pub fn create_empty() -> Vec<u8> {
    return Vec::new();
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut vec = vec![1; 5];
    for i in 2..5 {
        vec[i] = vec[i - 1] + vec[i - 2];
    }
    return vec;
}
