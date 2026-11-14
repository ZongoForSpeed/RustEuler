# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Solutions to Project Euler problems in Rust (edition 2024), compiled into one binary crate (`rust_euler`). Big-integer work goes through GMP via `gmp-mpfr-sys`, so building needs a working C toolchain.

## Reference C++ implementation

This crate is a port of the C++20 solutions in `../ProjectEuler`, which has its own `CLAUDE.md`. Before solving or porting a problem, check for an existing solution there in `problemes/probleme0xx/` … `probleme3xx/`, or in `problemes/probleme<N>.cpp` for N ≥ 400. Identifiers on the C++ side are in French. These are the main equivalents:

| C++ (`../ProjectEuler`)            | Rust (here)                         |
|------------------------------------|-------------------------------------|
| `ENREGISTRER_PROBLEME(N, "...")`   | `register_problem!(N, "...", fn)`   |
| `utils/mpz_nombre`, `mpq_fraction` | `utils::mpz_number`, `mpq_fraction` |
| `maths/premiers` (`crible2`, …)    | `maths::primes`                     |
| `maths/arithmetique`               | `maths::arithmetic`                 |
| `maths/matrice`, `polynome`        | `maths::matrix`, `polynomial`       |
| `maths/graphe`, `pythagoricien`    | `maths::graph`, `pythagorean`       |

The two repos share the `data/` input files and the `data/solutions.txt` format.

## Commands

```sh
cargo build --release
cargo run --release                 # run every registered problem
cargo run --release -- 42 101       # run specific problems
cargo run --release -- 201-270      # run an inclusive range (ranges and single numbers can be mixed)
cargo test --release                # all unit tests
cargo test --release test_problem270   # a single problem's test
cargo test --release maths::primes     # tests in one module
```

Run from the repo root: `data/solutions.txt` and the problem input files are opened with relative paths like `data/p042_words.txt`. Use `--release`, because many solutions are too slow in debug builds.

## Architecture

- **Problem registration**: each problem file calls `register_problem!(number, "title", fn_name)` (defined in `src/problems/euler.rs`). The macro uses the `inventory` crate to submit a `Problem { number, name, function: fn() -> String }` at link time. `main.rs` collects every registered `Problem` into a `BTreeMap` keyed by number. There is no central list of problems to update.
- **Verification**: `main.rs` reads `data/solutions.txt` (lines of `<number> <answer>`, whitespace-separated). `Problem::launch` times the run with `ScopeTimer` and prints the result in green if it matches the expected answer or red if it does not. Every problem function returns its answer as a `String`.
- **Layout**: problems live in `src/problems/problems_FFF_LLL/problemNNN.rs`, where each group holds 100 consecutive problems (`problems_001_100`, `problems_101_200`, …) and the name gives its range. Problem names are zero-padded to three digits. Each group's `mod.rs` declares one `mod problemNNN;` line per file.
- **Shared code**:
  - `src/maths/`: number-theory and combinatorics helpers (primes, arithmetic, digits, power, polynomial, matrix, graph, pythagorean, fibonacci, etc.).
  - `src/utils/`: `MpzNumber` / `MpqFraction`, which are hand-written safe wrappers over raw GMP `mpz_t`/`mpq_t` with operator overloads; `ScopeTimer`; permutations; float epsilon helpers. Check these before writing new helpers.

## Existing helpers (check before writing new code)

Use these instead of re-implementing them. Trait methods need the trait in scope (`use crate::maths::arithmetic::Arithmetic;` etc.). Unless noted, integer traits cover all of `u8…u128, usize, i8…i128, isize`.

