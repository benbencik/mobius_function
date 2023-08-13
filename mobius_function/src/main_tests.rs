#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    // fn calculate(bottom: Mperm, top: Mperm) -> i32 {
    //     let (downward_links, upward_links) = building_poset::build_poset(&bottom, &top);
    //     let result = mobius_func::naive(&bottom, &top, &upward_links);
    //     for (mperm, links) in &upward_links {
    //         println!("{:?}: {:?}", mperm, links);
    //     }
    //     return result;
    // }

    // #[test]
    // fn ascending1() {
    //     let result = calculate(
    //         Mperm::new(vec![1, 2, 3], 3),
    //         Mperm::new(vec![1, 2, 3, 3], 3),
    //     );
    //     assert_eq!(result, -1);
    // }

    // #[test]
    // fn ascending2() {
    //     let result = calculate(
    //         Mperm::new(vec![1, 2, 2], 2),
    //         Mperm::new(vec![1, 2, 3, 3], 3),
    //     );
    //     assert_eq!(result, -1);
    // }

    // #[test]
    // fn ascending3() {
    //     let result = calculate(
    //         Mperm::new(vec![1, 1], 1),
    //         Mperm::new(vec![1, 2, 3, 3], 3),
    //     );
    //     assert_eq!(result, -1);
    // }

    // // #[test]
    // // fn test_04() {
    // //     let bottom: Mperm = Mperm::new(vec![1, 2, 3], 3);
    // //     let top: Mperm = Mperm::new(vec![1, 2, 3, 4], 4);
    // //     assert_eq!(calculate(bottom, top), 1);
    // // }
}
