use std::collections::BinaryHeap;
use std::error::Error;
use std::process;
use std::rc::Rc;

pub mod file;
pub mod goal;
pub mod heuristique;
pub mod node;
pub mod utils;

use file::*;
use goal::*;
use heuristique::*;
use node::*;
use utils::*;

#[derive(Debug)]
pub struct NPuzzle {
    pub size: i64,
    pub goal: Vec<Vec<i64>>,
    pub heuristique: Heuristique,
    pub open_list: BinaryHeap<Rc<Node>>,
    pub close_list: Vec<Rc<Node>>,
    pub max_state: usize,
    pub max_iteration: u64,
}

impl NPuzzle {
    pub fn new(
        arg: String,
        heuristique: Heuristique,
        goal: Goal,
        max_iteration: u64,
    ) -> Result<NPuzzle, Box<dyn Error>> {
        let (size, initial) = parse_file(arg)?;
        println!("SIZE : {:?}", size);
        println!("INITIAL : {:?}", initial);
        let goal = goal.generate(&size, &initial);
        println!("GOAL : {:?}", goal);
        if !solvable(&initial, &goal) {
            println!("Unsolvable puzzle");
            process::exit(1);
        }
        let h = heuristique.process_h(&initial, &goal);
        println!("Heuristique : {:?}", h);
        let mut open_list: BinaryHeap<Rc<Node>> = BinaryHeap::new();
        open_list.push(Rc::new(Node::new(initial, None, &goal, &heuristique)));
        Ok(NPuzzle {
            size,
            goal: goal.clone(),
            heuristique: heuristique.clone(),
            open_list,
            close_list: Vec::new(),
            max_state: 0,
            max_iteration,
        })
    }

    /*
     * Main loop which runs the algorithm
     */
    pub fn run(&mut self) {
        println!("RUN !");
        let mut epochs: u64 = 0;
        let solved = loop {
            epochs += 1;
            let current = self.open_list.pop().unwrap();

            if current.h == 0.0 {
                break current;
            }

            // println!("OPEN LIST : {:?}", self.open_list);
            // println!("CURRENT : {:?}", current);
            // println!("EPOCH: {}", epochs);

            // Empty Space position
            let pos = current
                .grid
                .iter()
                .enumerate()
                .fold((0usize, 0usize), |acc, (i, x)| {
                    if x.iter().any(|&y| y == 0) {
                        (i, x.iter().position(|&y| y == 0).unwrap())
                    } else {
                        acc
                    }
                });
            // println!("POS: {:?}", pos);

            let mut swaps: Vec<Rc<Node>> = vec![];
            let current_grid = current.grid.clone();
            let goal = self.goal.clone();
            let parent = current;

            if pos.0 as i32 > 0 {
                // println!("TOP SWAP !");
                let mut top = current_grid.clone();
                top[pos.0][pos.1] = top[pos.0 - 1][pos.1];
                top[pos.0 - 1][pos.1] = 0;
                swaps.push(Rc::new(Node::new(
                    top,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                )));
            }

            if pos.1 + 1 < self.size as usize {
                // println!("RIGHT SWAP !");
                let mut right = current_grid.clone();
                right[pos.0][pos.1] = right[pos.0][pos.1 + 1];
                right[pos.0][pos.1 + 1] = 0;
                swaps.push(Rc::new(Node::new(
                    right,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                )));
            }

            if pos.0 + 1 < self.size as usize {
                // println!("BOTTOM SWAP !");
                let mut bottom = current_grid.clone();
                bottom[pos.0][pos.1] = bottom[pos.0 + 1][pos.1];
                bottom[pos.0 + 1][pos.1] = 0;
                swaps.push(Rc::new(Node::new(
                    bottom,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                )));
            }

            if pos.1 as i32 > 0 {
                // println!("LEFT SWAP !");
                let mut left = current_grid.clone();
                left[pos.0][pos.1] = left[pos.0][pos.1 - 1];
                left[pos.0][pos.1 - 1] = 0;
                swaps.push(Rc::new(Node::new(
                    left,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                )));
            }

            self.close_list.push(parent);
            self.open_list.extend(swaps);
            let l = self.open_list.len();
            self.max_state = if l > self.max_state {
                l
            } else {
                self.max_state
            };
            // println!("NPUZZLE : {:?}", self);
        };

        // Display of the solved puzzle
        let mut cur: &Option<Rc<Node>> = &Some(solved);
        println!("RESOLVED :");
        while cur.is_some() {
            let _ = cur.as_ref().map(|node| {
                println!("{}", node);
                cur = &node.parent;
            });
        }
        println!("EPOCHS : {}", epochs);
        println!("MAX STATES : {}", self.max_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_goals() {
        let goal: Goal = Goal::Snail;
        assert_eq!(
            goal.generate(&3, &vec![vec![3, 1, 5], vec![4, 2, 6], vec![0, 8, 7]]),
            vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]]
        );
    }
}
