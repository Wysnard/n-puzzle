use std::collections::BinaryHeap;
use std::error::Error;
use std::process;
use std::rc::Rc;

pub mod file;
pub mod goal;
pub mod heuristique;
pub mod node;
pub mod utils;
pub mod algorithm;
pub mod strategy;

use file::*;
use goal::*;
use heuristique::*;
use node::*;
use utils::*;
use algorithm::*;
use strategy::*;

#[derive(Debug)]
pub struct NPuzzle {
	pub size: i64,
	pub goal: Vec<Vec<i64>>,
	pub heuristique: Heuristique,
	pub algorithm: Algorithm,
	pub strategy: Strategy,
	pub open_list: BinaryHeap<Rc<Node>>,
	pub close_list: Vec<Rc<Node>>,
	pub max_state: usize,
	pub max_iteration: u64,
}

impl NPuzzle {
	pub fn new(
		arg: String,
		heuristique: Heuristique,
		algorithm: Algorithm,
		strategy: Strategy,
		goal: Goal,
		max_iteration: u64,
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
		let h = heuristique.process_h(&initial, &goal);
		println!("Heuristique : {:?}", h);
		let mut open_list: BinaryHeap<Rc<Node>> = BinaryHeap::new();
		open_list.push(Rc::new(Node::new(initial, None, &goal, &heuristique)));
		Ok(NPuzzle {
			size,
			goal: goal.clone(),
			heuristique: heuristique.clone(),
			algorithm,
			strategy,
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

			let pos = find_nb(0, &current.grid);
			// println!("POS: {:?}", pos);

			let current_grid = current.grid.clone();
			let goal = self.strategy.process(&current_grid ,self.goal.clone());
			let parent = current;
			let swaps: Vec<Rc<Node>> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
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
					Rc::new(Node::new(
						swap,
						Some(parent.clone()),
						&goal,
						&self.heuristique,
					))
				})
				.filter(|swap| self.close_list.is_empty() || !self.close_list.contains(swap))
				.collect();
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
		println!("RESOLVED :");
		fn display(cur: &Option<Rc<Node>>) {
			if cur.is_some() {
				let _ = cur.as_ref().map(|node| {
					display(&node.parent);
					println!("{}", node);
				});
			}
		}
		display(&Some(solved));
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
			goal.generate(3, &vec![vec![3, 1, 5], vec![4, 2, 6], vec![0, 8, 7]]),
			vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]]
		);
	}
}