- **`maths::arithmetic::Arithmetic`** (`gcd`, `bezout`, `lcm`, `factorization`, `divisors`, `number_of_divisors`, `sum_of_divisors`, `phi`, `radical`, `mobius`, `square_factor`, `count_factor`, `repunit_a`). The prime-taking methods take a prime list, e.g. `n.phi(&primes)`. `factorization(primes, |p, exp| …)` yields `(prime, exponent)` pairs. `bezout(a, b)` returns `(x, y, gcd)`.
- **`maths::primes`**:
  - `Primes` trait on `u16, u32, u64, u128, usize`: `miller_rabin(reps)` and `next_prime(reps)` (smallest prime strictly greater than `self`). Use these for primality; don't write new Miller–Rabin or trial division.
  - `crible2`, `crible23`, `crible235` (`pub(crate)`, generic over the integer type, closure receives each prime). `crible235` emits primes up to the next multiple of 30 above the bound, so filter when the exact bound matters.
- **`maths::digits::Digits`**: `sum_digits`, `count_digits`, `extract_digits`, `loop_digits`, `palindrome`, `pandigital`, `reverse`, `count_digit`, `is_permutation(a, b, base)`, `concat_numbers(a, b, base)`, `ones`.
- **`maths::factorial::Factorial`**: `factorial(n)`, `binomial(n, p)`.
- **`maths::power`**: `Power` trait on `u8…usize`: `power(exp)`, `power_mod(exp, modulo)`. For `(a * b) % m` on native integers, use `power_mod` or widen to `u128` (the private `ModArith` trait is not callable from problems).
- **`maths::integer_root`**: `isqrt(n)`, `icbrt(n) -> bool` (whether `n` is a perfect cube).
- **`maths::polygonal::Polygonal`**: `is_square`, `get_sqrt`, `is_triangular/pentagonal/hexagonal/heptagonal/octagonal`, `is_polygonal(itype)`, `triangular/pentagonal/hexagonal/heptagonal/octagonal`, `polygonal(itype)`.
- **`maths::matrix`**: `power_matrix(a, exp)` and `power_matrix_mod(a, exp, modulo)` on `ndarray::Array2` (integer types and `f32`/`f64`).
- **`maths::fibonacci`**: `fibonacci::<N>()` (starts 0, 1) and `fibonacci_limit(limit)` (values < limit).
- **`maths::pythagorean`**: `pythagorean_limit(limit)` yields primitive triples `(a, b, c)` with `c < limit`. `pythagorean()` exists but is unused.
- **`maths::polynomial::Polynomial<T>`**: `new`, `from(&vec)`, `from_n`, `value(x)`, `reduce`, `len`, with `+ - *` operators.
- **`maths::graph`**: `a_star(start, end, graph, distance, heuristic)`, `Dijkstra::new(graph, start, end).algorithm()`, `kruskal(&edges)`.
- **`utils::mpz_number::MpzNumber`** (GMP big integers): arithmetic operators, `from_u64`/`from_str`, `factorial`, `binomial_ui`, `catalan`, `power_ui`, `power_mod_ui`, `gcd`, `sqrt`, `sum_digits`, `number_digits`, `palindrome`, `to_string`, `get_u128`.
- **`utils::mpq_fraction::MpqFraction`**: exact rationals (GMP).
- **`utils::permutations::permutations(v)`**, **`utils::epsilon::{equal_epsilon, compare_epsilon}`** (f64 comparison), **`utils::timer::ScopeTimer`**.

## Adding a problem

1. Create `src/problems/problems_FFF_LLL/problemNNN.rs` (the group whose range contains NNN) that:
   - calls `register_problem!(NNN, "Title", problemNNN)`
   - puts the problem statement in a `///` doc comment on `pub fn problemNNN() -> String`
   - ends with a `#[cfg(test)] mod tests` containing `test_problemNNN`, which asserts the answer
2. Add `mod problemNNN;` to that group's `mod.rs`. If this starts a new group of 100, also add the group module to `src/problems/mod.rs`.
3. Add `NNN<TAB>answer` to `data/solutions.txt`.
4. Put any input file in `data/` as `pNNN_<name>.txt`.
