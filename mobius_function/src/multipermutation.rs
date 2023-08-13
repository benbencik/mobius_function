use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
#[derive(Clone, Hash)]
pub struct Mperm {
    value: Vec<u8>,
    max_elem: u8,
}

impl Mperm {
    pub fn new(value: Vec<u8>, max_elem: u8) -> Mperm {
        let mperm = Mperm { value, max_elem };
        // mperm.check_validity(); // ! later will be removed
        return mperm;
    }

    fn check_validity(&self) {
        // multipermutation should contain each element from range [1, max_elem]
        assert!(self.value.len() > self.max_elem as usize);
        for i in 1..=self.max_elem {
            assert!(self.value.contains(&i));
        }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn get_value(&self) -> &Vec<u8> {
        return &self.value;
    }

    fn contains_at_least_two(&self, elem: u8) -> bool {
        let mut count = 0;
        for i in 0..self.len() {
            if self.value[i] == elem {
                count += 1;
            }
        }
        return count >= 2;
    }

    pub fn gen_submperms(&self) -> Vec<Mperm> {
        let mut submperms: HashSet<Mperm> = HashSet::new();

        for i in 0..self.len() {
            let mut subperm: Vec<u8>;

            if self.contains_at_least_two(self.value[i]) {
                subperm = self.value.clone();
                subperm.remove(i);
                submperms.insert(Mperm::new(subperm, self.max_elem));
            } else {
                subperm = Vec::new();
                for j in 0..self.len() {
                    if j != i {
                        if self.value[j] > self.value[i] {
                            subperm.push(self.value[j] - 1);
                        } else {
                            subperm.push(self.value[j]);
                        }
                    }
                }
                submperms.insert(Mperm::new(subperm, self.max_elem - 1));
            }
        }
        return submperms.into_iter().collect();
    }
}

impl PartialEq for Mperm {
    fn eq(&self, other: &Mperm) -> bool {
        return self.value == other.value;
    }
}

impl Eq for Mperm {}

impl fmt::Debug for Mperm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonrepeating_elements() {
        let mperm: Mperm = Mperm::new(vec![1, 2, 3, 4, 5], 5);
        let expected: Vec<Mperm> = vec![Mperm::new(vec![1, 2, 3, 4], 4)];
        assert!(mperm.gen_submperms().iter().all(|item| expected.contains(item)));
    }

    #[test]
    fn repeating_elements() {
        let mperm: Mperm = Mperm::new(vec![1, 2, 3, 3, 4, 5], 5);
        let expected: Vec<Mperm> = vec![
            Mperm::new(vec![1, 2, 2, 3, 4], 4),
            Mperm::new(vec![1, 2, 3, 4, 5], 5),
            Mperm::new(vec![1, 2, 3, 3, 4], 4),
        ];
        assert!(mperm.gen_submperms().iter().all(|item| expected.contains(item)));
    }
}
