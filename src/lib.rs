use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

pub mod heuristique;
pub mod node;

use node::*;
use heuristique::*;

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
    pub open_list: Vec<Rc<Node>>,
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
        println!("Heuristique : {:?}", h);
        Ok(NPuzzle {
            size,
            goal: goal.clone(),
            heuristique: heuristique.clone(),
            open_list: vec![Rc::new(Node::new(initial, None, &goal, &heuristique))],
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
            println!("CURRENT : {}", current);

            // empty space position "0"
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
                println!("TOP SWAP !");
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
                println!("RIGHT SWAP !");
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
                println!("BOTTOM SWAP !");
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
                println!("LEFT SWAP !");
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
            self.open_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
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
        println!("NODE Explored : {}", i);
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
