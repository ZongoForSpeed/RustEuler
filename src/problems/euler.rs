use crate::utils::timer::ScopeTimer;
use colored::Colorize;
use inventory;
use std::time::Duration;

/// Au-delà de ce temps, une solution correcte est affichée en jaune.
const SEUIL_LENT: Duration = Duration::from_secs(20);

pub(crate) struct Problem {
    pub number: usize,
    pub name: &'static str,
    pub function: fn() -> String,
}

impl Problem {
    pub fn launch(&self, solution: &String) {
        let label = format!("Problem {} {}", self.number, self.name);
        let (result, elapsed) = {
            let timer = ScopeTimer::new(label.as_str(), false);
            let result = (self.function)();
            (result, timer.now.elapsed())
        };
        if solution.eq(&result) {
            let format = format!("Solution {}: {}", self.number, result);
            if elapsed > SEUIL_LENT {
                println!("{}", format.yellow().bold());
            } else {
                println!("{}", format.green().bold());
            }
        } else {
            let format = format!("ERROR Solution {}: {} != {}", self.number, solution, result);
            println!("{}", format.red().bold());
        }
    }
}

inventory::collect!(Problem);

#[macro_export]
macro_rules! register_problem {
    ($number:expr, $name:expr, $function:ident) => {
        inventory::submit! {
            crate::problems::euler::Problem {
                number: $number,
                name: $name,
                function: $function,
            }
        }
    };
}
