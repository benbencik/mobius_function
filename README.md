

# Effective Calculation of Möbius Function on Poset

## Running the project

To run this Rust project, make sure you have Rust and Cargo installed on your system. If not, you can follow the steps on [the installation page](https://www.rust-lang.org/tools/install). After cloning the repository the executable can be run with `cargo run`. Similarly, the tests can be run with `cargo test` and benchmarks with `cargo bench`.

## Definitions

### Partially Ordered Set

A **partially ordered set (poset)** has a homogeneous relation $\leq$ on set $P$ such that all $a, b, c \in P$ must satisfy the following properties:

1. $a \leq a$ (reflexivity)
2. $a \leq b$ and $b \leq a$ implies $a = b$ (antisymmetry)
3. $a \leq b$ and $b \leq c$ implies $a \leq c$ (transitivity)

Permutation $b$ **contains** permutation $a$ ($[a_1, a_2, \ldots, a_k] \leq [b_1, b_2, \ldots, b_m]$) if there exists a subsequence in $b$ that is order-isomorphic to $a$. The poset provides the necessary order relations between elements, which are crucial for determining the intervals and their properties.

### Möbius Function on Permutations

Given an interval $[a, b]$ in a poset $P$, the **Möbius function** $\mu_p$ of this interval is recursively defined as:

- $\mu_p(a, b) = 0$ if $a \not\leq b$
- $\mu_p(a, b) = 1$ if $a = b$
- $\mu_p(a, b) = -\sum_{a \leq c < b} \mu_p(a, c)$ otherwise

The Möbius function is useful for studying permutation patterns. However, its computation is demanding due to its recursive nature. 

### Posets and Permutations

In the context of permutations, posets represent the ordering relationships between different permutations. This allows for the analysis of permutation patterns and the study of interval structures within the poset. The Möbius function on these intervals provides valuable information about the poset's topology and combinatorial properties.

### Multipermutations
A multipermutation is a generalization of a permutation where elements can repeat. For example, $[1, 2, 2, 3]$ is a multipermutation of the set $\{1, 2, 3\}$. This project computes the mobius function on multipermutations. Note that permutations are a special case of multipermutations where no elements repeat.

## Implementation Description

### Computation Process

1. **Input Data**: The computation starts with two multipermutations $P$ and $Q$, where $P \leq Q$. These multipermutations serve as the bounds for the interval in the poset.

2. **Building the Poset**: The poset is constructed from top to bottom. Starting with $Q$, the program iteratively generates all subposets down to $P$, generating all permutations between $P$ and $Q$.
   - While the queue is not empty:
     - Remove the first element from the queue.
     - If this element is equal to the bottom element of the poset, skip to the next iteration.
     - If this element is not already in the downward links, generate its sub-multipermutations.
     - For each sub-multipermutation:
       - If the sub-multipermutation is larger or equal in length to the bottom element, add it to the queue.
       - Update the downward and upward links to include the relationships between the current element and the sub-multipermutation.
      
3. **Möbius Function Calculation**: The Möbius function $\mu_p(a, b)$ for the interval $[a, b]$ in the poset is calculated recursively:
    - If $a \not\leq b$, $\mu_p(a, b) = 0$.
    - If $a = b$, $\mu_p(a, b) = 1$.
    - Otherwise, $\mu_p(a, b) = -\sum_{a \leq c < b} \mu_p(a, c)$.

4. **Memoization**: Memoization is used to optimize the recursive computation. This involves caching the results of previously computed Möbius function values to avoid redundant calculations.

### Implementation Details

The implementation is divided into several key components:

1. **Data Structures**:
    - **Poset**: Represented using a custom `Poset` struct, which holds the elements and their order relations.
    - **Memoization Cache**: A `HashMap` is used to store previously computed Möbius function values for quick lookup.

2. **Recursive Function**:
    - The core of the implementation is a recursive function `calculate_mobius`, which computes the Möbius function value for a given interval in the poset. It checks the base cases and then recursively sums the results for all intermediate elements.

3. **Error Handling**:
    - Rust’s error-handling mechanisms are utilized to ensure that invalid inputs are gracefully handled. This includes checking for invalid permutations and ensuring the input poset is well-formed.

4. **Testing**:
    - The implementation includes a suite of tests to verify the correctness of the Möbius function calculations. These tests cover various edge cases and ensure compliance with the mathematical definition.

## Example Usage

```rust
fn main() {
    let bottom: Mperm = Mperm::new(vec![1]);
    let top: Mperm = Mperm::new(vec![1, 2, 3, 5, 4]);

    let (downward_links, upward_links) = building_poset::build_poset(&bottom, &top);
    let result = mobius_func::naive(&bottom, &top, &downward_links, &upward_links);
    println!("Result of μ({:?}, {:?}) is: {}", bottom, top, result);
}
```
