use rusted_advent_of_code as raoc;
use raoc::problem as problem;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
        problem::Problem{name: "day3_p1",
            sample_inputs: vec![problem::Example{sample_input: "problems/day3/sample_input", expected: "157"}],
            main_input: "problems/day3/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day3::parser), solver:  Box::new(day3::solverP1)}),
        },
        problem::Problem{name: "day3_p2",
            sample_inputs: vec![problem::Example{sample_input: "problems/day3/sample_input", expected: "70"}],
            main_input: "problems/day3/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day3::parser), solver:  Box::new(day3::solverP2)}),
        },
        problem::Problem{name: "day4_p1",
            sample_inputs: vec![problem::Example{sample_input: "problems/day4/sample_input", expected: "2"}],
            main_input: "problems/day4/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day4::parser), solver:  Box::new(day4::solverP1)}),
        },
        problem::Problem{name: "day4_p2",
            sample_inputs: vec![problem::Example{sample_input: "problems/day4/sample_input", expected: "4"}],
            main_input: "problems/day4/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day4::parser), solver:  Box::new(day4::solverP2)}),
        },
        problem::Problem{name: "day5_p1",
            sample_inputs: vec![problem::Example{sample_input: "problems/day5/sample_input", expected: "CMZ"}],
            main_input: "problems/day5/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day5::parser), solver:  Box::new(day5::solverP1)}),
        },
        problem::Problem{name: "day5_p2",
            sample_inputs: vec![problem::Example{sample_input: "problems/day5/sample_input", expected: "MCD"}],
            main_input: "problems/day5/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day5::parser), solver:  Box::new(day5::solverP2)}),
        },
        problem::Problem{name: "day6_p1",
            sample_inputs: vec![problem::Example{sample_input: "problems/day6/sample1", expected: "7"},
                                problem::Example{sample_input: "problems/day6/sample2", expected: "5"},
                                problem::Example{sample_input: "problems/day6/sample3", expected: "6"},
                                problem::Example{sample_input: "problems/day6/sample4", expected: "10"},
                                problem::Example{sample_input: "problems/day6/sample5", expected: "11"}],
            main_input: "problems/day6/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day6::parser), solver:  Box::new(day6::solverP1)}),
        },
        problem::Problem{name: "day6_p2",
            sample_inputs: vec![problem::Example{sample_input: "problems/day6/sample1", expected: "19"},
                                problem::Example{sample_input: "problems/day6/sample2", expected: "23"},
                                problem::Example{sample_input: "problems/day6/sample3", expected: "23"},
                                problem::Example{sample_input: "problems/day6/sample4", expected: "29"},
                                problem::Example{sample_input: "problems/day6/sample5", expected: "26"}],
            main_input: "problems/day6/input",
            runner: Box::new(problem::Runner{parser:  Box::new(day6::parser), solver:  Box::new(day6::solverP2)}),
        },

    ]};

    suit.run();
}