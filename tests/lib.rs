extern crate puzzle;

use puzzle::puzzle::Puzzle as Puzzle;

#[test]
fn it_new_puzzle() {
    assert!(Puzzle::new(1,1).is_none());
    assert!(Puzzle::new(11,11).is_none());
    assert!(!Puzzle::new(3,3).is_none());
    let puzzle = Puzzle::new(3,3).unwrap();
    assert_eq!(puzzle.goal.len() as i8, 9);
    assert_eq!(puzzle.start.len() as i8, 9);

}

#[test]
fn it_generate_puzzle() {
    let mut puzzle = Puzzle::new(4,4).unwrap();
    puzzle.generate();
    assert_eq!(puzzle.puzzle.len() as i8, 4*4);
    assert_eq!(puzzle.start.len() as i8, 4*4);

}
#[test]
fn it_set_puzzle() {
    let mut puzzle = Puzzle::new(2,2).unwrap();
    // puzzle.set_puzzle(vec![2,3,1,0]);
    assert!(puzzle.set_puzzle(vec![3,1,2,0]));
    assert!(!puzzle.set_puzzle(vec![3,2,1,0]));
    let mut puzzle = Puzzle::new(3,3).unwrap();
    assert!(!puzzle.set_puzzle(vec![1,2,3,4,5,6,7,8,9,0]));

}

#[test]
fn it_search_sets_test() {
    let mut puzzle = Puzzle::new(2, 2).unwrap();
    assert!(puzzle.set_puzzle(vec![3, 1, 2, 0]));
    let mut sets: Vec<Vec<i8>>;
    sets = puzzle.search_sets(vec![3, 1, 2, 0]);
    assert_eq!(sets.len(), 2);
    assert_eq!(sets[0], vec![3, 0, 2, 1]);
    assert_eq!(sets[1], vec![3, 1, 0, 2]);
    sets = puzzle.search_sets(vec![3, 1, 0, 2]);
    assert_eq!(sets.len(), 2);
    assert_eq!(sets[0], vec![0, 1, 3, 2]);
    assert_eq!(sets[1], vec![3, 1, 2, 0]);
    sets = puzzle.search_sets(vec![0, 1, 3, 2]);
    assert_eq!(sets.len(), 2);
    assert_eq!(sets[0], vec![3, 1, 0, 2]);
    assert_eq!(sets[1], vec![1, 0, 3, 2]);
    sets = puzzle.search_sets(vec![1, 0, 3, 2]);
    assert_eq!(sets.len(), 2);
    assert_eq!(sets[0], vec![1, 2, 3, 0]);
    assert_eq!(sets[1], vec![0, 1, 3, 2]);

    let mut puzzle = Puzzle::new(3, 4).unwrap();
    assert!(puzzle.set_puzzle(vec![8, 10, 6, 2, 9, 4, 11, 5, 1, 3, 7, 0]));
    sets = puzzle.search_sets(vec![8, 10, 6, 2, 9, 4, 11, 5, 1, 3, 7, 0]);
    assert_eq!(sets.len(), 2);
    assert_eq!(sets[0], vec![8, 10, 6, 2, 9, 4, 11, 5, 0, 3, 7, 1]);
    assert_eq!(sets[1], vec![8, 10, 6, 2, 9, 4, 11, 5, 1, 3, 0, 7]);

    sets = puzzle.search_sets(vec![8, 10, 6, 2, 9, 4, 11, 5, 1, 3, 0, 7]);
    assert_eq!(sets.len(), 3);
    assert_eq!(sets[0], vec![8, 10, 6, 2, 9, 4, 11, 0, 1, 3, 5, 7]);
    assert_eq!(sets[1], vec![8, 10, 6, 2, 9, 4, 11, 5, 1, 0, 3, 7]);
    assert_eq!(sets[2], vec![8, 10, 6, 2, 9, 4, 11, 5, 1, 3, 7, 0]);

    sets = puzzle.search_sets(vec![8, 10, 6, 2, 9, 4, 11, 0, 1, 3, 5, 7]);
    assert_eq!(sets.len(), 4);
    assert_eq!(sets[0], vec![8, 10, 6, 2, 0, 4, 11, 9, 1, 3, 5, 7]);
    assert_eq!(sets[1], vec![8, 10, 6, 2, 9, 4, 11, 5, 1, 3, 0, 7]);
    assert_eq!(sets[2], vec![8, 10, 6, 2, 9, 4, 0, 11, 1, 3, 5, 7]);
    assert_eq!(sets[3], vec![8, 10, 6, 2, 9, 4, 11, 1, 0, 3, 5, 7]);
}

