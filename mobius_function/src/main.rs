mod building_poset;
mod io;
mod is_contained;
mod mobius_func;
mod multipermutation;

use multipermutation::Mperm;

fn main() {
    let bottom: Mperm = Mperm::new(vec![1]);
    let top: Mperm = Mperm::new(vec![1, 2, 3, 5, 4]);

    let (downward_links, upward_links) = building_poset::build_poset(&bottom, &top);
    let result = mobius_func::naive(&bottom, &top, &downward_links, &upward_links);
    println!("Result of μ({:?}, {:?}) is: {}", bottom, top, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test_file(path: &str) {
        let res = io::load_tests(path);
        match res {
            Ok((tests, result)) => {
                for (test, res) in tests.iter().zip(result.iter()) {
                    let bottom = Mperm::new(vec![1]);
                    let c = test.clone();
                    let top = Mperm::new(c);
                    let (downward_links, upward_links) = building_poset::build_poset(&bottom, &top);
                    let result = mobius_func::naive(&bottom, &top, &downward_links, &upward_links);
                    assert_eq!(result, *res);
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn test5() {
        run_test_file("../test_files/all5.txt");
    }

    #[test]
    fn test6() {
        run_test_file("../test_files/all5.txt");
    }

    #[test]
    fn test7() {
        run_test_file("../test_files/all5.txt");
    }

    #[test]
    fn test8() {
        run_test_file("../test_files/all5.txt");
    }

    #[test]
    fn test9() {
        run_test_file("../test_files/all5.txt");
    }
}
