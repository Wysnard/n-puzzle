#[derive(Debug, Clone)]
pub enum Heuristique {
    Manhattan,
    Hamming,
}

impl Heuristique {
    pub fn process_h(&self, grid: &Vec<Vec<i64>>, goal: &Vec<Vec<i64>>) -> f64 {
        match &self {
            Heuristique::Manhattan => Heuristique::process_Manhattan(grid, goal),
            _ => 0.0,
        }
    }

    pub fn process_Manhattan(grid: &Vec<Vec<i64>>, goal: &Vec<Vec<i64>>) -> f64 {
        let res: f64 = grid
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, col)| **col != 0)
                    .fold(0f64, |acc, (x, col)| {
                        let g = goal
                            .iter()
                            .enumerate()
                            .filter(|(_, l)| l.iter().any(|c| c == col))
                            .fold((0f64, 0f64), |_, (g_y, l)| {
                                (
                                    g_y as f64,
                                    l.iter().enumerate().find(|c| c.1 == col).unwrap().0 as f64,
                                )
                            });
                        acc + (g.0 - y as f64).abs() + (g.1 - x as f64).abs()
                    })
            })
            .sum();
        // println!("HEURISTIQUE RES : {:?}", res);
        res
    }
}