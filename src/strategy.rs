use super::heuristique::*;
use super::utils::*;
use std::process;

#[derive(Debug)]
pub enum Strategy {
    Standard(Heuristique),
    Sandwich(Vec<Vec<f64>>, Heuristique),
}

impl Strategy {
    pub fn parse(s: String, heuristique: Heuristique) -> Strategy {
        match &s.to_lowercase() as &str {
            "standard" | "std" => Strategy::Standard(heuristique),
            "sandwich" => Strategy::Sandwich(vec![], heuristique),
            _ => {
                println!("Strategy not recognized");
                process::exit(1);
            }
        }
    }

    /*
    	*	Init the Strategy
    	*/
    pub fn init(&mut self, goal: &Vec<Vec<i64>>) {
        match self {
            Strategy::Standard(_) => {}
            Strategy::Sandwich(weight, _) => Self::init_sandwich(weight, goal),
        };
    }

    /*
    	*	Init the sandwich strategy
    	*/
    fn init_sandwich(weight: &mut Vec<Vec<f64>>, goal: &Vec<Vec<i64>>) {
        *weight = vec![];
    }

    pub fn process(&self, current: &Vec<Vec<i64>>, goal: &Vec<Vec<i64>>) -> f64 {
        match self {
            Strategy::Standard(heuristique) => Self::process_std(heuristique, current, goal),
            Strategy::Sandwich(weight, heuristique) => {
                Self::process_sandwich(weight, heuristique, current, goal)
            }
        }
    }

    fn process_std(
        heuristique: &Heuristique,
        current: &Vec<Vec<i64>>,
        goal: &Vec<Vec<i64>>,
    ) -> f64 {
        heuristique.process_h(current, &goal)
    }

    fn process_sandwich(
        weight: &Vec<Vec<f64>>,
        heuristique: &Heuristique,
        current: &Vec<Vec<i64>>,
        goal: &Vec<Vec<i64>>,
    ) -> f64 {
        // TODO: Weight * heuristique
        Self::process_std(heuristique, current, goal)
    }
}
