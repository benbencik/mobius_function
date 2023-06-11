use std::collections::HashMap;

fn main() {
    let left_bound = 0;
    let right_bound = 100;

    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    let result = mobius_function(left_bound, right_bound, &mut memo);
    println!("Result: {}", result);
}

fn mobius_function(left: i32, right: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    if left == right {
        return 1;
    }
    else if memo.contains_key(&(left, right)) {
        return memo.get(&(left, right)).unwrap().clone();
    }
    else{
        let mut res: i32 = 0;
        for i in left + 1..=right {
            res += mobius_function(i, right, memo);
        }
        if !memo.contains_key(&(left, right)){
            memo.insert((left, right), -res);
        }
        return -res;
    }
}
    
