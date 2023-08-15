use crate::multipermutation::Mperm;
use std::collections::{HashMap, LinkedList};

pub fn build_poset(
    poset_bottom: &Mperm,
    poset_top: &Mperm,
) -> (HashMap<Mperm, Vec<Mperm>>, HashMap<Mperm, Vec<Mperm>>) {
    let mut downward_links: HashMap<Mperm, Vec<Mperm>> = HashMap::new();
    let mut upward_links: HashMap<Mperm, Vec<Mperm>> = HashMap::new();
    let mut queue: LinkedList<Mperm> = LinkedList::new();

    queue.push_back(poset_top.clone());

    while queue.len() > 0 {
        let current_mperm = queue.pop_front().unwrap();
        if &current_mperm == poset_bottom {
            continue;
        }
        
        if !downward_links.contains_key(&current_mperm) {
            let links = current_mperm.gen_submperms();

            for new_mperm in links {
                if new_mperm.len() >= poset_bottom.len() {
                    queue.push_back(new_mperm.clone());
                }

                // downward links
                if !downward_links.contains_key(&current_mperm) {
                    downward_links.insert(current_mperm.clone(), vec![new_mperm.clone()]);
                } else {
                    let links = downward_links.get_mut(&current_mperm).unwrap();
                    links.push(new_mperm.clone());
                }

                // upward links
                if !upward_links.contains_key(&new_mperm) {
                    upward_links.insert(new_mperm.clone(), vec![current_mperm.clone()]);
                } else {
                    let links = upward_links.get_mut(&new_mperm).unwrap();
                    links.push(current_mperm.clone());
                }
            }
        }
    }
    return (downward_links, upward_links);
}
