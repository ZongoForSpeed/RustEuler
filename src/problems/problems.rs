use colored::Colorize;
use crate::maths::timer::ScopeTimer;
use inventory;

pub(crate) struct Problem {
    pub number: usize,
    pub name: &'static str,
    pub function: fn() -> String,
}

impl Problem {
    pub fn launch(&self, solution: &String) {
        /*
        if (resultat == solution) {
            std::cout << "\033[1;32m" << "Solution : " << resultat << "\033[0m" << std::endl;
        } else {
            std::cout << "\033[1;31m" << "ERREUR !" << std::endl;
            std::cout << "Résultat : " << resultat << std::endl;
            std::cout << "Solution : " << solution << std::endl;
            std::cout << "\033[0m";
        }
         */
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
            crate::problems::problems::Problem {
                number: $number,
                name: $name,
                function: $function,
            }
        }
    };
}
