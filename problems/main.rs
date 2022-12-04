use rusted_advent_of_code as raoc;
use raoc::problem as problem;

mod day1;
mod day2;

fn main() {
    let suit = raoc::suit::Suit{problems: vec![
        problem::Problem{name: "day1_p1",
                        sample_inputs: vec![problem::Example{sample_input: "problems/day1/sample_input", expected: "24000"}],
                        main_input: "problems/day1/input",
                        runner: Box::new(problem::Runner{parser:  Box::new(day1::parser), solver:  Box::new(day1::solverP1)}),
        },
        problem::Problem{name: "day1_p2",
            sample_inputs: vec![problem::Example{sample_input: "problems/day1/sample_input", expected: "45000"}],
            main_input: "problems/day1/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day1::parser), solver:  Box::new(day1::solverP2)}),
        },
        problem::Problem{name: "day2_p1",
            sample_inputs: vec![problem::Example{sample_input: "problems/day2/sample_input", expected: "15"}],
            main_input: "problems/day2/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day2::parser), solver:  Box::new(day2::solver)}),
        },
        problem::Problem{name: "day2_p2",
            sample_inputs: vec![problem::Example{sample_input: "problems/day2/sample_input", expected: "12"}],
            main_input: "problems/day2/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day2::parserP2), solver:  Box::new(day2::solver)}),
        },
    ]};

    suit.run();
}