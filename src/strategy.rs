use super::heuristique::*;
use super::utils::*;
use std::process;

#[derive(Debug)]
pub enum Strategy {
    Standard(Heuristique),
    Sandwich(Vec<Vec<f64>>, Heuristique),
    Cross(Vec<Vec<f64>>, Heuristique),
}

impl Strategy {
    pub fn parse(s: String, heuristique: Heuristique) -> Strategy {
        match &s.to_lowercase() as &str {
            "standard" | "std" => Strategy::Standard(heuristique),
            "sandwich" => Strategy::Sandwich(vec![], heuristique),
            "cross" => Strategy::Cross(vec![], heuristique),
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
            Strategy::Cross(weight, _) => Self::init_cross(weight, goal),
        };
    }

    /*
     *	Init the sandwich strategy
     */
    fn init_sandwich(weight: &mut Vec<Vec<f64>>, goal: &Vec<Vec<i64>>) {
        *weight = Vec::new();
        let (x, y) = find_nb(0, goal);
        for i in 0..goal.len() {
            let mut new_grid = Vec::new();
            for j in 0..goal.len() {
                let max = (i as f64 - x as f64).abs().max((j as f64 - y as f64).abs());
                new_grid.push(max + 1f64);
            }
            weight.push(new_grid);
        }
    }

    fn init_cross(weight: &mut Vec<Vec<f64>>, goal: &Vec<Vec<i64>>) {
        *weight = Vec::new();
        let (x, y) = find_nb(0, goal);
        for i in 0..goal.len() {
            let mut new_grid = Vec::new();
            for j in 0..goal.len() {
                let max = (i as f64 - x as f64).abs() + (j as f64 - y as f64).abs();
                new_grid.push(max + 1f64);
            }
            weight.push(new_grid);
        }
    }

    pub fn process(&self, current: &Vec<Vec<i64>>, goal: &Vec<Vec<i64>>) -> f64 {
        match self {
            Strategy::Standard(heuristique) => Self::process_std(heuristique, current, goal),
            Strategy::Sandwich(weight, heuristique) => {
                Self::process_sandwich(weight, heuristique, current, goal)
            },
            Strategy::Cross(weight, heuristique) => {
                Self::process_cross(weight, heuristique, current, goal)
            }
        }
    }

    fn process_std(
        heuristique: &Heuristique,
        current: &Vec<Vec<i64>>,
        goal: &Vec<Vec<i64>>,
    ) -> f64 {
        heuristique
            .process_h(current, &goal)
            .iter()
            .map(|x| x.iter().sum())
            .collect::<Vec<f64>>()
            .iter()
            .sum()
    }

    fn process_sandwich(
        weight: &Vec<Vec<f64>>,
        heuristique: &Heuristique,
        current: &Vec<Vec<i64>>,
        goal: &Vec<Vec<i64>>,
    ) -> f64 {
        // TODO: Weight * heuristique
        heuristique
            .process_h(current, &goal)
            .iter()
            .enumerate()
            .map(|(i, x)| x.iter().enumerate().map(|(j, y)| y * weight[i][j] * weight[i][j]).sum())
            .collect::<Vec<f64>>()
            .iter()
            .sum() 
    }

    fn process_cross(
        weight: &Vec<Vec<f64>>,
        heuristique: &Heuristique,
        current: &Vec<Vec<i64>>,
        goal: &Vec<Vec<i64>>,
    ) -> f64 {
        heuristique
            .process_h(current, &goal)
            .iter()
            .enumerate()
            .map(|(i, x)| x.iter().enumerate().map(|(j, y)| y * weight[i][j]).sum())
            .collect::<Vec<f64>>()
            .iter()
            .sum() 
    }
}
