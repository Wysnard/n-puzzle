use std::process;

#[derive(Debug)]
pub enum Strategy {
    Standard,
    Sandwich,
}

impl Strategy {
    pub fn parse(s: String) -> Strategy {
        match &s.to_lowercase() as &str {
            "standard" | "std" => Strategy::Standard,
            "sandwich" => Strategy::Sandwich,
            _ => {
                println!("Strategy not recognized");
                process::exit(1);
            }
        }
    }

    pub fn process(&self, current: &Vec<Vec<i64>>, goal: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        match self {
            Strategy::Standard => Self::process_std(goal),
            Strategy::Sandwich => Self::process_sandwich(current, goal),
        }
    }

    fn process_std(goal: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        goal
    }

    fn process_sandwich(current: &Vec<Vec<i64>>, goal: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        // TODO
        goal
    }
}