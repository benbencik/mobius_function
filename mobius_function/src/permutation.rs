use std::cmp::Ordering;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

struct Multipermutation {
    values: Vec<char>,
    max_character: char,
}

impl Multipermutation {
    fn new(input: &str, max_character: char) -> Multipermutation {
        let values: Vec<char> = input.chars().collect();
        Multipermutation {
            values ,
            max_character,
        }
    }

    fn compare(&self, other: &Multipermutation) -> Ordering {
        // Compare the values of the two permutations
        for i in 0..self.values.len() {
            if self.values[i] > other.values[i] {
                return Ordering::Greater;
            } else if self.values[i] < other.values[i] {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }

    fn successor(&mut self) -> bool {
        let mut i = self.values.len() - 1;
        
        while i >= 0 {
            if self.values[i] < self.max_character {
                self.values[i] = (self.values[i] as u8 + 1) as char;
                break;
            }
            else{
                if i == 0 { return false; } // No successor exists
                i -= 1;
            }
        }
        return true;
    }
}

fn main() {
    let mut perm1 = Multipermutation::new("abb", 'c');
    let mut perm2 = Multipermutation::new("ccc", 'c');
    
    // println!("Result: {:?}", perm1.compare(&perm2) == Ordering::Greater);
    let result = mobius_function(&mut perm1, &mut perm2);
    println!("Result: {}", result);
}

fn mobius_function(left: &mut Multipermutation, right: &mut Multipermutation) -> i32 {
    if left.compare(&right) == Ordering::Greater {
        return 1;
    }
    else {
        let mut res: i32 = 0;
        while left.compare(&right) == Ordering::Less {
            left.successor();
            res += mobius_function(left, right);
        }    
        return -res;
    }
}