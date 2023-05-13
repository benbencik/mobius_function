# Effective Calculation of Möbius Function on Poset

## 1. Introduction
**Partially ordered set** has a homogenous relation $\leq$ on set $P$ such that all of $a, b , c \in P$ must satisfy the following properties:
1. $a \leq a$ (reflexivity)
2. $a \leq b$ and $b \leq a$ implies $a = b$ (antisymmetry)
3. $a \leq b$ and $b \leq c$ implies $a \leq c$ (transitivity)

The **Möbius Function** of a poset is a function $\mu: P \times P \rightarrow \mathbb{Z}$ such that:
1. $a = c$: 1
2. $a \leq c$: $-\sum_{a \leq b < c} \mu(a, c)$
