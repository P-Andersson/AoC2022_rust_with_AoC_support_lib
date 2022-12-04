use std::collections::HashSet;

type Compartment = HashSet<char>;
    
pub struct Rucksack {
    left: Compartment,
    right: Compartment,
}

impl Rucksack {
    fn add(&mut self, items: Vec<char>) {
        for (i, &item) in items.iter().enumerate() {
            if i < items.len()/2 {
                self.left.insert(item);
            } else {
                self.right.insert(item);
            }
        }
    }

    fn get_mispacked(&self) -> Compartment {
        let mut new: Compartment = Compartment::new();
        for item in self.left.intersection(&self.right) {
            new.insert(item.clone());
        }
        return new;
    }

    fn all_item_types(&self) -> Compartment {
        let mut new: Compartment = Compartment::new();
        for item in self.left.union(&self.right) {
            new.insert(item.clone());
        }
        return new;
    }
}

fn priority(item: char) -> i32 {
    let priority: i32;
    if item.is_uppercase() {
        priority = (item as u32 - 'A' as u32) as i32 + 27;
    } else {
        priority = (item as u32 - 'a' as u32) as i32 + 1;
    }
    return priority; 
}

fn to_string(items: &Compartment) -> String {
    let mut out = String::new();
    for item in items {
        out += &item.to_string();
    }
    return out;
}

pub fn parser(raw_input: &String) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    for line in raw_input.lines() {
        let mut rucksack = Rucksack{left: Compartment::new(), right: Compartment::new()};
        rucksack.add(line.chars().collect());
        rucksacks.push(rucksack);
    }
    return rucksacks;
}

pub fn solverP1(input: Vec<Rucksack>) -> String {
    let mut score: i32 = 0;
    for rucksack in input {
        let mispacked = rucksack.get_mispacked();
        let rep = to_string(&mispacked);
        for item in mispacked {
            score += priority(item);
        }
    }

    return score.to_string();
}

fn group_rucksacks(sacks: Vec<Rucksack>, group_size: u32) -> Vec<Vec<Rucksack>> {
    let mut groups: Vec<Vec<Rucksack>> = Vec::new();
    
    let mut group_index = 0;
    let mut group: Vec<Rucksack> = Vec::new();
    for sack in sacks {
        group.push(sack);
        group_index += 1;
        if group_index >= group_size {
            groups.push(group);
            group_index = 0;
            group = Vec::new();
        }
    }
    return groups;
}

pub fn solverP2(input: Vec<Rucksack>) -> String {
    let groups = group_rucksacks(input, 3);

    let mut score: i32 = 0;
    for group in groups {
        let mut all_common: Compartment = Compartment::new();
        for sack in group {
            if all_common.len() == 0 {
                all_common = sack.all_item_types();
            } else {
                let mut new: Compartment = Compartment::new();
                for item in all_common.intersection(&sack.all_item_types()) {
                    new.insert(item.clone());
                }
                all_common = new;
            }
        }
        for item in all_common {
            score += priority(item);
        }
    }

    return score.to_string();
}
