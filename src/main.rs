extern crate puzzle;

use puzzle::puzzle::Puzzle as Puzzle;

fn main() {

    let tmp_puzzle = Puzzle::new(3,3);
    match tmp_puzzle {
        None => {
            println!("Use not supported puzzle size");
            return;
        },
        _ => {
            let mut puzzle = tmp_puzzle.unwrap();
            // assert_eq!((true, 16), puzzle.cost(vec![1,2,8,6,3,4,5,7,0]));
            puzzle.generate();
            let gen_puzzle:Vec<i8> = puzzle.puzzle.clone();
            let is_good:bool = puzzle.set_puzzle(gen_puzzle);
            if !is_good {
                 println!("Use not supported puzzle size");
                return;
             }
            println!("My first puzzle on Rust with size: {}, {}", puzzle.size_h, puzzle.size_v);
            println!("{:?}", puzzle.puzzle);
            let one_result = puzzle.search_solution();
            println!("-------RESULT-------");
            for step in one_result {
                println!("set -: {:?} witsh hash - {:?}, prev - {:?}", step.set, step.hash_current, step.hash_prev);
            }

        },
    }

}
