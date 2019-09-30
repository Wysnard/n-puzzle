use std::process;
use super::utils::*;


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
		let mut deep = 1;
		let (x, y) = find_nb(0, &goal);
		'deep: for i in 0..(goal.len() - 2) {
			for j in 0..(goal.len() - 2) {
				if i < x as usize {
					if current[i][j] != goal[i][j] {
						break 'deep;
					}
				}
				else {
					if current[goal.len() - (i - x as usize) - 1][j] != goal[goal.len() - (i - x as usize) - 1][j] {
						break 'deep;
					}
				}
				if j < y as usize {
					if current[i][j] != goal[i][j] {
						break 'deep;
					}
				}
				else {
					if current[goal.len() - (j - y as usize) - 1][i] != goal[goal.len() - (j - y as usize) - 1][i] {
						break 'deep;
					}
				}
			}
			deep = i + 1;
		}
		if deep >= goal.len() - 2 {
			return goal;
		}
		let mut new_grid = Vec::new();

		for _i in 0..goal.len() {
			new_grid.push(vec![0 as i64; goal.len()]);
		}
		for i in 0..deep {
			for j in 0..goal.len() {
				if i < x as usize {
					new_grid[i][j] = goal[i][j];
				}
				else {
					new_grid[goal.len() - (i - x as usize) - 1][j] = goal[goal.len() - (i - x as usize) - 1][j];
				}
				if i < y as usize {
					new_grid[j][i] = goal[j][i];
				}
				else {
					new_grid[goal.len() - (j - y as usize) - 1][i] = goal[goal.len() - (j - y as usize) - 1][i];
				}
			}
		}
		new_grid
	}
}
