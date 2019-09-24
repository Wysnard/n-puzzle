use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FileParsingError {
	EmptyMap,
	BadSize,
}

impl fmt::Display for FileParsingError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			FileParsingError::EmptyMap => write!(f, "It seems that the input file is empty"),
			FileParsingError::BadSize => write!(f, "Map size is incorrect."),
			_ => write!(f, "Format Error."),
		}
	}
}

impl Error for FileParsingError {}

pub fn parse_file(arg: String) -> Result<(i64, Vec<Vec<i64>>), Box<dyn Error>> {
	let initial: Result<Vec<Vec<i64>>, _> = arg
		.lines()
		.map(|x| x.split('#').next().unwrap().trim())
		.filter(|x| !x.is_empty())
		.map(|x| x.split_whitespace().map(|x| x.parse::<i64>()).collect())
		.collect();
	let mut initial = initial?;
	let mut size = initial.remove(0);
	if size.len() > 1 {
		return Err(Box::new(FileParsingError::BadSize));
	}
	let size = size.remove(0);
	if size <= 0 || size != initial.len() as i64 {
		return Err(Box::new(FileParsingError::BadSize));
	}
	Ok((size, initial))
}
