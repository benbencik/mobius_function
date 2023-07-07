use std::collections::HashMap;

struct Poset {
    nodes: HashMap<Vec<i32>, Vec<Vec<i32>>>, // HashMap to store nodes and their linked nodes
}

impl Poset {
    fn new() -> Self {
        Poset {
            nodes: HashMap::new(), // Initialize the Poset with an empty HashMap
        }
    }

    fn add_node(&mut self, value: Vec<i32>) {
        self.nodes.insert(value, Vec::new()); // Insert a new node with an empty vector of linked nodes
    }

    fn add_link(&mut self, from: Vec<i32>, to: Vec<i32>) {
        if let Some(links) = self.nodes.get_mut(&from) {
            links.push(to.clone()); // Add a link from 'from' node to 'to' node
        }
    }

    fn get_linked_nodes(&self, value: &Vec<i32>) -> Option<&Vec<Vec<i32>>> {
        self.nodes.get(value) // Get the linked nodes for a given value
    }

    fn contains_node(&self, value: &Vec<i32>) -> bool {
        self.nodes.contains_key(value) // Check if a node with the given value is present in the Poset
    }
}

fn main() {
    let mut poset = Poset::new();

    poset.add_node(vec![1, 2]);
    poset.add_node(vec![3, 4]);
    poset.add_node(vec![5, 6]);

    let v1 = vec![1, 2];
    let v2 = vec![3, 4];
    let v3 = vec![5, 7];
    println!("Contains [1, 2] {:?}", poset.contains_node(&v1));
    println!("Contains [3, 4] {:?}", poset.contains_node(&v2));
    println!("Contains [5, 7] {:?}", poset.contains_node(&v3));

    poset.add_link(vec![1, 2], vec![3, 4]);
    poset.add_link(vec![1, 2], vec![5, 6]);
    poset.add_link(vec![3, 4], vec![5, 6]);

    let linked_nodes = poset.get_linked_nodes(&vec![1, 2]);
    println!("Linked nodes for [1, 2]: {:?}", linked_nodes);

    let linked_nodes = poset.get_linked_nodes(&vec![3, 4]);
    println!("Linked nodes for [3, 4]: {:?}", linked_nodes);
}

