use std::collections::{LinkedList, HashMap};
use crate::is_contained::is_contained;

/* 
NAIVE IMPLEMENTATION =================================================================
*/
pub fn build_poset(start: &Vec<u8>, goal: &Vec<u8>) -> HashMap<Vec<u8>, Vec<Vec<u8>>>{
    /*
    Poset is built from the goal permutation
    Hashmap contains links between the multipermutations 
     */

    let mut queue: LinkedList<Vec<u8>> = LinkedList::new();
    let mut mperms: HashMap<Vec<u8>, Vec<Vec<u8>>> = HashMap::new();

    queue.push_back(start.clone());
    mperms.insert(start.clone(), Vec::new());

    while queue.len() > 0 {
        let current_mperm = queue.pop_front().unwrap();
        // do not need to continue if current_mperm is contained in goal
        if is_contained(&current_mperm, goal) { continue; }

        // decremetning elements by one
        for i in 0..current_mperm.len() {
            let mut new_mperm = current_mperm.clone();
            if new_mperm[i] > 1 {
                new_mperm[i] -= 1;
                if is_contained(&new_mperm, &current_mperm){
                    if !mperms.contains_key(&new_mperm) {
                        queue.push_back(new_mperm.clone());
                        mperms.insert(new_mperm.clone(), Vec::new());
                    }
                    mperms.get_mut(&current_mperm).unwrap().push(new_mperm.clone());
                }
            }
        }

        // removing single elements
        if current_mperm.len() > goal.len() {
            for i in 0..current_mperm.len() {
                let mut new_mperm = current_mperm.clone();
                new_mperm.remove(i);
                if is_contained(&new_mperm, &current_mperm){
                    if !mperms.contains_key(&new_mperm) {
                        queue.push_back(new_mperm.clone());
                        mperms.insert(new_mperm.clone(), Vec::new());
                    }
                    mperms.get_mut(&current_mperm).unwrap().push(new_mperm.clone());
                }
            }
        }
    }
    return mperms;
}
