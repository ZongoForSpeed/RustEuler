use crate::maths::digits::conversion;
use crate::register_problem;
use itertools::Itertools;
use rustsat::encodings::card::{encode_cardinality_constraint, totalizer::Totalizer};
use rustsat::instances::{BasicVarManager, Cnf, ManageVars};
use rustsat::solvers::Solve;
use rustsat::types::constraints::CardConstraint;
use rustsat::types::{Lit, TernaryVal};
use rustsat_minisat::core::Minisat;
use std::path::Path;

register_problem!(185, "Number Mind", problem185);

type Pair = (Vec<usize>, usize);

fn read_input(filepath: &str) -> Vec<Pair> {
    let path = Path::new(filepath);
    let lines: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut tries: Vec<Pair> = Vec::new();

    for line in lines {
        let (a, b, _) = line.split_whitespace().next_tuple().unwrap();
        // println!("{}, {}", a, b);
        let indexes = a
            .bytes()
            .map(|b| b - '0' as u8)
            .map(|b| b as usize)
            .collect::<Vec<usize>>();
        tries.push((indexes, b[1..].parse::<usize>().unwrap()));
    }
    tries
}

fn add_equal_constraint(var_manager: &mut BasicVarManager, mut cnf: &mut Cnf, literals: &Vec<Lit>, k: usize) {
    let eq_constr = CardConstraint::new_eq(literals.to_vec(), k);

    encode_cardinality_constraint::<Totalizer, Cnf>(eq_constr, &mut cnf, var_manager)
        .expect("cardinality encoding failed");
}

fn solve_number_mind(filepath: &str, length: usize) -> Option<String> {
    let tries = read_input(filepath);
    println!("{:?}", tries);

    // Declare variables
    let mut var_manager = BasicVarManager::default();
    let mut var = vec![vec![]; length];
    for p in 0..length {
        for _d in 0..10 {
            let v = var_manager.new_var();
            var[p].push(v.pos_lit());
        }
    }

    let mut cnf = Cnf::new();

    // Add constrains from input
    for (v, k) in tries {
        let literals = v
            .iter()
            .enumerate()
            .map(|(n, index)| var[n][*index])
            .collect();
        add_equal_constraint(&mut var_manager, &mut cnf, &literals, k);
    }

    // Add constrains on digits (a0 + a1 ... + a9 = 1)
    for p in 0..length {
        add_equal_constraint(&mut var_manager, &mut cnf, &var[p], 1);
    }

    let mut solver = Minisat::default();
    solver.add_cnf(cnf).expect("Panic message");

    let mut result = Vec::new();

    match solver.solve() {
        Ok(_) => {
            for p in 0..length {
                for d in 0..10 {
                    match solver.lit_val(var[p][d]) {
                        Ok(val) => {
                            if let TernaryVal::True = val {
                                println!("Position {}/{} : True", p, d);
                                result.push(d);
                            }
                        }
                        Err(error) => {
                            println!("ERROR ! Position {}/{} : {}", p, d, error);
                        }
                    }
                }
            }
        }
        Err(error) => {
            println!("ERROR : {}", error);
            return None;
        }
    }

    println!("{:?}", result);
    Some(conversion(&result, 10).to_string())
}

pub fn problem185() -> String {
    // The game Number Mind is a variant of the well known game Master Mind.
    //
    // Instead of coloured pegs, you have to guess a secret sequence of digits. After each guess
    // you're only told in how many places you've guessed the correct digit. So, if the sequence was
    // 1234 and you guessed 2036, you'd be told that you have one correct digit; however, you would
    // NOT be told that you also have another digit in the wrong place.
    //
    // For instance, given the following guesses for a 5-digit secret sequence,
    //
    //              90342 ;2 correct
    //              70794 ;0 correct
    //              39458 ;2 correct
    //              34109 ;1 correct
    //              51545 ;2 correct
    //              12531 ;1 correct
    //
    // The correct sequence 39542 is unique.
    //
    // Based on the following guesses,
    //
    //              5616185650518293 ;2 correct
    //              3847439647293047 ;1 correct
    //              5855462940810587 ;3 correct
    //              9742855507068353 ;3 correct
    //              4296849643607543 ;3 correct
    //              3174248439465858 ;1 correct
    //              4513559094146117 ;2 correct
    //              7890971548908067 ;3 correct
    //              8157356344118483 ;1 correct
    //              2615250744386899 ;2 correct
    //              8690095851526254 ;3 correct
    //              6375711915077050 ;1 correct
    //              6913859173121360 ;1 correct
    //              6442889055042768 ;2 correct
    //              2321386104303845 ;0 correct
    //              2326509471271448 ;2 correct
    //              5251583379644322 ;2 correct
    //              1748270476758276 ;3 correct
    //              4895722652190306 ;1 correct
    //              3041631117224635 ;3 correct
    //              1841236454324589 ;3 correct
    //              2659862637316867 ;2 correct
    //
    // Find the unique 16-digit secret sequence.
    if let Some(value) = solve_number_mind("data/p185_example.txt", 5) {
        println!("Example : {value}");
    }

    solve_number_mind("data/p185_number_mind.txt", 16).unwrap()
}
