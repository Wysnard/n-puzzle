use std::cmp::Ordering;
use std::rc::Rc;
use std::fmt;

use super::heuristique::*;

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

    pub fn get_f(&self) -> f64 {
        return self.g + self.h;
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        return self.h == other.h && self.g == self.g;
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        let ord = self.get_f().partial_cmp(&other.get_f());

        match ord {
            Some(v) => {
                Some(match v {
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => self.g.partial_cmp(&other.g).unwrap().reverse(),
                })
            },
            _ => None
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|grid: {:?} ~ f: {} = h: {} + g: {}|", self.grid, self.h + self.g, self.h, self.g)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}