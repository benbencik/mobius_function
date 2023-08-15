use crate::multipermutation::Mperm;
use std::collections::{HashMap, HashSet, LinkedList};

fn dfs_sum(
    current: &Mperm,
    upward_links: &HashMap<Mperm, Vec<Mperm>>,
    values: &HashMap<Mperm, i32>,
    visited: &mut HashSet<Mperm>,
) -> i32 {
    visited.insert(current.clone());
    let mut sum = 0;
    let links = upward_links.get(current);
    match links {
        Some(x) => {
            for link in x {
                if !visited.contains(link) {
                    sum += dfs_sum(link, upward_links, values, visited);
                }
            }
        }
        None => {}
    }
    if values.get(current).is_some() {
        sum += values.get(current).unwrap();
    }
    return sum;
}

pub fn naive(
    poset_bottom: &Mperm,
    poset_top: &Mperm,
    downward_links: &HashMap<Mperm, Vec<Mperm>>,
    upward_links: &HashMap<Mperm, Vec<Mperm>>,
) -> i32 {
    let mut queue: LinkedList<Mperm> = LinkedList::new();
    let mut values: HashMap<Mperm, i32> = HashMap::new();
    queue.push_back(poset_top.clone());

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();

        // node was already visited
        if values.contains_key(&current) {
            continue;
        }

        // calculate value of the node
        let val: i32;
        if current == *poset_top {
            val = 1;
        } else {
            let mut visited: HashSet<Mperm> = HashSet::new();
            val = -dfs_sum(&current, upward_links, &values, &mut visited);
        }
        values.insert(current.clone(), val);

        // poset bottom reached, do not continue
        if &current == poset_bottom {
            return val;
        }

        // add new nodes to queue
        let links = downward_links.get(&current).unwrap();
        for link in links {
            queue.push_back(link.clone());
        }
    }

    return values.get(poset_bottom).unwrap().clone();
}
