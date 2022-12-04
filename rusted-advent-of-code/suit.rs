use std::env;

use super::problem as problem;

pub struct Suit {
    pub problems: Vec<problem::Problem>,
}

impl Suit {
    pub fn run(&self) {
        let problem = if let Some(d) = self.problems.last() { d } else { panic!("No defined problems"); };
        let args: Vec<_> = env::args().collect();
        if args.contains(&String::from("--example")) {
            let (ok, output) = problem.run_samples();
            println!("{} {}", if ok {"Exmaple: Ok -- "} else {"Example: NOK -- "}, output);
        } else {
            println!("Output: {}", problem.run());
        }
    }
}
