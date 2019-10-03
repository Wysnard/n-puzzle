use rayon::prelude::*;
use std::cmp;
use std::collections::BinaryHeap;
use std::error::Error;
use std::process;
use std::sync::Arc;

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
    pub open_list: BinaryHeap<Arc<Node>>,
    pub close_list: BinaryHeap<Arc<Node>>,
    pub max_state: usize,
    pub max_iteration: u64,
    pub debug: bool,
    pub thread: usize,
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
        thread: usize,
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
        let mut open_list: BinaryHeap<Arc<Node>> = BinaryHeap::new();
        open_list.push(Arc::new(Node::new(
            initial, None, &goal, &algorithm, &strategy,
        )));
        Ok(NPuzzle {
            size,
            goal: goal.clone(),
            algorithm,
            strategy,
            open_list,
            close_list: BinaryHeap::new(),
            max_state: 0,
            max_iteration,
            debug,
            thread,
        })
    }

    /*
     * Main loop which runs the algorithm
     */
    pub fn run(&mut self) {
        println!("RUN !");
        let mut epochs: u64 = 0;
        let mut next: Vec<Arc<Node>> = vec![self.open_list.pop().unwrap()];
        let zero = Arc::new(Node {
            grid: vec![],
            f: 100_000f64,
            h: 100_000f64,
            g: 100_000f64,
            parent: None,
        });

        let solved = loop {
            epochs += 1;
            let currents = next;

            if currents.iter().any(|x| x.grid == self.goal) {
                break currents
                    .iter()
                    .fold(zero, |a, b| std::cmp::max(a, b.clone()));
            }

            if self.debug {
                println!("EPOCH: {}", epochs);
                println!("CURRENTS : {:?}", currents);
            }

            let mut swaps: BinaryHeap<Arc<Node>> = currents
                .par_iter()
                .map(|current| self.generate_swaps(find_nb(0, &current.grid), current))
                .flatten()
                .collect();

            self.close_list.extend(currents);
            self.open_list = self
                .open_list
                .iter()
                .cloned()
                .filter(|x| !swaps.iter().any(|y| y == x))
                .collect();

            let it: _ = (0..self.thread).into_iter();
            next = match self.algorithm {
                Algorithm::AStar | Algorithm::BStar => {
                    self.open_list.extend(swaps);
                    it.map(|_| self.open_list.pop())
                        .filter_map(Option::Some)
                        .map(|x| x.unwrap())
                        .collect()
                }
                Algorithm::Greedy => {
                    let res: _ = it
                        .map(|_| swaps.pop().unwrap_or_else(|| self.open_list.pop().unwrap()))
                        .collect();
                    self.open_list.extend(swaps);
                    res
                }
            };

            self.max_state = cmp::max(self.max_state, self.open_list.len());
        };
        // Display of the solved puzzle
        println!("RESOLVED :");
        let g = solved.g;
        Self::display(&Some(solved));
        println!("Number of moves: {}", g);
        println!("Number of iterations : {}", epochs);
        println!("Complexity Size (Max States): {}", self.max_state);
    }

    fn display(cur: &Option<Arc<Node>>) {
        if cur.is_some() {
            let _ = cur.as_ref().map(|node| {
                Self::display(&node.parent);
                println!("{}", node);
            });
        }
    }

    fn generate_swaps(&self, pos: (i32, i32), parent: &Arc<Node>) -> Vec<Arc<Node>> {
        let current_grid = parent.grid.clone();
        let goal = self.goal.clone();

        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .par_iter()
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
                Arc::new(Node::new(
                    swap,
                    Some(parent.clone()),
                    &goal,
                    &self.algorithm,
                    &self.strategy,
                ))
            })
            .filter(|swap| {
                !self
                    .close_list
                    .iter()
                    .any(|x: &Arc<Node>| x.grid == swap.grid && x.f <= swap.f)
            })
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
