fn main() {
    let x = [1, 2, 3];
    let y = [1, 2, 3, 4];
    println!("Result: {}", naive(&x, &y));
}

// the function recieves two multipermutations 
pub fn naive(a: &[i32], b: &[i32]) -> bool {
    let n: usize = b.len();
    let mut valid;

    // choose omitted element
    for i in 0..n {
        // itterate through all positions
        valid = true;
        for j in 0..n-1 {
            for k in 0..n-1 {
                let elem1 = if j < i {j} else {j+1};
                let elem2 = if k < i {k} else {k+1};
                if a[j] <= a[k] && elem1 > elem2 {
                    valid = false;
                    break;
                }
            }
            if ! valid { break; }
        }
        if valid { return true; }
    }
    return false;
}