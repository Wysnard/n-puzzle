use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;

use super::algorithm::*;
use super::strategy::*;

type Link = Option<Arc<Node>>;

#[derive(Clone)]
pub struct Node {
    pub grid: Vec<Vec<i64>>,
    pub f: f64,
    pub g: f64,
    pub h: f64,
    pub parent: Link,
}

impl Node {
    pub fn new(
        grid: Vec<Vec<i64>>,
        parent: Link,
        goal: &Vec<Vec<i64>>,
        algorithm: &Algorithm,
        strategy: &Strategy,
    ) -> Node {
        let p = parent.clone();
        let g = match p {
            Some(n) => n.g + 1.0,
            None => 0.0,
        };
        let h = strategy.process(&grid, goal);
        let p = parent.clone();
        let f = match algorithm {
            Algorithm::AStar | Algorithm::Greedy => h + g,
            Algorithm::BStar => match p {
                Some(n) => (h + g) - (n.h + n.g),
                None => 0.0,
            },
        };
        Node {
            grid,
            f,
            h,
            g,
            parent,
        }
    }
}

/*
 * Warning: Order for Node is reverse to make the binary heap a min-heap
 */
impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.f.partial_cmp(&other.f).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => self.g.partial_cmp(&other.g).unwrap(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.grid == other.grid && self.f <= other.f && self.g <= other.g
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "f: {} <- h: {} + g: {}
{}
",
            self.f,
            self.h,
            self.g,
            self.grid
                .iter()
                .map(|x| x
                    .iter()
                    .map(|&y| y.to_string())
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_node_equality() {
        let first = Node {
            grid: vec![vec![0; 3]; 3],
            f: 10f64,
            g: 5.0f64,
            h: 5.0f64,
            parent: None,
        };
        let second = first.clone();
        assert_eq!(true, first == second);
    }
    #[test]
    fn test_node_inequality() {
        let first = Node {
            grid: vec![vec![0; 3]; 3],
            f: 10f64,
            g: 5.0f64,
            h: 5.0f64,
            parent: None,
        };
        let mut second = first.clone();
        second.g = 3.0f64;
        assert_eq!(false, first == second);
    }

}
