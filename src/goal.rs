use super::file::*;
use std::fs;
use std::process;

pub enum Goal {
	Snail,
	Standard,
	Custom(String),
}

impl Goal {
	pub fn parse(s: String, file: String) -> Goal {
		match &s.to_lowercase() as &str {
			"snail" => Goal::Snail,
			"standard" | "std" => Goal::Standard,
			_ => Goal::Custom(file),
		}
	}

	pub fn generate(&self, size: &i64, map: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
		let mut map: Vec<i64> = map
			.clone()
			.into_iter()
			.flatten()
			.map(|x| if x == 0 { 9223372036854775807 } else { x })
			.collect();
		map.sort();
		match &self {
			Goal::Snail => Self::generate_snail(size, map),
			Goal::Standard => Self::generate_std(size, map),
			Goal::Custom(file) => Self::generate_custom(size, &vec![], file),
		}
	}

	fn generate_snail(size: &i64, map: Vec<i64>) -> Vec<Vec<i64>> {
		let mut map = map
			.iter()
			.map(|x| if x == &9223372036854775807 { &0 } else { x });

		let mut A: Vec<Vec<i64>> = vec![vec![0; *size as usize]; *size as usize];
		let n = *size;
		let mut len = *size;
		let mut k = 0;
		let mut p = 0;

		while k < n * n {
			for i in p..len {
				A[p as usize][i as usize] = *map.next().unwrap();
				k += 1;
			}

			for i in p + 1..len {
				A[i as usize][(len - 1) as usize] = *map.next().unwrap();
				k += 1;
			}

			for i in (p..len - 1).rev() {
				A[(len - 1) as usize][i as usize] = *map.next().unwrap();
				k += 1
			}

			for i in (p + 1..len - 1).rev() {
				A[i as usize][p as usize] = *map.next().unwrap();
				k += 1;
			}

			p += 1;
			len -= 1;
		}
		println!("A : {:?}", A);
		A
	}

	fn generate_std(size: &i64, map: Vec<i64>) -> Vec<Vec<i64>> {
		//TODO : Generate std order
		let mut res: Vec<Vec<i64>> = vec![vec![]; 3];
		for (i, c) in map.iter().enumerate() {
			res[i / *size as usize].push(*c);
		}
		res
	}

	fn generate_custom(size: &i64, map: &Vec<Vec<i64>>, file: &str) -> Vec<Vec<i64>> {
		if let Ok(map) = fs::read_to_string(file) {
			match parse_file(map) {
				Ok((_, v)) => v,
				Err(e) => {
					println!("{}", e);
					process::exit(1);
				}
			}
		} else {
			println!("Cannot read the file : {}", file);
			process::exit(1);
		}
	}
}