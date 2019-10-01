use std::process;

#[derive(Debug)]
pub enum Algorithm {
    AStar,
    BStar,
    Greedy,
}

impl Algorithm {
    pub fn parse(s: String) -> Algorithm {
        match &s.to_lowercase() as &str {
            "astar" => Algorithm::AStar,
            "bstar" => Algorithm::BStar,
            "greedy" => Algorithm::Greedy,
            _ => {
                println!("Algorithm not recognized");
                process::exit(1);
            }
        }
    }
}
