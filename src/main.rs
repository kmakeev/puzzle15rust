extern crate puzzle;

use puzzle::puzzle::Puzzle as Puzzle;

fn main() {

    let tmp_puzzle = Puzzle::new(2,2);
    match tmp_puzzle {
        None => {
            println!("Use not supported puzzle size");
            return;
        },
        _ => {
            let mut puzzle = tmp_puzzle.unwrap();
            // puzzle.generate();
            //let gen_puzzle:Vec<i8> = puzzle.puzzle.clone();
            let is_good:bool = puzzle.set_puzzle((vec![3,1,2,0]));
            if !is_good {
                 println!("Use not supported puzzle size");
                return;
             }
            let mut sets:Vec<Vec<i8>>;
            sets = puzzle.search_sets(vec![3,1,2,0]);
            // puzzle.search_solution();
            println!("My first puzzle on Rust with size: {}, {}", puzzle.size_h, puzzle.size_v);
            println!("{:?}", puzzle.puzzle);
            println!("Start: {:?}", puzzle.start);
            println!("Goal: {:?}", puzzle.goal);
        },
    }

}
