mod is_contained;
use is_contained::is_contained;

fn main() {
    let x = [1, 3, 3];
    let y = [1, 2, 3, 3];
    println!("Result: {}", is_contained(&x, &y));
}
