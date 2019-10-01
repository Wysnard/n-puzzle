use std::collections::BinaryHeap;
use std::error::Error;
use std::process;
use std::rc::Rc;
use std::collections::BTreeSet;

pub mod algorithm;
pub mod file;
pub mod goal;
pub mod heuristique;
pub mod node;
pub mod strategy;
pub mod utils;

use algorithm::*;
use file::*;
use goal::*;
use heuristique::*;
use node::*;
use strategy::*;
use utils::*;

#[derive(Debug)]
pub struct NPuzzle {
    pub size: i64,
    pub goal: Vec<Vec<i64>>,
    pub algorithm: Algorithm,
    pub strategy: Strategy,
    pub open_list: BinaryHeap<Rc<Node>>,
    pub close_list: BTreeSet<Rc<Node>>,
    pub max_state: usize,
    pub max_iteration: u64,
    pub debug: bool,
}

impl NPuzzle {
    pub fn new(
        arg: String,
        heuristique: Heuristique,
        algorithm: Algorithm,
        strategy: String,
        goal: Goal,
        max_iteration: u64,
        debug: bool,
    ) -> Result<NPuzzle, Box<dyn Error>> {
        let (size, initial) = parse_file(arg)?;
        println!("SIZE : {:?}", size);
        println!("INITIAL : {:?}", initial);
        let goal = goal.generate(size, &initial);
        println!("GOAL : {:?}", goal);
        if !solvable(&initial, &goal) {
            println!("Unsolvable puzzle");
            process::exit(1);
        }
        let mut strategy = Strategy::parse(strategy, heuristique);
        strategy.init(&goal);
        println!("Strategy: {:?}", strategy);
        let mut open_list: BinaryHeap<Rc<Node>> = BinaryHeap::new();
        open_list.push(Rc::new(Node::new(initial, None, &goal, &algorithm, &strategy)));
        Ok(NPuzzle {
            size,
            goal: goal.clone(),
            algorithm,
            strategy,
            open_list,
            close_list: BTreeSet::new(),
            max_state: 0,
            max_iteration,
            debug,
        })
    }

    /*
     * Main loop which runs the algorithm
     */
    pub fn run(&mut self) {
        println!("RUN !");
        let mut epochs: u64 = 0;
        let mut next = self.open_list.pop().unwrap();

        let solved = loop {
            epochs += 1;
            let current = next;

            if current.h == 0.0 {
                break current;
            }

            if self.debug {
                println!("EPOCH: {}", epochs);
                println!("CURRENT : {:?}", current);
                
            }

            let mut swaps: BinaryHeap<Rc<Node>> =
                self.generate_swaps(find_nb(0, &current.grid), &current);
            self.close_list.insert(current);

            match self.algorithm {
                Algorithm::AStar | Algorithm::BStar => {
                    self.open_list.extend(swaps);
                    next = self.open_list.pop().unwrap();
                }
                Algorithm::Greedy => {
                    next = swaps.pop().unwrap_or_else(|| self.open_list.pop().unwrap());
                    self.open_list.extend(swaps);
                }
            };

            let l = self.open_list.len();
            self.max_state = if l > self.max_state {
                l
            } else {
                self.max_state
            };
        };
        // Display of the solved puzzle
        println!("RESOLVED :");
        let g = solved.g;
        Self::display(&Some(solved));
        println!("Number of moves: {}", g);
        println!("Number of iterations : {}", epochs);
        println!("Complexity Size (Max States): {}", self.max_state);
    }

    fn display(cur: &Option<Rc<Node>>) {
        if cur.is_some() {
            let _ = cur.as_ref().map(|node| {
                Self::display(&node.parent);
                println!("{}", node);
            });
        }
    }

    fn generate_swaps(&self, pos: (i32, i32), parent: &Rc<Node>) -> BinaryHeap<Rc<Node>> {
        let current_grid = parent.grid.clone();
        let goal = self.goal.clone();

        vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .filter(|&(x, y)| {
                pos.0 + x >= 0
                    && pos.1 + y >= 0
                    && pos.0 + x < self.size as i32
                    && pos.1 + y < self.size as i32
            })
            .map(|(x, y)| {
                let mut swap = current_grid.clone();
                swap[pos.0 as usize][pos.1 as usize] =
                    swap[(pos.0 + x) as usize][(pos.1 + y) as usize];
                swap[(pos.0 + x) as usize][(pos.1 + y) as usize] = 0;
                Rc::new(Node::new(swap, Some(parent.clone()), &goal, &self.algorithm, &self.strategy))
            })
            .filter(|swap| self.open_list.iter().all(|x| x != swap))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_goals() {
        let goal: Goal = Goal::Snail;
        assert_eq!(
            goal.generate(3, &vec![vec![3, 1, 5], vec![4, 2, 6], vec![0, 8, 7]]),
            vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]]
        );
    }
}
