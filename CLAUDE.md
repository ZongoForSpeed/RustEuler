# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Solutions to Project Euler problems in Rust (edition 2024), compiled into one binary crate (`rust_euler`). Big-integer work goes through GMP via `gmp-mpfr-sys`, so building needs a working C toolchain.

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
- **Layout**: problems live in `src/problems/problems_Nxx/problemNNN.rs`, grouped by hundreds, with zero-padded three-digit names. Each group's `mod.rs` declares one `mod problemNNN;` line per file.
- **Shared code**:
  - `src/maths/`: number-theory and combinatorics helpers (primes, arithmetic, digits, power, polynomial, matrix, graph, pythagorean, fibonacci, etc.).
  - `src/utils/`: `MpzNumber` / `MpqFraction`, which are hand-written safe wrappers over raw GMP `mpz_t`/`mpq_t` with operator overloads; `ScopeTimer`; permutations; float epsilon helpers. Check these before writing new helpers.

## Adding a problem

1. Create `src/problems/problems_Nxx/problemNNN.rs` that:
   - calls `register_problem!(NNN, "Title", problemNNN)`
   - puts the problem statement in a `///` doc comment on `pub fn problemNNN() -> String`
   - ends with a `#[cfg(test)] mod tests` containing `test_problemNNN`, which asserts the answer
2. Add `mod problemNNN;` to that group's `mod.rs`. If this starts a new hundred, also add the group module to `src/problems/mod.rs`.
3. Add `NNN<TAB>answer` to `data/solutions.txt`.
4. Put any input file in `data/` as `pNNN_<name>.txt`.
