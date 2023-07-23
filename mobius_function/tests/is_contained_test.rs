use mobius_funciton::is_contained;

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

