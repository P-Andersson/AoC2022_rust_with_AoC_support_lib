use std::fs;

pub trait RunnerTrait {
    fn run(&self, input: &String) -> String;
}

pub struct Runner<InputT> {
    pub parser: Box<dyn Fn(&String) -> InputT>,
    pub solver: Box<dyn Fn(&InputT) -> String>,
}

impl<InputT> RunnerTrait for Runner<InputT> {
    fn run(&self, input: &String) -> String {
        return (&self.solver)(&(self.parser)(input));
    }
}

fn load_input_contents(input_file: &str) -> String {
    let result = fs::read_to_string(input_file);
    return match result {
        Ok(v) => v,
        Err(e) => {
            panic!("Failure reading input file, reason: {e:?}");
        },
    }
    
}

pub struct Example {
    pub sample_input:  &'static str,
    pub expected:  &'static str,
}

impl Example {
    fn run(&self, runner: &dyn RunnerTrait) -> (bool, String) {
        let contents = load_input_contents(&self.sample_input);
        let result = runner.run(&contents);
        if result == self.expected {
            return (true, result + " == " + &self.expected);
        }
        return (false, result + " != " + &self.expected);
    }
}

pub struct Problem {
    pub name: &'static str,
    pub sample_inputs: Vec<Example>,
    pub main_input:  &'static str,
    pub runner: Box<dyn RunnerTrait>,
}

impl Problem {
    pub fn run_samples(&self) -> (bool, String) {
        let mut result = true;
        let mut aggregate_output = String::new();
        for sample in &self.sample_inputs {
            let (ok, output_desc) = sample.run(&*self.runner);
            if !ok {
                result = false;
            } 
            aggregate_output += &(output_desc + "\n");
        }
        return (result, aggregate_output);
    }
    
    pub fn run(&self) -> String {
        let contents = load_input_contents(&self.main_input);
        return self.runner.run(&contents);
    }
}


