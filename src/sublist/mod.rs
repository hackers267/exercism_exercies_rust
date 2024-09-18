#[derive(Debug, PartialEq, Eq)]
enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let super_list = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|v| v == second_list);
    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|v| v == first_list);
    match (super_list, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sublist1() {
        let a = [1, 2, 3];
        let b = [1, 2, 3, 4, 5];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Sublist);
    }

    #[test]
    fn sublist2() {
        let a = [3, 4, 5];
        let b = [1, 2, 3, 4, 5];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Sublist);
    }

    #[test]
    fn sublist3() {
        let a = [3, 4];
        let b = [1, 2, 3, 4, 5];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Sublist);
    }
    #[test]
    fn sublist4() {
        let a = [1, 1, 2];
        let b = [1, 1, 1, 2];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Sublist);
    }
    #[test]
    fn sublist5() {
        let a = [1, 2, 1, 2, 3];
        let b = [1, 2, 3, 1, 2, 1, 2, 3, 2, 1];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Sublist);
    }
    #[test]
    fn equal() {
        let a = [1, 2, 3];
        let b = [1, 2, 3];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Equal);
    }
    #[test]
    fn superlist() {
        let a = [1, 2, 3, 4, 5];
        let b = [2, 3, 4];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Superlist);
    }

    #[test]
    fn unequal() {
        let a = [1, 2, 4];
        let b = [1, 2, 3, 4, 5];
        let result = sublist(&a, &b);
        assert_eq!(result, Comparison::Unequal);
    }
}
