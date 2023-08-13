use crate::multipermutation::Mperm;
use std::collections::{HashMap, LinkedList, HashSet};

fn dfs(
    current: &Mperm,
    poset_top: &Mperm,
    upward_links: &HashMap<Mperm, Vec<Mperm>>,
    values: &HashMap<Mperm, i32>,
    visited: &mut HashSet<Mperm>,
) -> i32 {
    visited.insert(current.clone());
    if current == poset_top {
        return 1;
    } else {
        let mut sum = 0;
        let links = upward_links.get(current).unwrap();
        for link in links {
            if !visited.contains(link) {
                sum += dfs(link, poset_top, upward_links, values, visited);
            }
        }
        return -sum;
    }
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
        let mut visited: HashSet<Mperm> = HashSet::new();
        let val = dfs(&current, poset_top, upward_links, &values, &mut visited);
        values.insert(current.clone(), val);

        if &current == poset_bottom {
            return val;
        }
        let links = downward_links.get(&current).unwrap();
        for link in links {
            queue.push_back(link.clone());
        }
    }

    return values.get(poset_bottom).unwrap().clone();
}
