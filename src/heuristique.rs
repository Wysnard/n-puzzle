use super::utils::*;
use rayon::prelude::*;
use std::process;

#[derive(Debug, Clone)]
pub enum Heuristique {
    Hamming,
    Manhattan,
    LinearConflict,
}

impl Heuristique {
    pub fn parse(s: String) -> Heuristique {
        match &s.to_lowercase() as &str {
            "hamming" => Heuristique::Hamming,
            "std" | "manhattan" => Heuristique::Manhattan,
            "linearconflict" => Heuristique::LinearConflict,
            _ => {
                println!("Heuristique not recognized");
                process::exit(1);
            }
        }
    }

    pub fn process_h(&self, goal: &[Vec<i64>], grid: &[Vec<i64>]) -> Vec<Vec<f64>> {
        match &self {
            Heuristique::Hamming => Self::process_hamming(grid, goal),
            Heuristique::Manhattan => Self::process_manhattan(grid, goal),
            Heuristique::LinearConflict => Self::process_linearconflict(grid, goal),
        }
    }

    fn process_hamming(grid: &[Vec<i64>], goal: &[Vec<i64>]) -> Vec<Vec<f64>> {
        grid.iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, &y)| {
                        let (a, b) = find_nb(y, goal);
                        if (a as usize) == i && (b as usize) == j {
                            1f64
                        } else {
                            0f64
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn process_manhattan(grid: &[Vec<i64>], goal: &[Vec<i64>]) -> Vec<Vec<f64>> {
        grid.iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, &y)| {
                        let (a, b) = find_nb(y, goal);
                        if y != 0 && y == goal[a as usize][b as usize] {
                            (i as f64 - a as f64).abs() + (j as f64 - b as f64).abs()
                        } else {
                            0f64
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn process_linearconflict(grid: &[Vec<i64>], goal: &[Vec<i64>]) -> Vec<Vec<f64>> {
        let tmp_vec = Self::process_manhattan(grid, goal);
        grid.iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, &y)| {
                        let (a, b) = find_nb(y, goal);
                        if y != 0 && (i == a as usize) ^ (j == b as usize) {
                            if (i as f64 - a as f64).abs() as f64
                                + (j as f64 - b as f64).abs() as f64
                                != 1f64
                                && goal[a as usize][b as usize] != 0
                            {
                                1f64 + tmp_vec[i][j]
                            } else {
                                0f64 + tmp_vec[i][j]
                            }
                        } else {
                            0f64 + tmp_vec[i][j]
                        }
                    })
                    .collect()
            })
            .collect()
    }
}
