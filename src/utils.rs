extern crate rand;

use rand::Rng;

fn interversion(map: &[Vec<i64>], goal: &[Vec<i64>]) -> usize {
    let mut initial: Vec<&i64> = map.iter().flatten().filter(|&&x| x != 0).collect();
    let mut goal: Vec<&i64> = goal.iter().flatten().filter(|&&x| x != 0).collect();
    let mut res: usize = 0;
    while !goal.is_empty() {
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
                return (x as i32, y as i32);
            }
        }
    }
    (-1, -1)
}

pub fn solvable(initial: &[Vec<i64>], goal: &[Vec<i64>]) -> bool {
    if initial == goal {
        return true;
    }
    let interv = interversion(initial, goal);
    println!("INTERVERSION: {}", interv);
    let size = goal.len();
    let res = match size % 2 {
        1 => match interv % 2 {
            0 => true,
            _ => false,
        },
        0 => {
            println!("EVEN");
            println!("INITIAL: {:?}", initial);
            let (x, y) = find_nb(0, initial);
            println!("X, Y: {}, {}", x, y);
            if x % 2 == 0 && interv % 2 == 1 {
                true
            } else if x % 2 == 1 && interv % 2 == 0 {
                true
            } else {
                false
            }
        }
        _ => false,
    };
    println!("RES: {}", res);
    res
}

pub fn creat_new_rand(size: usize) -> String {
    let mut new_grid = Vec::new();
    let mut rng = rand::thread_rng();
    new_grid.push(0 as i64);
    for i in 1..size.pow(2) {
        new_grid.insert(rng.gen_range(0, i + 1), i as i64);
    }
	let mut return_value = format!("{}\n", size);
    for _i in 0..size {
        let temp_vect: String = new_grid.drain(..size).map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
		return_value = format!("{}\n{}",return_value, temp_vect);
    }
	return_value
}

pub fn with_duplicate(map: &Vec<Vec<i64>>) -> bool {
	let size = map.len();
    for i in 0..size {
        for j in i..size {
            for x in 0..size {
                for y in x..size {
                    if map[i as usize][x as usize] == map[j as usize][y as usize] && (i != j || x != y) {
						println!("{} | {}",map[i as usize][x as usize], map[j as usize][y as usize]);
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interversion_1() {
        let i = interversion(
            &vec![vec![1, 2, 3], vec![4, 5, 6], vec![8, 7, 0]],
            &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]],
        );
        assert_eq!(1 as usize, i);
    }
    #[test]
    fn test_interversion_2() {
        let i = interversion(
            &vec![vec![0, 2, 3], vec![4, 5, 6], vec![7, 8, 1]],
            &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]],
        );
        assert_eq!(7 as usize, i);
    }
    #[test]
    fn test_find_nb_1() {
        let v = &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
        let coord = find_nb(0, v);
        assert_eq!((2, 2), coord);
    }
    #[test]
    fn test_solvable_1() {
        let initial = &vec![vec![1, 8, 2], vec![0, 4, 3], vec![7, 6, 5]];
        let goal = &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
        assert_eq!(true, solvable(initial, goal));
    }
    #[test]
    fn test_solvable_2() {
        let initial = &vec![
            vec![13, 2, 10, 3],
            vec![1, 12, 8, 4],
            vec![5, 0, 9, 6],
            vec![15, 14, 11, 7],
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
    #[test]
    fn test_duplicate_1() {
        let initial = &vec![
            vec![1, 9, 1, 15],
            vec![14, 11, 4, 6],
            vec![13, 0, 10, 12],
            vec![2, 7, 8, 5],
        ];
        assert_eq!(true, with_duplicate(initial));
    }
    #[test]
    fn test_duplicate_2() {
        let initial = &vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        assert_eq!(false, with_duplicate(initial));
    }
    #[test]
    fn test_duplicate_3() {
        let initial = &vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        assert_eq!(true, with_duplicate(initial));
    }
	    #[test]
    fn test_duplicate_4() {
        let initial = &vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![1, 14, 15, 0],
        ];
        assert_eq!(true, with_duplicate(initial));
    }


}
