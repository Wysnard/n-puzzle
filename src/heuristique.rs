use super::utils::*;
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
            "manhattan" => Heuristique::Manhattan,
            "linearconflict" => Heuristique::LinearConflict,
            _ => {
                println!("Heuristique not recognized");
                process::exit(1);
            }
        }
    }

    pub fn process_h(&self, grid: &[Vec<i64>], goal: &[Vec<i64>]) -> Vec<Vec<f64>> {
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

    fn process_manhattan(grid: &[Vec<i64>], goal: &[Vec<i64>]) -> f64 {
        let mut res: f64 = 0.0f64;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let (x, y) = find_nb(grid[i][j], goal);
                if grid[i][j] != 0 && grid[i][j] == goal[x as usize][y as usize] {
                    res += (i as f64 - x as f64).abs() + (j as f64 - y as f64).abs();
                }
            }
        }
        // println!("HEURISTIQUE RES : {:?}", res);
        res
    }

    fn process_linearconflict(grid: &[Vec<i64>], goal: &[Vec<i64>]) -> f64 {
        let mut h = Self::process_manhattan(grid, goal);
        for i in 0..grid.len() {
            let i = i as i32;
            for j in 0..grid.len() {
                let j = j as i32;
                let (x, y) = find_nb(grid[i as usize][j as usize], goal);
                if grid[x as usize][y as usize] == 0 {
                    continue;
                }

                if (i == x) ^ (j == y) {
                    if (i - x).abs() as f64 + (j - y).abs() as f64 != 1f64
                        && goal[x as usize][y as usize] != 0
                    {
                        h += 1f64;
                    }
                }
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solvable_1() {
        let initial = &vec![vec![1, 2, 3], vec![4, 5, 6], vec![8, 7, 0]];
        let goal = &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
        assert_eq!(2f64, Heuristique::process_manhattan(initial, goal));
    }
}
