extern crate puzzle;

use puzzle::puzzle::Puzzle as Puzzle;

fn main() {

    let tmp_puzzle = Puzzle::new(4,3);
    match tmp_puzzle {
        None => {
            println!("Use not supported puzzle size");
            return;
        },
        _ => {
            let mut puzzle = tmp_puzzle.unwrap();
            assert_eq!(2, puzzle.check_column_conflict(0,vec![5,8,1]));
            // assert_eq!((true, 11), puzzle.cost(vec![1,2,3,6,8,0,5,4,7]));
            puzzle.generate();
            let gen_puzzle:Vec<i8> = puzzle.puzzle.clone();
            let is_good:bool = puzzle.set_puzzle(gen_puzzle);
            if !is_good {
                 println!("Use not supported puzzle size");
                return;
             }
            // let mut sets:Vec<Vec<i8>>;
            // sets = puzzle.search_sets(vec![3,1,2,0]);
            // puzzle.search_solution();
            println!("My first puzzle on Rust with size: {}, {}", puzzle.size_h, puzzle.size_v);
            println!("{:?}", puzzle.puzzle);
            println!("Start: {:?}", puzzle.start);
            println!("Goal: {:?}", puzzle.goal);
        },
    }

}
