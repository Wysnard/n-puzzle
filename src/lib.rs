use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Heuristique {
    Manhattan,
    Hamming,
}

impl Heuristique {
    fn process_h(&self, grid: &Vec<Vec<i64>>, goal: &Vec<Vec<i64>>) -> f64 {
        match &self {
            Heuristique::Manhattan => Heuristique::process_Manhattan(grid, goal),
            _ => 0.0,
        }
    }

    fn process_Manhattan(grid: &Vec<Vec<i64>>, goal: &Vec<Vec<i64>>) -> f64 {
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

#[derive(Debug, Clone)]
pub struct Node {
    grid: Vec<Vec<i64>>,
    g: f64,
    h: f64,
    parent: Option<Rc<Node>>,
}

impl Node {
    fn new(
        grid: Vec<Vec<i64>>,
        parent: Option<Rc<Node>>,
        goal: &Vec<Vec<i64>>,
        heuristique: &Heuristique,
    ) -> Node {
        let g = parent.clone();
        let g = match g {
            Some(n) => n.g + 1.0,
            None => 0.0,
        };
        let h = heuristique.process_h(&grid, goal);
        Node { grid, h, g, parent }
    }

    fn get_f(&self) -> f64 {
        return self.g + self.h;
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        return self.get_f() == other.get_f();
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        self.get_f().partial_cmp(&other.get_f())
    }
}

#[derive(Debug)]
pub enum PuzzleError {
    EmptyMap,
    BadSize,
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            PuzzleError::EmptyMap => write!(f, "It seems that the input file is empty"),
            PuzzleError::BadSize => write!(f, "Map size is incorrect."),
            _ => write!(f, "Format Error."),
        }
    }
}

impl Error for PuzzleError {}

#[derive(Debug)]
pub struct NPuzzle {
    pub size: i64,
    pub goal: Vec<Vec<i64>>,
    pub heuristique: Heuristique,
    pub open_list: Vec<Node>,
    pub close_list: Vec<Rc<Node>>,
}

impl NPuzzle {
    pub fn new(mut arg: String, heuristique: Heuristique) -> Result<NPuzzle, Box<dyn Error>> {
        let initial: Result<Vec<Vec<i64>>, _> = arg
            .lines()
            .map(|x| x.split('#').next().unwrap().trim())
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split_whitespace()
                    // .map(|x| if x == "0" { "9223372036854775807" } else { x })
                    .map(|x| x.parse::<i64>())
                    .collect()
            })
            .collect();
        let mut initial = initial?;
        let mut size = initial.remove(0);
        if size.len() > 1 {
            return Err(Box::new(PuzzleError::BadSize));
        }
        let size = size.remove(0);
        println!("SIZE : {:?}", size);
        if size <= 0
            || size != initial.len() as i64
            || size != initial.iter().map(|x| x.len()).max().unwrap() as i64
        {
            return Err(Box::new(PuzzleError::BadSize));
        }
        println!("INITIAL : {:?}", initial);
        let goal = NPuzzle::generate_goal(&size, &initial);
        println!("GOAL : {:?}", goal);
        let h = heuristique.process_h(&initial, &goal);
        println!("Heauristique : {:?}", h);
        Ok(NPuzzle {
            size,
            goal: goal.clone(),
            heuristique: heuristique.clone(),
            open_list: vec![Node::new(initial, None, &goal, &heuristique)],
            close_list: Vec::new(),
        })
    }

    pub fn generate_goal(size: &i64, map: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let mut map: Vec<i64> = map
            .clone()
            .into_iter()
            .flatten()
            .map(|x| if x == 0 { 9223372036854775807 } else { x })
            .collect();
        map.sort();

        let mut map = map
            .iter()
            .map(|x| if x == &9223372036854775807 { &0 } else { x });

        let mut A: Vec<Vec<i64>> = vec![vec![0; *size as usize]; *size as usize];
        let n = *size;
        let mut len = *size;
        let mut k = 0;
        let mut p = 0;

        while k < n * n {
            // println!("#1");
            for i in p..len {
                // println!("{} {}", p, i);
                A[p as usize][i as usize] = *map.next().unwrap();
                k += 1;
            }

            // println!("#2");
            for i in p + 1..len {
                A[i as usize][(len - 1) as usize] = *map.next().unwrap();
                // println!("{} {} : {}", i, len, A[i as usize][(len - 1) as usize]);
                k += 1;
            }

            // println!("#3");
            for i in (p..len - 1).rev() {
                A[(len - 1) as usize][i as usize] = *map.next().unwrap();
                // println!("{} {} : {}", len - 1, i, A[(len - 1) as usize][i as usize]);
                k += 1
            }

            // println!("#4");
            for i in (p + 1..len - 1).rev() {
                A[i as usize][p as usize] = *map.next().unwrap();
                // println!("{} {} : {}", i, p, A[i as usize][p as usize]);
                k += 1;
            }

            p += 1;
            len -= 1;
        }
        println!("A : {:?}", A);
        A
    }

    pub fn run(&mut self) {
        println!("RUN !");
        let mut i = 0;
        let solved = loop {
            i += 1;
            let current = self.open_list.remove(0);

            if current.h == 0.0 {
                break current;
            }

            println!("OPEN LIST : {:?}", self.open_list);
            println!("CURRENT : {:?}", current);

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
            println!("POS: {:?}", pos);

            let mut swaps: Vec<Node> = vec![];
            let current_grid = current.grid.clone();
            let goal = self.goal.clone();
            let parent = Rc::new(current);
            if pos.0 as i32 > 0 {
                println!("TOP SWAP !");
                let mut top = current_grid.clone();
                top[pos.0][pos.1] = top[pos.0 - 1][pos.1];
                top[pos.0 - 1][pos.1] = 0;
                swaps.push(Node::new(
                    top,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                ));
            }

            if pos.1 + 1 < self.size as usize {
                println!("RIGHT SWAP !");
                let mut right = current_grid.clone();
                right[pos.0][pos.1] = right[pos.0][pos.1 + 1];
                right[pos.0][pos.1 + 1] = 0;
                swaps.push(Node::new(
                    right,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                ));
            }

            if pos.0 + 1 < self.size as usize {
                println!("BOTTOM SWAP !");
                let mut bottom = current_grid.clone();
                bottom[pos.0][pos.1] = bottom[pos.0 + 1][pos.1];
                bottom[pos.0 + 1][pos.1] = 0;
                swaps.push(Node::new(
                    bottom,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                ));
            }

            if pos.1 as i32 > 0 {
                println!("LEFT SWAP !");
                let mut left = current_grid.clone();
                left[pos.0][pos.1] = left[pos.0][pos.1 - 1];
                left[pos.0][pos.1 - 1] = 0;
                swaps.push(Node::new(
                    left,
                    Some(parent.clone()),
                    &goal,
                    &self.heuristique,
                ));
            }

            // println!("SWAPS | {} : {:?}", swaps.len(), swaps);

            self.close_list.push(parent);
            self.open_list.extend(swaps);
            self.open_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
            // println!("NPUZZLE : {:?}", self);
        };

        println!("RESOLVED : {:?}", solved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_goals() {
        assert_eq!(
            NPuzzle::generate_goal(&3, &vec![vec![3, 1, 5], vec![4, 2, 6], vec![0, 8, 7]]),
            vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]]
        );
    }
}
