#[derive(Debug, PartialEq, Eq)]
enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    let first_len = first_list.len();
    let second_len = second_list.len();
    if first_len == 0 {
        return Comparison::Sublist;
    } else if second_len == 0 {
        return Comparison::Superlist;
    }
    if first_len < second_len {
        let result = second_list
            .windows(first_len)
            .any(|list| list == first_list);
        if result {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else {
        let result = first_list
            .windows(second_len)
            .any(|list| list == second_list);
        if result {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
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
