use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use super::strategy::*;

type Link = Option<Rc<Node>>;

#[derive(Clone)]
pub struct Node {
    pub grid: Vec<Vec<i64>>,
    pub g: f64,
    pub h: f64,
    pub parent: Link,
}

impl Node {
    pub fn new(
        grid: Vec<Vec<i64>>,
        parent: Link,
        goal: &Vec<Vec<i64>>,
        strategy: &Strategy,
    ) -> Node {
        let g = parent.clone();
        let g = match g {
            Some(n) => n.g + 1.0,
            None => 0.0,
        };
        let h = strategy.process(&grid, goal);
        Node { grid, h, g, parent }
    }

    pub fn get_f(&self) -> f64 {
        self.g + self.h
    }
}

/*
 * Warning: Order for Node is reverse to make the binary heap a min-heap
*/

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.get_f().partial_cmp(&other.get_f()).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => self.g.partial_cmp(&other.g).unwrap(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.grid == other.grid && self.g <= other.g
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        let ord = self.get_f().partial_cmp(&other.get_f());

        match ord {
            Some(v) => Some(match v {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => self.g.partial_cmp(&other.g).unwrap(),
            }),
            _ => None,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "f: {} = h: {} + g: {}
{}
",
            self.h + self.g,
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
            g: 5.0f64,
            h: 5.0f64,
            parent: None,
        };
        let mut second = first.clone();
        second.g = 3.0f64;
        assert_eq!(false, first == second);
    }

}
