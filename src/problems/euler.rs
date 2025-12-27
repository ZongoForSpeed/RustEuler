use crate::maths::timer::ScopeTimer;
use colored::Colorize;
use inventory;

pub(crate) struct Problem {
    pub number: usize,
    pub name: &'static str,
    pub function: fn() -> String,
}

impl Problem {
    pub fn launch(&self, solution: &String) {
        let label = format!("Problem {} {}", self.number, self.name);
        let result = {
            let _timer = ScopeTimer::new(label.as_str(), false);
            (self.function)()
        };
        if solution.eq(&result) {
            let format = format!("Solution {}: {}", self.number, result);
            println!("{}", format.green().bold());
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
