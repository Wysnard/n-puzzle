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

pub trait Algo {
    fn run(&self);
}

#[derive(Debug)]
pub struct AStar {
    pub open_list: BinaryHeap<Rc<Node>>,
    pub close_list: Vec<Rc<Node>>,
}

impl Algo for AStar {
    fn run(&self) {
        println!("{:?}", self.close_list);
    }
}