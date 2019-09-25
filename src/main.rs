use npuzzle::goal::Goal;
use npuzzle::heuristique::Heuristique;
use npuzzle::NPuzzle;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn input_manager() -> Result<NPuzzle, Box<dyn Error>> {
    let mut goal: Goal = Goal::Snail;
    let mut heuristique: Heuristique = Heuristique::Manhattan;
    let mut input: String = "".to_string();
    let mut max_iteration: u64 = 10_000_000;
    let mut args: Vec<String> = env::args().skip(1).rev().collect();

    while let Some(arg) = args.pop() {
        match &arg as &str {
            "--input" | "-i" => {
                if let Some(a) = args.pop() {
                    input = fs::read_to_string(a)?;
                } else {
                    println!("No file input given");
                    process::exit(1);
                }
            }
            "--heuristique" | "-h" => {
                if let Some(a) = args.pop() {
                    heuristique = Heuristique::parse(a);
                } else {
                    println!("No heuristique given");
                    process::exit(1);
                }
            }
            "--goal" | "-o" => {
                if let Some(a) = args.pop() {
                    goal = match &a.to_lowercase() as &str {
                        "custom" | "cstm" => {
                            if let Some(b) = args.pop() {
                                Goal::parse(a, b)
                            } else {
                                println!("Goal File Missing");
                                process::exit(1);
                            }
                        }
                        "std" | "standard" | "snail" => Goal::parse(a, "".to_string()),
                        _ => {
                            println!("Goal arguments missing or invalid");
                            process::exit(1);
                        }
                    }
                } else {
                    println!("No goal given");
                    process::exit(1);
                }
            }
            "--iteration" | "-n" => {
                max_iteration = match args.pop() {
                    Some(v) => {
                        if let Ok(n) = v.parse::<u64>() {
                            n
                        } else {
                            println!("Cannot parse the number of iterations");
                            process::exit(1);
                        }
                    }
                    _ => {
                        println!("number of iteration missing");
                        process::exit(1);
                    }
                };
            }
            _ => {
                println!("Argument not recognized");
                process::exit(1);
            }
        };
    }
    let puzzle = NPuzzle::new(input, heuristique, goal, max_iteration).unwrap_or_else(|err| {
        eprintln!("Problem with the format of the map : {}", err);
        process::exit(1);
    });
    Ok(puzzle)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = input_manager()?;
    puzzle.run();
    Ok(())
}
