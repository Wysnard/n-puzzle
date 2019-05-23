use npuzzle::Heuristique;
use npuzzle::NPuzzle;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn file_input() -> Result<Option<String>, Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let res = match args.next() {
        Some(arg) => arg,
        None => return Ok(None),
    };

    let res = fs::read_to_string(res)?;
    Ok(Some(res))
}

fn main() -> Result<(), Box<dyn Error>> {
    let heuristique = Heuristique::Manhattan;
    let f = match file_input()? {
        Some(arg) => arg,
        None => return Ok(()),
    };
    println!("{:?}", f);

    let mut puzzle = NPuzzle::new(f, heuristique).unwrap_or_else(|err| {
        eprintln!("Problem with the format of the map : {}", err);
        process::exit(1);
    });
    puzzle.run();
    Ok(())
}
