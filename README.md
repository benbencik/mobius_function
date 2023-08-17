# Effective Calculation of Möbius Function on Poset

## Running the project

To run this Rust project, make sure you have Rust and Cargo installed on your system. If not, you can follow steps on [the installation page](https://www.rust-lang.org/tools/install). After cloning the repository the executable can be run with `cargo run`. Similarly the tests can be run with `cargo test` and benchmarks with `cargo bench`.

## Definitions

**Partially ordered set** has a homogenous relation $\leq$ on set $P$ such that all of $a, b , c \in P$ must satisfy the following properties:

1. $a \leq a$ (reflexivity)
2. $a \leq b$ and $b \leq a$ implies $a = b$ (antisymmetry)
3. $a \leq b$ and $b \leq c$ implies $a \leq c$ (transitivity)

It is said that permutation $b$ **contains** permutation $a$: $[a_1, a_2 ... a_k] \leq [b_1, b_2 ... b_m] \iff \exists: 1 \leq i_1 < i_2 ... i_k \leq m$ such that $\forall x, y: a_x \leq a_y \iff b_{i_x} \leq b_{i_y}$. For example $[1,1,2] < [5,5,4,3,3,5]$ for $i_1 = 4, i_2 = 5, i_3 = 6$.

Given an interval $[a, b]$ in a poset $P$ the **Möbius Function** $\mu_p$ of this interval is recursively defined as $\mu_p(a, b)$:

- $0$ if $a \not\leq b$
- $1$ if $a = b$
- $\sum_{a \leq c < b} \mu_p(a, c)$

## Naive Implementation

Let's have two multipermutations $P, Q: P \leq Q$. In this approach the poset is build from top to bottom, starting with $Q$ and ending when encountering $P$. The program itteratively generates subpermutations. Checking if the new permutation is contained is done in the module `is_contained.rs`.
