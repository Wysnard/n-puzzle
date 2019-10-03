use npuzzle::algorithm::*;
use npuzzle::goal::Goal;
use npuzzle::heuristique::Heuristique;
use npuzzle::utils::*;
use npuzzle::NPuzzle;
use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::time::SystemTime;

fn input_manager() -> Result<NPuzzle, Box<dyn Error>> {
    let mut goal: Goal = Goal::Snail;
    let mut heuristique: Heuristique = Heuristique::Manhattan;
    let mut algorithm: Algorithm = Algorithm::AStar;
    let mut strategy: String = "std".to_string();
    let mut input: String = "".to_string();
    let mut max_iteration: u64 = 10_000_000;
    let mut debug: bool = false;
    let mut args: Vec<String> = env::args().skip(1).rev().collect();
    let mut thread: usize = 1;

    while let Some(arg) = args.pop() {
        match &arg as &str {
            "--debug" | "-d" => debug = true,
            "--input" | "-i" => {
                if let Some(a) = args.pop() {
                    if let Ok(a) = a.parse::<usize>() {
                        if a < 6 && a > 0 {
                            input = creat_new_rand(a);
                        } else {
                            println!("Map size has to be between 1 and 5 included");
                            process::exit(0);
                        }
                    } else {
                        input = fs::read_to_string(a)?;
                    }
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
            "--algorithm" | "--algo" | "-a" => {
                if let Some(a) = args.pop() {
                    algorithm = Algorithm::parse(a);
                } else {
                    println!("No algorithm given");
                    process::exit(1);
                }
            }
            "--strategy" | "--strat" | "-s" => {
                if let Some(a) = args.pop() {
                    strategy = a;
                } else {
                    println!("No strategy given");
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
            "--thread" | "--thrd" | "--th" | "-t" => {
                if let Some(a) = args.pop() {
                    if let Ok(a) = a.parse::<usize>() {
                        if a > 0 && a < 5 {
                            thread = a
                        } else {
                            println!("Put between 1 and 4 threads");
                            process::exit(1);
                        }
                    } else {
                        println!("Wrong number of thread");
                        process::exit(1);
                    }
                } else {
                    println!("Give the number of thread you want");
                    process::exit(1);
                }
            }
            _ => {
                println!("Argument not recognized");
                process::exit(1);
            }
        };
    }
    let puzzle = NPuzzle::new(
        input,
        heuristique,
        algorithm,
        strategy,
        goal,
        max_iteration,
        debug,
        2usize,
    )
    .unwrap_or_else(|err| {
        eprintln!("Problem with the format of the map : {}", err);
        process::exit(1);
    });
    Ok(puzzle)
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = SystemTime::now();
    match input_manager() {
        Ok(mut puzzle) => puzzle.run(),
        Err(e) => println!("Sorry, You have a mental disease : {}", e),
    }
    match now.elapsed() {
        Ok(elapsed) => {
            let dur = elapsed.as_millis();
            println!("Time Complexity : {}.{} seconds", dur / 1000, dur % 1000);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}
