use mobius_funciton::is_contained;

#[test]
fn test_01() {
    let x = [1, 2, 3];
    let y = [1, 2, 3, 4];
    assert_eq!(is_contained(&x, &y), true);
}

#[test]
fn test_02() {
    let x = [1, 2, 3, 4];
    let y = [1, 2, 3, 4];
    assert_eq!(is_contained(&x, &y), true);
}

#[test]
fn test_03() {
    let x = [1, 2, 2];
    let y = [1, 2, 3, 4];
    assert_eq!(is_contained(&x, &y), false);
}

#[test]
fn test_04() {
    let x = [1, 1, 1];
    let y = [1, 2, 3, 4];
    assert_eq!(is_contained(&x, &y), false);
}

#[test]
fn test_05() {
    let x = [1, 2, 2];
    let y = [1, 2, 3, 3];
    assert_eq!(is_contained(&x, &y), true);
}

#[test]
fn test_06() {
    let x = [4, 4, 4];
    let y = [4, 3, 2, 1];
    assert_eq!(is_contained(&x, &y), false);
}

#[test]
fn test_07() {
    let x = [4, 3, 2];
    let y = [4, 3, 2, 1];
    assert_eq!(is_contained(&x, &y), true);
}

#[test]
fn test_08() {
    let x = [3, 2, 1];
    let y = [7, 6, 5, 4];
    assert_eq!(is_contained(&x, &y), true);
}

#[test]
fn test_09() {
    let x = [7, 5, 6];
    let y = [7, 6, 5, 4];
    assert_eq!(is_contained(&x, &y), false);
}

#[test]
fn test_10() {
    // 
    let x = [1];
    let y = [1, 2, 3];
    assert_eq!(is_contained(&x, &y), false);
}

#[test]
fn test_11() {
    let x = [1, 2, 3];
    let y = [1, 2];
    assert_eq!(is_contained(&x, &y), false);
}

