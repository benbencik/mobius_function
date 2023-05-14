// Mobuis function on a poset


fn main() {
    // let poset: [i32; 5] = [1, 2, 3, 4, 5];
    let l: i32 = 0;
    let r: i32 = 4;
    mobius_function(l, r, 0);

}



fn mobius_function(l: i32, r: i32, val: i32) -> i32{
    if l == r {
        return 1;
    }
    else if l < r{
        for i in l..r {
            // let val = val + mobius_function(poset, i, r, val);
            println!("{}", i);
        }
        return val;
    }
    else {
        return 0;
    }
}