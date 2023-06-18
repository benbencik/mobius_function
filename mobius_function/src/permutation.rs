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

        print_type_of(&i);
        
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
    let mut perm = Multipermutation::new("abb", 'c');
    
    perm.successor();
    perm.successor();

    println!("{:?}", perm.values);   
}