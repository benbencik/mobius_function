use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_tests(path: &str) -> Result<(Vec<Vec<u8>>, Vec<i32>), Box<dyn std::error::Error>> {
    let path = Path::new(&path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data_vec: Vec<Vec<u8>> = Vec::new();
    let mut result: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num_str| num_str.parse::<i32>().unwrap())
            .collect();

        let last_num = numbers.pop().unwrap();
        result.push(last_num);

        let mut unums: Vec<u8> = Vec::new();
        for x in numbers {
            unums.push(x as u8);
        }
        data_vec.push(unums);
    }

    return Ok((data_vec, result));
}
