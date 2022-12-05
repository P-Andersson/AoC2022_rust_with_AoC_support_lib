use regex::Regex;

type CrateStack = Vec<char>;
type Stacks = Vec<CrateStack>;

pub struct Action {
    source: usize, 
    dest: usize,
    count: usize,
}
type Actions = Vec<Action>;

impl Action {
    fn to_string(&self) -> String{
        return format!("move {} from {} to {}", self.count, self.source + 1, self.dest + 1);
    }
}

fn to_string(stacks: &Stacks) -> String {
    let mut tallest = 0;
    for stack in stacks {
        tallest = tallest.max(stack.len());
    }

    let mut result = String::new();
    for inverted_y in 0..tallest {
        let y = tallest - inverted_y - 1;
        let mut line = String::new();
        for stack in stacks {
            if y < stack.len() {
                line += &format!("[{}]", stack[y]);
            } else 
            {
                line += "   ";
            } 
        }
        result += &(line + "\n");
    }
    return result;
}


pub fn parser(raw_input: &String) -> (Stacks, Actions) {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut stacks = Stacks::new();
    let mut instructions  = Actions::new();

    let mut parsing_moves = false;
    for line in raw_input.lines() {
        if line.len() == 0 {
            parsing_moves = true;
        }
        else if !parsing_moves {
            for (i, c) in line.chars().enumerate() {
                if c != ' ' && c != '[' && c != ']' {
                    let stack_no = i / 4;
                    while stacks.len() < stack_no + 1 {
                        stacks.push(CrateStack::new());
                    }
                    stacks[stack_no].insert(0, c);
                } 
            }
        } else {
            for cap in move_re.captures_iter(line) {
                instructions.push(Action{source: cap[2].parse::<usize>().unwrap() - 1,
                                         dest: cap[3].parse::<usize>().unwrap() - 1,
                                         count: cap[1].parse::<usize>().unwrap()});
            }
        }
    }
    return (stacks, instructions);
}

fn apply_action(action: &Action, stacks: &mut Stacks) {
    let mut dest = stacks[action.dest].clone();
    let mut src = stacks[action.source].clone();
    for index_inv in (0..action.count) {
        let index = src.len() - index_inv - 1;
        dest.push(src[index]);
    }
    src.resize(src.len() - action.count, '0');
    stacks[action.dest] = dest;
    stacks[action.source] = src;
}

pub fn solverP1(input: (Stacks, Actions)) -> String {
    let (mut stacks, instructions) = input;
    for action in instructions {
        apply_action(&action, &mut stacks);
    }

    let mut message = String::new();
    for stack in stacks {
        if stack.len() > 1 {
            message += &stack.last().unwrap().to_string();
        } else {
            message += " ";
        }
    }

    return message;
}

fn apply_action_crate_mover_9001(action: &Action, stacks: &mut Stacks) {
    let mut dest = stacks[action.dest].clone();
    let mut src = stacks[action.source].clone();
    for index_inv in (0..action.count) {
        let index = src.len() - action.count + index_inv;
        dest.push(src[index]);
    }
    src.resize(src.len() - action.count, '0');
    stacks[action.dest] = dest;
    stacks[action.source] = src;
}


pub fn solverP2(input: (Stacks, Actions)) -> String {
    let (mut stacks, instructions) = input;
    println!("{}", to_string(&stacks));
    for action in instructions {
        println!("{}", action.to_string()); 
        apply_action_crate_mover_9001(&action, &mut stacks);
        println!("{}", to_string(&stacks));
    }

    let mut message = String::new();
    for stack in stacks {
        if stack.len() > 1 {
            message += &stack.last().unwrap().to_string();
        } else {
            message += " ";
        }
    }

    return message;
}

