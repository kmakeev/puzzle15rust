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