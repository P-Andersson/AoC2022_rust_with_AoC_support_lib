use regex::Regex;

type Range = (i32, i32);
type ElfPair = (Range, Range);

fn to_string(pair: &ElfPair) -> String {
    let ((f1, f2), (s1, s2)) = pair;
    return format!("{}-{}, {}-{}", f1, f2, s1, s2);
}

fn is_inside(pair: &ElfPair) -> bool
{
    let ((l1, r1), (l2, r2)) = pair;

    return (l1 >= l2 && l1 <= r2 && r1 >= l2 && r1 <= r2) || 
           (l2 >= l1 && l2 <= r1 && r2 >= l1 && r2 <= r1);
}

fn partially_overlaps(pair: &ElfPair) -> bool
{
    let ((l1, r1), (l2, r2)) = pair;

    return ((l1 >= l2 && l1 <= r2) || (r1 >= l2 && r1 <= r2)) ||
           ((l2 >= l1 && l2 <= r1) || (r2 >= l1 && r2 <= r1));
}



pub fn parser(raw_input: &String) -> Vec<ElfPair> {
    let pair_re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut pairs: Vec<ElfPair> = Vec::new();
    for line in raw_input.lines() {
        for cap in pair_re.captures_iter(line) {
            pairs.push(((cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()), 
                        (cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap())));
        }
    }
    return pairs;
}

pub fn solverP1(input: Vec<ElfPair>) -> String {
    let mut score: i32 = 0;
    for pair in input {
        if is_inside(&pair) {
            score += 1;
        }
    }

    return score.to_string();
}

pub fn solverP2(input: Vec<ElfPair>) -> String {
    let mut score: i32 = 0;
    for pair in input {
        if partially_overlaps(&pair) {
            score += 1;
        }
    }

    return score.to_string();
}