#[test]
fn it_check_linear_conflict_test() {
    let puzzle = Puzzle::new(3, 3).unwrap();
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![0,1,5]));
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![1,0,2]));
    assert_eq!(2, puzzle.check_linear_conflict(0,vec![3,1,2]));
    assert_eq!(2, puzzle.check_linear_conflict(0,vec![1,3,2]));
    assert_eq!(2, puzzle.check_linear_conflict(0,vec![2,3,1]));
    assert_eq!(2, puzzle.check_linear_conflict(1,vec![5,4,1]));
    assert_eq!(2, puzzle.check_linear_conflict(1,vec![4,6,5]));
    assert_eq!(2, puzzle.check_linear_conflict(2,vec![8,7,1]));
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![1,3,0]));
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![4,6,5]));
    assert_eq!(0, puzzle.check_linear_conflict(1,vec![0,9,8]));
    let puzzle = Puzzle::new(4, 4).unwrap();
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![1,2,3,4]));
    assert_eq!(2, puzzle.check_linear_conflict(0,vec![1,3,2,4]));
    assert_eq!(2, puzzle.check_linear_conflict(1,vec![5,6,8,7]));
    assert_eq!(2, puzzle.check_linear_conflict(1,vec![0,8,7,6]));
    assert_eq!(2, puzzle.check_linear_conflict(2,vec![9,10,12,11]));
    assert_eq!(2, puzzle.check_linear_conflict(3,vec![0,14,13,15]));
    assert_eq!(0, puzzle.check_linear_conflict(2,vec![9,0,10,6]));
    let puzzle = Puzzle::new(4, 3).unwrap();
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![0,1,5,9]));
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![1,2,5,4]));
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![10,3,9,4]));
    assert_eq!(2, puzzle.check_linear_conflict(0,vec![1,4,5,3]));
    assert_eq!(2, puzzle.check_linear_conflict(0,vec![4,5,0,1]));
    assert_eq!(0, puzzle.check_linear_conflict(1,vec![0,4,3,1]));
    assert_eq!(0, puzzle.check_linear_conflict(1,vec![5,6,9,7]));
    assert_eq!(2, puzzle.check_linear_conflict(1,vec![6,7,9,5]));
    assert_eq!(2, puzzle.check_linear_conflict(1,vec![0,6,5,9]));
    assert_eq!(0, puzzle.check_linear_conflict(2,vec![0,10,11,5]));
    assert_eq!(0, puzzle.check_linear_conflict(2,vec![9,1,11,5]));
    assert_eq!(2, puzzle.check_linear_conflict(2,vec![12,0,11,5]));
    assert_eq!(2, puzzle.check_linear_conflict(2,vec![5,0,12,11]));
    let puzzle = Puzzle::new(5, 3).unwrap();
    assert_eq!(0, puzzle.check_linear_conflict(0,vec![9,1,3,5,0]));
    assert_eq!(2, puzzle.check_linear_conflict(0,vec![0,3,2,5,9]));

}

#[test]
fn it_check_column_conflict_test() {
    let puzzle = Puzzle::new(3, 3).unwrap();
    assert_eq!(0, puzzle.check_column_conflict(0, vec![1, 4, 6]));
    assert_eq!(0, puzzle.check_column_conflict(0, vec![1, 0, 7]));
    assert_eq!(0, puzzle.check_column_conflict(0, vec![8, 2, 6]));
    assert_eq!(0, puzzle.check_column_conflict(2, vec![4, 1, 7]));
    assert_eq!(2, puzzle.check_column_conflict(0, vec![1, 7, 4]));
    assert_eq!(2, puzzle.check_column_conflict(0, vec![7, 1, 0]));
    assert_eq!(2, puzzle.check_column_conflict(1, vec![2, 8, 5]));
    assert_eq!(2, puzzle.check_column_conflict(1, vec![0, 5, 2]));
    assert_eq!(2, puzzle.check_column_conflict(2, vec![0,6, 3]));
    assert_eq!(2, puzzle.check_column_conflict(2, vec![6,7, 3]));
    let puzzle = Puzzle::new(4, 4).unwrap();
    assert_eq!(0, puzzle.check_column_conflict(0,vec![1,5,9,13]));
    assert_eq!(2, puzzle.check_column_conflict(0,vec![1,9,5,4]));
    assert_eq!(2, puzzle.check_column_conflict(1,vec![10,6,1,2]));
    assert_eq!(2, puzzle.check_column_conflict(1,vec![2,0,14,10]));
    assert_eq!(2, puzzle.check_column_conflict(2,vec![3,7,15,11]));
    assert_eq!(2, puzzle.check_column_conflict(3,vec![8,4,0,1]));
    assert_eq!(0, puzzle.check_column_conflict(2,vec![0,3,11,6]));
    let puzzle = Puzzle::new(4, 3).unwrap();
    assert_eq!(0, puzzle.check_column_conflict(0,vec![1,5,9]));
    assert_eq!(0, puzzle.check_column_conflict(1,vec![2,6,10]));
    assert_eq!(0, puzzle.check_column_conflict(1,vec![6,1,10]));
    assert_eq!(2, puzzle.check_column_conflict(0,vec![5,8,1]));
    assert_eq!(0, puzzle.check_column_conflict(1,vec![5,8,1]));
    assert_eq!(2, puzzle.check_column_conflict(2,vec![7,1,3]));

}