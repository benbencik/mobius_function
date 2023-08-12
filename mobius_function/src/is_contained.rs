pub fn is_contained(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    /*
    Naive implementation of a function that checks if
    multipermutation "a" is contained in "b"

    Assusmes that "a" has |b| or |b-1| elements
    */

    let n: usize = b.len();
    let mut valid;

    // check for assumption
    if b.len() < a.len() {
        return false;
    }
    if (b.len() - a.len()) > 1 {
        return false;
    }

    if a.len() == b.len() {
        // |a| == |b|
        valid = true;
        for j in 0..n - 1 {
            // itterate through all positions
            for k in j + 1..n {
                if (a[j] == a[k] && b[j] != b[k]) || (a[j] <= a[k] && b[j] > b[k]) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            return true;
        }
    } else {
        // |a| < |b|
        for i in 0..n {
            // choose omitted element
            valid = true;
            '_first_index: for j in 0..n - 2 {
                // itterate through all positions
                '_second_index: for k in j + 1..n - 1 {
                    let elem1 = if j < i { j } else { j + 1 };
                    let elem2 = if k < i { k } else { k + 1 };
                    if (a[j] == a[k] && b[elem1] != b[elem2])
                        || (a[j] <= a[k] && b[elem1] > b[elem2])
                    {
                        valid = false;
                        break '_first_index;
                    }
                }
            }
            if valid {
                return true;
            }
        }
    }
    return false;
}

mod tests {
    use super::is_contained;

    #[test]
    fn test_01() {
        assert_eq!(is_contained(&vec![1, 2, 3] , &vec![1, 2, 3, 4]), true);
    }

    #[test]
    fn test_02() {
        assert_eq!(is_contained(&vec![1, 2, 3, 4], &vec![1, 2, 3, 4]), true);
    }

    #[test]
    fn test_03() {
        assert_eq!(is_contained(&vec![1, 2, 2], &vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test_04() {
        assert_eq!(is_contained(&vec![1, 1, 1], &vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test_05() {
        assert_eq!(is_contained(&vec![1, 2, 2] , &vec![1, 2, 3, 3]), true);
    }

    #[test]
    fn test_06() {
        assert_eq!(is_contained(&vec![4, 4, 4] , &vec![4, 3, 2, 1]), false);
    }

    #[test]
    fn test_07() {
        assert_eq!(is_contained(&vec![4, 3, 2], &vec![4, 3, 2, 1]), true);
    }

    #[test]
    fn test_08() {
        assert_eq!(is_contained(&vec![3, 2, 1], &vec![7, 6, 5, 4]), true);
    }

    #[test]
    fn test_09() {
        assert_eq!(is_contained(&vec![7, 5, 6], &vec![7, 6, 5, 4]), false);
    }

    #[test]
    fn test_10() {
        assert_eq!(is_contained(&vec![1], &vec![1, 2, 3]), false);
    }

    #[test]
    fn test_11() {
        assert_eq!(is_contained(&vec![1, 2, 3], &vec![1, 2]), false);
    }
}