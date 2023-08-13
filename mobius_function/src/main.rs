mod building_poset;
mod is_contained;
mod mobius_func;
mod multipermutation;

use multipermutation::Mperm;

fn main() {
    let x: Mperm = Mperm::new(vec![1, 2], 2);
    // let y: Mperm = Mperm::new(vec![1, 2, 3, 3], 3);
    let y: Mperm = Mperm::new(vec![1, 2, 3, 3, 4], 3);

    let (downward_links, upward_links) = building_poset::build_poset(&x, &y);
    // for (mperm, links) in &downward_links {
    //     println!("{:?}: {:?}", mperm, links);
    // }
    // println!("");
    // for (mperm, links) in &upward_links {
    //     println!("{:?}: {:?}", mperm, links);
    // }

    let result = mobius_func::naive(&x, &y, &downward_links, &upward_links);
    println!("{}", result);
}

