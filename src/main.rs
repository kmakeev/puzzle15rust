extern crate puzzle;

use puzzle::puzzle::Puzzle as Puzzle;
use std::env;
use std::process::exit;

const USAGE: &str = "\
Usage: puzzle <horisontal puzzle size> <vertical puzzle size>

The horizontal and vertical puzzle size must be greater than 1 and less than 10.
Finding a puzzle solution with a size of 4 or higher may take some time.
";

fn usage() -> ! {
    println!("{}", USAGE);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut size_h:i8;
    let mut size_v:i8;

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
    let tmp_puzzle = Puzzle::new(size_h,size_v);
    match tmp_puzzle {
        None => {
            println!("Use not supported puzzle size");
            usage();
        },
        _ => {
            let mut puzzle = tmp_puzzle.unwrap();
            // assert_eq!((true, 16), puzzle.cost(vec![1,2,8,6,3,4,5,7,0]));
            puzzle.generate();
            let gen_puzzle:Vec<i8> = puzzle.puzzle.clone();
            let is_good:bool = puzzle.set_puzzle(gen_puzzle);
            if !is_good {
                 println!("Use not supported puzzle size");
                usage();
             }
            println!("My puzzle on size: {}, {}", puzzle.size_h, puzzle.size_v);
            for i in 0..size_v {
                println!("{:?}", puzzle.puzzle[(size_h*i) as usize..(size_h*(i+1)) as usize].to_vec());
            }
            let mut result = puzzle.search_solution();
            println!();
            println!("-------RESULT-------");
            result.reverse();
            for step in result {
                println!("Step - {:?}", step.set.g);
                for i in 0..size_v {
                    println!("{:?}", step.set.position[(size_h*i) as usize..(size_h*(i+1)) as usize].to_vec());
                }
                //println!("set -: {:?} witsh hash - {:?}, prev - {:?}", step.set, step.hash_current, step.hash_prev);
            }

        },
    }

}
