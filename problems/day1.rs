pub struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn total_calories(&self) -> i32 {
        return self.calories.iter().sum();
    }
}

pub fn parser(raw_input: &String) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut calories: Vec<i32> = Vec::new();
    for line in raw_input.lines() {
        if line.len() == 0 {
            elves.push(Elf{calories: calories});
            calories = Vec::new();
        } else {
            calories.push(line.parse::<i32>().unwrap())
        }   
    }
    elves.push(Elf{calories: calories});
    return elves;
}

pub fn solverP1(input: Vec<Elf>) -> String {
    let mut highest: i32 = 0;
    for elf in input {
        let cur = elf.total_calories();
        if highest < cur {
            highest = cur;
        }
    }

    return highest.to_string();
}

pub fn solverP2(input: Vec<Elf>) -> String {
    let mut calories_total: Vec<i32> = input.into_iter().map(|elf| elf.total_calories()).collect();
    calories_total.sort();

    let mut highest_three: i32 = 0;

    highest_three += calories_total[calories_total.len()-1];
    highest_three += calories_total[calories_total.len()-2];
    highest_three += calories_total[calories_total.len()-3];

    return highest_three.to_string();
}