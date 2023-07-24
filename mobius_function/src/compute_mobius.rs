use std::collections::{HashMap, VecDeque};

pub fn compute(start: &Vec<u8>, goal: &Vec<u8>, poset: &HashMap<Vec<u8>, Vec<Vec<u8>>>) -> HashMap<Vec<u8>, i32> {
    let mut value: HashMap<Vec<u8>, i32> = HashMap::new();
    let mut queue: VecDeque<Vec<u8>> = VecDeque::new();
    
    queue.push_back(start.clone());
    value.insert(start.clone(), 1);

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        let neighbors = poset.get(&current).unwrap();
        let parent_val = value.entry(current.clone()).or_insert(0);
        *parent_val *= -1;
        let ppp = parent_val.clone();
        
        if current == *goal {
            return value;
        }

        for i in 0..neighbors.len() {
            let sub_mperm = neighbors[i].clone();
            queue.push_back(sub_mperm.clone());
            let val = value.entry(sub_mperm.clone()).or_insert(0);
            *val += ppp;
        }
    }
    return value;
}