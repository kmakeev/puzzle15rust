extern crate puzzle;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use puzzle::puzzle::Puzzle as Puzzle;
use std::env;
use std::process::exit;
use csv::Writer;
// use serde::ser::{Serialize};

const USAGE: &str = "\
Usage: puzzle <horisontal puzzle size> <vertical puzzle size> optional <number of solutions to save them to a file>

The horizontal and vertical puzzle size must be greater than 1 and less than 10.
Finding a puzzle solution with a size of 4 or higher may take some time.
";
const FILENAME: &str = "output.csv";


#[derive(Serialize)]
struct Row {
    size_h:i8,
    size_v:i8,
    position: String,
    states: String,
    result: i8,

}

fn usage() -> ! {
    println!("{}", USAGE);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut size_h:i8;
    let mut size_v:i8;
    let mut count_solutions:i32 = 0;

    if args.len() < 3 {
        usage()
    }
    let obj_size_h = &args[1].parse::<i8>();
    match obj_size_h {
        Err(n) => {
            usage();
        },
        Ok(r) => {
            size_h = *r;
        }

    }
    let obj_size_v = &args[2].parse::<i8>();
    match obj_size_v {
        Err(n) => {
            usage();
        },
        Ok(r) => {
            size_v= *r;
        }

    }
    if args.len() == 3 {
        let tmp_puzzle = Puzzle::new(size_h, size_v);
        match tmp_puzzle {
            None => {
                println!("Use not supported puzzle size");
                usage();
            },
            _ => {
                let mut puzzle = tmp_puzzle.unwrap();
                // assert_eq!((true, 16), puzzle.cost(vec![1,2,8,6,3,4,5,7,0]));
                puzzle.generate();
                let gen_puzzle: Vec<i8> = puzzle.puzzle.clone();
                let is_good: bool = puzzle.set_puzzle(gen_puzzle);
                if !is_good {
                    println!("Use not supported puzzle size");
                    usage();
                }
                println!("My puzzle on size: {}, {}", puzzle.size_h, puzzle.size_v);
                for i in 0..size_v {
                    println!("{:?}", puzzle.puzzle[(size_h * i) as usize..(size_h * (i + 1)) as usize].to_vec());
                }
                let mut result = puzzle.search_solution();
                println!();
                println!("-------RESULT-------");
                result.reverse();
                for step in result {
                    println!("Step - {:?}", step.set.g);
                    for i in 0..size_v {
                        println!("{:?}", step.set.position[(size_h * i) as usize..(size_h * (i + 1)) as usize].to_vec());
                    }
                    //println!("set -: {:?} witsh hash - {:?}, prev - {:?}", step.set, step.hash_current, step.hash_prev);
                }
            },
        }
    } else if args.len() == 4 {
        let obj_number = &args[3].parse::<i32>();
        match obj_number {
            Err(n) => {
                println!("Impossible to determine the required amount of solutions, enter an integer greater than 0. Specified - {}", &args[3]);
                usage();
            },
            Ok(r) => {
                count_solutions = *r;
            }
        }

        let tmp_puzzle = Puzzle::new(size_h, size_v);
        match tmp_puzzle {
            None => {
                println!("Use not supported puzzle size");
                usage();
            },
            _ => {
                let mut wtr = Writer::from_path(FILENAME).unwrap();
                let mut puzzle = tmp_puzzle.unwrap();
                for i in 0..count_solutions {
                    puzzle.generate();
                    print!("\r Generete and search solution in  puzzle on size: {}, {} count - {}", puzzle.size_h, puzzle.size_v, i);
                    let mut result = puzzle.search_solution();
                    result.reverse();
                    for (j, step) in result[..result.len()-1].iter().enumerate() {
                        let position = step.set.position.clone();
                        let all_sets = puzzle.search_all_sets(position.clone());
                        let set = result[(j+1) as usize].set.position.clone();
                        let mut result:i8 = match set {
                            _ if set == all_sets[0] => 0,
                            _ if set == all_sets[1] => 1,
                            _ if set == all_sets[2] => 2,
                            _ if set == all_sets[3] => 3,
                            _ => -1,
                        };
                        match wtr.serialize(Row{
                                            size_h: size_h,
                                            size_v: size_v,
                                            position: format!("{:?}", puzzle.get_points(position.clone())),
                                            states: format!("{:?}", puzzle.get_states(position.clone())),
                                            result: result,
                        }) {
                            Err(err) => {println!("Some error - {}", err)},
                            Ok(t) => {}
                        };
                    }
                }
                match wtr.flush() {
                    Err(err) => {
                        println!();
                        println!("Some error on write ti file- {}", err)
                    },
                    Ok(t) => {println!();
                        println!("File saved - {}", FILENAME)
                    }
                }
            }
        }
    } else {
        usage()
    }

}
