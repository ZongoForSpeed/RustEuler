use crate::problems::euler::Problem;
use colored::Colorize;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::env;
use std::path::Path;

pub mod maths;
pub mod problems;
pub mod utils;

fn main() {
    let solutions = read_solutions();

    let register: BTreeMap<usize, &Problem> = inventory::iter::<Problem>
        .into_iter()
        .map(|p| (p.number, p))
        .collect();

    let problems = read_args();
    println!("Running problems: {:?}", problems);

    if problems.is_empty() {
        for (n, problem) in register {
            let solution = solutions.get(&n).cloned().unwrap_or("".to_string());
            problem.launch(&solution);
        }
    } else {
        for p in problems {
            match register.get(&p) {
                None => {
                    let format = format!("Cannot find problem {}", p);
                    println!("{}", format.red().bold());
                }
                Some(problem) => {
                    let solution = solutions.get(&p).cloned().unwrap_or("".to_string());
                    problem.launch(&solution);
                }
            }
        }
    }
}

fn read_args() -> Vec<usize> {
    let mut problems: Vec<usize> = Vec::new();
    for argument in env::args().skip(1) {
        if argument.contains('-') {
            let (left, right): (usize, usize) = argument
                .split('-')
                .map(|p| p.parse().unwrap())
                .next_tuple()
                .unwrap();
            problems.extend(left..=right);
        } else {
            problems.push(argument.parse::<usize>().unwrap());
        }
    }

    problems.sort();
    problems
}

fn read_solutions() -> BTreeMap<usize, String> {
    let path = Path::new("data/solutions.txt");
    let solutions: BTreeMap<usize, String> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::to_string)
        .filter_map(|line| read_line(line))
        .collect();
    solutions
}

fn read_line(line: String) -> Option<(usize, String)> {
    let (left, right) = line.split_whitespace().next_tuple()?;
    Some((left.parse().ok()?, right.to_string()))
}
