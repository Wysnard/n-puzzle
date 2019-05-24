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
                line.iter().enumerate().fold(0f64, |acc, (x, col)| {
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
        println!("HEURISTIQUE RES : {:?}", res);
        res
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    grid: Vec<Vec<i64>>,
    g: f64,
    h: f64,
    parent: Rc<Option<Node>>,
}

impl Node {
    fn new(
        grid: Vec<Vec<i64>>,
        parent: Option<Node>,
        goal: &Vec<Vec<i64>>,
        heuristique: &Heuristique,
    ) -> Node {
        let gplus = parent.clone();
        let gplus = match gplus {
            Some(n) => n.g,
            None => 0.0,
        };
        Node {
            grid,
            h: 0.0,
            g: 0.0 + gplus,
            parent: Rc::new(parent),
        }
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
    pub close_list: Vec<Node>,
}

impl NPuzzle {
    pub fn new(mut arg: String, heuristique: Heuristique) -> Result<NPuzzle, Box<dyn Error>> {
        let initial: Result<Vec<Vec<i64>>, _> = arg
            .lines()
            .map(|x| x.split('#').next().unwrap().trim())
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split(' ')
                    .map(|x| if x == "0" { "9223372036854775807" } else { x })
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
        let mut map: Vec<i64> = map.clone().into_iter().flatten().collect();
        map.sort();

        let mut goal: Vec<i64> = Vec::new();
        let mut row_begin = 0;
        let mut row_end = *size - 1;
        let mut col_begin = 0;
        let mut col_end = *size - 1;

        println!("SORTED : {:?}", map);
        while row_begin <= row_end && col_begin <= col_end {
            println!("#1");
            for i in col_begin..=col_end {
                println!(
                    "{} {} : {}",
                    row_begin,
                    i,
                    map[(row_begin * size + i) as usize]
                );
                goal.push(map[(row_begin * size + i) as usize]);
            }
            row_begin += 1;

            println!("#2");
            for i in row_begin..=row_end {
                println!("{} {} : {}", i, col_end, map[(i * size + col_end) as usize]);
                goal.push(map[(i * size + col_end) as usize]);
            }
            col_end -= 1;

            println!("#3");
            if row_begin <= row_end {
                for i in (col_begin..=col_end).rev() {
                    println!("{} {} : {}", row_end, i, map[(row_end * size + i) as usize]);
                    goal.push(map[(row_end * size + i) as usize]);
                }
            }
            row_end -= 1;

            println!("#4");
            if col_begin <= col_end {
                for i in (row_begin..=row_end).rev() {
                    println!(
                        "{} {} : {}",
                        i,
                        col_begin,
                        map[(i * size + col_begin) as usize]
                    );
                    goal.push(map[(i * size + col_begin) as usize]);
                }
            }
            col_begin += 1;
        }
        println!("GOAL : {:?}", goal);

        let mut res: Vec<Vec<i64>> = vec![vec![0; *size as usize]; *size as usize];
        for (i, s) in goal.iter().enumerate() {
            res[i / *size as usize][i % *size as usize] = *s;
        }
        // println!("RES : {:?}", res);
        res
    }

    pub fn run(&mut self) {
        println!("RUN !");
    }
}
