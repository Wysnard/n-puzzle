use std::process;
use super::*;

#[derive(Debug)]
pub enum Algorithm {
    AStar,
    Greedy,
}

impl Algorithm {
    pub fn parse(s: String) -> Algorithm {
        match &s.to_lowercase() as &str {
            "astar" => Algorithm::AStar,
            "greedy" => Algorithm::Greedy,
            _ => {
                println!("Algorithm not recognized");
                process::exit(1);
            }
        }
    }
}