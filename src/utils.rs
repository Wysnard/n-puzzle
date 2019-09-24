use std::process;

fn interversion(map: &[Vec<i64>], goal: &[Vec<i64>]) -> usize {
    let mut initial: Vec<i64> = map.clone().into_iter().flatten().filter(|&&x| x != 0).map(|&x| x).collect();
    let mut goal: Vec<i64> = goal.clone().into_iter().flatten().filter(|&&x| x != 0).map(|&x| x).collect();
    let mut res: usize = 0;
    while goal.len() != 0 {
        let g = goal.remove(0);
        let pos: usize = initial.iter().position(|&x| x == g).unwrap() as usize;
        res += initial[0..pos].iter().count() as usize;
        initial.remove(pos);
    }
    res
}

pub fn find_nb(nb: i64, map: &[Vec<i64>]) -> (i32, i32) {
    for x in 0..map.len() {
        for y in 0..map.len() {
            if nb == map[x][y] {
                return (x as i32 , y as i32);
            }
        }
    }
    (-1,-1)
}

pub fn solvable(initial: &[Vec<i64>], goal: &[Vec<i64>]) -> bool {
    let interv = interversion(initial, goal);
    let size = initial.len();
    let res = match size % 2 {
        1 => match interv % 2 {
            0 => true,
            _ => false,
        },
        _ => {
            let (x, y) = find_nb(0, initial);
            if x % 2 == 0 && interv % 2 == 1 {
                true
            } else if x % 2 == 1 && interv % 2 == 0 {
                true
            } else {
                false
            }
        },
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interversion_1() {
        let i = interversion(&vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![8, 7, 0]
        ],
        &vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 0]
        ]);
        assert_eq!(1 as usize, i);
    }
    #[test]
    fn test_interversion_2() {
        let i = interversion(&vec![
            vec![0, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 1]
        ],
        &vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 0]
        ]);
        assert_eq!(7 as usize, i);
    }
    #[test]
    fn test_find_nb_1() {
        let v = &vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 0]
        ];
        let coord = find_nb(0, v);
        assert_eq!((2,2), coord);
    }
    #[test]
    fn test_solvable_1() {
        let initial = &vec![
            vec![1, 8, 2],
            vec![0, 4, 3],
            vec![7, 6, 5]
        ];
        let goal = &vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 0]
        ];
        assert_eq!(true, solvable(initial, goal));
    }
    #[test]
    fn test_solvable_2() {
        let initial = &vec![
            vec![13, 2, 10, 3],
            vec![1, 12, 8, 4],
            vec![5, 0, 9, 6],
            vec![15,14,11,7],
        ];
        let goal = &vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        assert_eq!(true, solvable(initial, goal));
    }
    #[test]
    fn test_solvable_3() {
        let initial = &vec![
            vec![6, 13, 7, 10],
            vec![8, 9, 11, 0],
            vec![15, 2, 12, 5],
            vec![14, 3, 1, 4],
        ];
        let goal = &vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        assert_eq!(true, solvable(initial, goal));
    }
    #[test]
    fn test_solvable_4() {
        let initial = &vec![
            vec![3, 9, 1, 15],
            vec![14, 11, 4, 6],
            vec![13, 0, 10, 12],
            vec![2, 7, 8, 5],
        ];
        let goal = &vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        assert_eq!(false, solvable(initial, goal));
    }
}