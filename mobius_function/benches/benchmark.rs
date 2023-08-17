use criterion::{criterion_group, criterion_main, Criterion};
use mobius_funciton::{building_poset, io, mobius_func, multipermutation};

fn run_test_file(path: &str) {
    let res = io::load_tests(path);
    match res {
        Ok((tests, result)) => {
            for (test, res) in tests.iter().zip(result.iter()) {
                let bottom = multipermutation::Mperm::new(vec![1]);
                let c = test.clone();
                let top = multipermutation::Mperm::new(c);
                let (downward_links, upward_links) = building_poset::build_poset(&bottom, &top);
                let result = mobius_func::naive(&bottom, &top, &downward_links, &upward_links);
                assert_eq!(result, *res);
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn benchmark_test_files(c: &mut Criterion) {
    let test_files = vec![
        "../test_files/all5.txt",
        "../test_files/all6.txt",
        "../test_files/all7.txt",
        //"../test_files/all8.txt",
        //"../test_files/all9.txt",
    ];
    c.bench_function("test 0", |b| b.iter(|| run_test_file(test_files[0])));
    c.bench_function("test 1", |b| b.iter(|| run_test_file(test_files[1])));
    c.bench_function("test 2", |b| b.iter(|| run_test_file(test_files[2])));
}

criterion_group!(benches, benchmark_test_files);
criterion_main!(benches);
