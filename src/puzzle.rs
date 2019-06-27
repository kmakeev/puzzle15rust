extern crate rand;
// extern crate tuple;

use rand::Rng;
use std::convert::TryFrom;
use std::fmt;
use std::sync::Arc;
// use test::Options;
// use tuple::*;

pub struct Point {
    pub h: i8,
    pub v: i8,
}

struct Set {
    g: u32,
    h: u32,
    f: u32,
    prev: Vec<i8>,
    goal: Vec<i8>,
}

pub struct Puzzle {
    pub size_h: i8,
    pub size_v: i8,
    pub puzzle: Vec<i8>,
    // pub start: Vec<Point>,
    // pub goal: Vec<Point>,
    pub start: Vec<i8>,
    pub goal: Vec<i8>,
}


impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.h, self.v)
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[g - {}, h - {}, f - {}, prev - {:?}, goal - {:?}]", self.g, self.h, self.f,
                                                                        self.prev, self.goal)
    }
}

impl Puzzle {
    pub fn new(size_h: i8, size_v: i8) -> Option<Puzzle> {
        if size_h > 1 && size_h < 10 && size_v > 1 && size_v < 10 {
            let puzzle: Vec<i8> = vec![];
            // let mut start: Vec<Point> = vec![];
            // let mut goal: Vec<Point> = vec![];
            let mut start: Vec<i8> = vec![];
            let mut goal: Vec<i8> = vec![];
            //for i in 0..size_v {
            //    for j in 0..size_v {
            //        //goal.push(Point { h: i, v: j })
            //    }
            //}
            for i in 1..size_v*size_h {
                start.push(i);
                goal.push(i);
            }
            start.push(0);
            goal.push(0);
            // assert_eq!(goal.len() as i8, size_h*size_v);
            // for _i in 0..size_h*size_v {
            //    start.push(Point{h:0, v:0})
            // }
            // assert_eq!(start.len() as i8, size_h*size_v);
            // println!("My null tuple {:#?}", start);
            Some(Puzzle{size_h, size_v, puzzle, start, goal})
        } else {
            None
        }
    }
    pub fn generate(& mut self) {
        let mut is_generate: bool = true;
        let mut x: i8;
        //let mut i: i8 = 0;
        while is_generate {
            self.puzzle = vec![];
            while self.puzzle.len() < usize::try_from(self.size_h * self.size_v - 1).unwrap() {
                x = rand::thread_rng().gen_range(1,self.size_h * self.size_v);
                if self.puzzle.iter().position(|&r| r == x) == None {
                    self.puzzle.push(x);
                    // self.start[i as usize] = x;
                    //self.start[(x-1) as usize] = Point{h:i / self.size_h, v:i % self.size_v};
                    //i += 1;
                }
            }

            let mut summ:i32 = 0;
            let mut _b:i32 = 0;
            for (c, i) in self.puzzle.iter().enumerate() {
                if *i != 0 {
                    _b = 0;
                    let s_puzzle:Vec<i8> = self.puzzle[c..self.puzzle.len()].to_vec();
                    for j in s_puzzle.iter() {
                        if *j < *i && *j!=0 {
                            _b += 1;
                        }
                    }
                    summ = summ + _b;
                }
            }
            if summ % 2 == 0 {
                is_generate = false;
            }
        }
        self.puzzle.push(0);
        self.start = self.puzzle.clone();
        // self.start.push(0);
        //self.start[self.puzzle.len()-1] = Point{h:self.size_h-1, v:self.size_v-1};
        // assert_eq!(self.puzzle.len() as i8, self.size_h*self.size_v);
        // assert_eq!(self.start.len() as i8, self.size_h*self.size_v);
    }
    pub fn set_puzzle(& mut self, puz:Vec<i8>) -> bool {
        println!("H in set puzzle: {:?}", puz.len() as i8 / self.size_v);
        let mut is_good:bool = false;
        if puz.len() as i8 % (self.size_v*self.size_h) !=0 {
            is_good
        } else {
            self.puzzle = puz.clone();
            self.start = puz.clone();
            let mut summ:i32 = 0;
            let mut _b:i32 = 0;
            for (c, i) in self.puzzle.iter().enumerate() {
                if *i != 0 {
                    _b = 0;
                    let s_puzzle:Vec<i8> = self.puzzle[c..self.puzzle.len()].to_vec();
                    for j in s_puzzle.iter() {
                        if *j < *i && *j!=0 {
                            _b += 1;
                        }
                    }
                    summ = summ + _b;
                }
            }
            if summ % 2 == 0 {
                is_good = true;
            }
            is_good
        }

    }
    pub fn search_solution(& self) {
        let h:i8 = self.size_h;
        let v:i8 = self.size_v;
        let goal:Vec<i8> = self.goal.clone();
        let start:Vec<i8> = self.start.clone();
        // println!("Start in search: {:?}", start);
        // println!("Goal in search: {:?}", goal);
        let mut open_sets:Vec<Set> = vec![];
        let mut close_sets:Vec<Set> = vec![];
        let mut new_sets:Vec<i8>;
        let mut g:u32 = 0;
        let mut h:u32 = 100;
        let mut f:u32 = g+h;
        let first_set:Set = Set{g:g, h:h, f:f, prev:start, goal:goal};
        println!("First set in search: {:?}", first_set);
        open_sets.push(first_set);
        println!("Open_sets in search: {:?}", open_sets);

    }

    pub fn search_sets(& self, prev: Vec<i8>) -> Vec<Vec<i8>> {
        if prev.len()  != usize::try_from(self.size_h * self.size_v).unwrap() {
            vec![]
        } else {
            let mut tmp: Vec<i8> = vec![];
            let mut result: Vec<Vec<i8>> = vec![];
            let mut c:i8;
            let pos:i8;
            let pos_opt:Option<usize>;

                pos_opt = prev.iter().position(|&r| r == 0);

               if pos_opt == None {
                result
            } else {
                   pos = pos_opt.unwrap() as i8;
                   if pos / self.size_h > 0 {
                       tmp = prev.clone();
                       c = prev[(pos - self.size_h) as usize];
                       tmp.remove((pos - self.size_h) as usize);
                       tmp.insert((pos - self.size_h) as usize, 0);
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   if (pos + self.size_h) < (self.size_v*self.size_h) {
                       tmp = prev.clone();
                       c = prev[(pos + self.size_h) as usize];
                       tmp.remove((pos + self.size_h) as usize);
                       tmp.insert((pos + self.size_h) as usize, 0);
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   if pos % self.size_v > 0 {
                       tmp = prev.clone();
                       c = prev[(pos - 1) as usize];
                       tmp.remove((pos - 1) as usize);
                       tmp.insert((pos - 1) as usize, 0);
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   if (pos % self.size_h) < (self.size_h-1) {
                       tmp = prev.clone();
                       c = prev[(pos + 1) as usize];
                       tmp.remove((pos + 1) as usize);
                       tmp.insert((pos + 1) as usize, 0);
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   result
               }
        }
    }

}