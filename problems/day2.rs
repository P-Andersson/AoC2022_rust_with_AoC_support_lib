enum RockScissorsPapers {
    rock,
    paper,
    scissors,
}

enum Outcome {
    defeat,
    draw,
    victory,
}

impl RockScissorsPapers {
    fn outcome_against(&self, other: &RockScissorsPapers) -> Outcome {
        match self {
            RockScissorsPapers::rock => {
                return match other {
                    RockScissorsPapers::rock => Outcome::draw,
                    RockScissorsPapers::paper => Outcome::defeat,
                    RockScissorsPapers::scissors => Outcome::victory,
                }},
            RockScissorsPapers::paper => {
                return match other {
                    RockScissorsPapers::rock => Outcome::victory,
                    RockScissorsPapers::paper => Outcome::draw,
                    RockScissorsPapers::scissors => Outcome::defeat,
                }},
            RockScissorsPapers::scissors => {
                return match other {
                    RockScissorsPapers::rock => Outcome::defeat,
                    RockScissorsPapers::paper => Outcome::victory,
                    RockScissorsPapers::scissors => Outcome::draw,
                }},
        };
    }

    fn score(&self) -> i32 {
        return match self {
            RockScissorsPapers::rock => 1,
            RockScissorsPapers::paper => 2,
            RockScissorsPapers::scissors => 3,
        }
    }

    fn beaten_by(&self) -> RockScissorsPapers {
        return match self {
            RockScissorsPapers::rock => RockScissorsPapers::paper,
            RockScissorsPapers::paper => RockScissorsPapers::scissors,
            RockScissorsPapers::scissors => RockScissorsPapers::rock,
        }
    }

    fn beats(&self) -> RockScissorsPapers {
        return match self {
            RockScissorsPapers::rock => RockScissorsPapers::scissors,
            RockScissorsPapers::paper => RockScissorsPapers::rock,
            RockScissorsPapers::scissors => RockScissorsPapers::paper,
        }
    }

    fn draws_with(&self) -> RockScissorsPapers {
        return match self {
            RockScissorsPapers::rock => RockScissorsPapers::rock,
            RockScissorsPapers::paper => RockScissorsPapers::paper,
            RockScissorsPapers::scissors => RockScissorsPapers::scissors,
        }
    }
}

impl Outcome {
    fn score(&self) -> i32 {
        return match self {
            Outcome::defeat => 0,
            Outcome::draw => 3,
            Outcome::victory => 6,
        }
    }
}
pub struct StrategyGuide {
    rounds: Vec<(RockScissorsPapers, RockScissorsPapers)>,
}

pub fn parser(raw_input: &String) -> StrategyGuide {
    let mut rounds: Vec<(RockScissorsPapers, RockScissorsPapers)> = Vec::new();
    for line in raw_input.lines() {
        if line.len() >= 3 {
            let opponent_play = match line.chars().nth(0).unwrap() {
                'A' => RockScissorsPapers::rock,
                'B' => RockScissorsPapers::paper,
                'C' => RockScissorsPapers::scissors,
                _ => { panic!("Unparseable opponent strategy: {}", line.chars().nth(0).unwrap())},
            };
            let own_play = match line.chars().nth(2).unwrap() {
                'X' => RockScissorsPapers::rock,
                'Y' => RockScissorsPapers::paper,
                'Z' => RockScissorsPapers::scissors,
                _ => { panic!("Unparseable own strategy: {}", line.chars().nth(2).unwrap())},
            };
            rounds.push((opponent_play, own_play));
        }   
    }
    return StrategyGuide{rounds: rounds};
}

pub fn parserP2(raw_input: &String) -> StrategyGuide {
    let mut rounds: Vec<(RockScissorsPapers, RockScissorsPapers)> = Vec::new();
    for line in raw_input.lines() {
        if line.len() >= 3 {
            let opponent_play = match line.chars().nth(0).unwrap() {
                'A' => RockScissorsPapers::rock,
                'B' => RockScissorsPapers::paper,
                'C' => RockScissorsPapers::scissors,
                _ => { panic!("Unparseable opponent strategy: {}", line.chars().nth(0).unwrap())},
            };
            let own_play = match line.chars().nth(2).unwrap() {
                'X' => opponent_play.beats(),
                'Y' => opponent_play.draws_with(),
                'Z' => opponent_play.beaten_by(),
                _ => { panic!("Unparseable own strategy: {}", line.chars().nth(2).unwrap())},
            };
            rounds.push((opponent_play, own_play));
        }   
    }
    return StrategyGuide{rounds: rounds};
}

pub fn solver(input: StrategyGuide) -> String {
    let mut score: i32 = 0;
    for round in &input.rounds {
        let (opponent, own) = round;
        let result = own.outcome_against(&opponent);
        score += result.score() + own.score();
    }

    return score.to_string();
}
