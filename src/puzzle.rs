extern crate rand;

use rand::Rng;
use std::convert::TryFrom;
use std::fmt;
use std::sync::Arc;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Point {
    pub h: i8,
    pub v: i8,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Set {
    pub g: u32,
    pub h: u32,
    pub f: u32,
    pub position: Vec<i8>,
}


fn calculate_hash<T: Hash> (t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Clone)]
pub struct Step {
    pub hash_prev: u64,
    pub set: Set,
    pub hash_current: u64,
}

pub struct Puzzle {
    pub size_h: i8,
    pub size_v: i8,
    pub puzzle: Vec<i8>,
    // pub start: Vec<Point>,
    // pub goal: Vec<Point>,ƒ
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
        write!(f, "[g - {}, h - {}, f - {}, position - {:?}]", self.g, self.h, self.f,
                                                                        self.position)
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
            Some(Puzzle{size_h, size_v, puzzle, start, goal})
        } else {
            None
        }
    }
    pub fn generate(& mut self) {
        let mut is_generate: bool = true;
        let mut x: i8;
        while is_generate {
            self.puzzle = vec![];
            while self.puzzle.len() < usize::try_from(self.size_h * self.size_v - 1).unwrap() {
                x = rand::thread_rng().gen_range(1,self.size_h * self.size_v);
                if self.puzzle.iter().position(|&r| r == x) == None {
                    self.puzzle.push(x);
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
    }
    pub fn set_puzzle(& mut self, puz:Vec<i8>) -> bool {
        // println!("H in set puzzle: {:?}", puz.len() as i8 / self.size_v);
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
    pub fn search_solution(& self) -> Vec<Step> {
        let h:i8 = self.size_h;
        let v:i8 = self.size_v;
        let mut path_map:Vec<Step> = vec![];
        let mut open_sets:Vec<Step> = vec![];
        let mut close_sets:Vec<Step> = vec![];
        let mut sets: Vec<Vec<i8>>;
        let mut g:u32 = 0;
        let mut f:u32;
        let (cost, h) = self.cost(self.start.clone());
        f = g+h;
        open_sets.push(Step{hash_prev: 0, set:Set{g:g, h:h, f:f, position:self.start.clone()},
            hash_current: calculate_hash(&Set{g:g, h:h, f:f, position:self.start.clone()})});
        while open_sets.len() !=0 {
            let mut prev = open_sets.pop().unwrap();
            close_sets.push(prev.clone());

            if prev.set.position.clone() == self.goal {
                path_map.push(prev.clone());
                while prev.hash_prev != 0 {
                    let pos_opt = close_sets.iter().position(|r| r.hash_current == prev.clone().hash_prev);
                    prev = close_sets[pos_opt.unwrap()].clone();
                    path_map.push(prev.clone());
                }
                break;
            }
            sets = self.search_sets(prev.set.position.clone());
            for new in sets {
                if close_sets.iter().position(|r| r.set.position == new) != None {
                    // println!("new set - {:?} is in close, continue...", new);
                    continue;
                }
                let mut tentive_g_score:u32 = prev.set.g + 1;
                let mut tentive_is_better:bool = false;
                let pos_opt:Option<usize>;
                pos_opt = open_sets.iter().position(|r| r.set.position == new);
                if pos_opt == None {
                    tentive_is_better = true;
                } else {
                    if tentive_g_score < open_sets[pos_opt.unwrap()].set.g {
                        tentive_is_better = true;
                    }
                }
                if tentive_is_better {
                    let (cost_, h_) = self.cost(new.clone());
                    if cost_ {
                        g = tentive_g_score;
                        f = g+h_;
                        let set:Set = Set{g:g, h:h_, f:f, position:new};
                        let hash = calculate_hash(&set);
                        // println!("Found set -: {:?} witsh hash - {:?}", set, hash);
                        open_sets.push(Step{hash_prev:prev.hash_current, set:set, hash_current:hash})

                    }
                }

            }
            open_sets.sort_by(|a, b| b.set.f.cmp(&a.set.f));
        }

        return path_map;
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
                       tmp[(pos - self.size_h) as usize] = 0;
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   if (pos + self.size_h) < (self.size_v*self.size_h) {
                       tmp = prev.clone();
                       c = prev[(pos + self.size_h) as usize];
                       tmp[(pos + self.size_h) as usize] = 0;
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   if pos % self.size_v > 0 {
                       tmp = prev.clone();
                       c = prev[(pos - 1) as usize];
                       tmp[(pos - 1) as usize] = 0;
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   if (pos % self.size_h) < (self.size_h-1) {
                       tmp = prev.clone();
                       c = prev[(pos + 1) as usize];
                       tmp[(pos + 1) as usize] = 0;
                       tmp[pos as usize] = c;
                       result.push(tmp);
                   }
                   result
               }
        }
    }

    pub fn check_linear_conflict(& self, idx: i8, line: Vec<i8>) -> u32 {
        let mut is_conflict:u32 = 0;
        if line.len() as i8 > self.size_h {
            is_conflict
        } else {
            for (c, j) in line[..line.len()-1].iter().enumerate() {
                if *j > (0 + idx*self.size_h) && (*j <= self.size_h*(idx+1)) {
                    for i in line[c + 1..line.len()].to_vec() {
                        if i > (0 + idx*self.size_h) && (i <= self.size_h*(idx+1)) && *j>i {
                            is_conflict = 2;
                            break;
                        }
                    }
                    if is_conflict > 0 {
                        break;
                    }
                }
            }
            is_conflict
        }
    }

    pub fn check_column_conflict(& self, idx: i8, line: Vec<i8>) -> u32 {
        let mut is_conflict:u32 = 0;
        if line.len() as i8 > self.size_v {
            is_conflict
        } else {
            for (c, j) in line[..line.len()-1].iter().enumerate() {
                if *j % self.size_h == ((idx+1) % self.size_h) {
                    for i in line[c + 1..line.len()].to_vec() {
                        if i!=0 && (i % self.size_h) == ((idx+1) % self.size_h)
                            && *j/self.size_h > i/self.size_h {
                            is_conflict = 2;
                            break;
                        }
                    }
                }
                if is_conflict > 0 {
                    break;
                }
            }
            is_conflict
        }
    }

    pub fn cost(& self, line: Vec<i8>) -> (bool,u32) {
        let mut cost:u32 = 0;
        if line.len() as i8 != self.size_v*self.size_h {
            (false, cost)
        } else {
            for  (c, j) in line.iter().enumerate() {
                if *j != 0 {
                    let mut v: i8 = (((*j-1) / self.size_v) - (c as i8 / self.size_v)).abs();
                    let mut h: i8 = (((*j-1) % self.size_h) - (c as i8 % self.size_h)).abs();
                    cost = cost + v as u32 + h as u32;
                }
            }
            // println!("cost without conflict - {}" , cost);
            if cost > 0 && self.size_v > 2 && self.size_h > 2 {

                // check linear conflict for all lines
                for i in 0..self.size_v {
                        cost = cost + self.check_linear_conflict(i,
                                                                 line[(self.size_v*i) as usize..(self.size_v*i+self.size_h) as usize].to_vec());
                }
                // println!("cost with h line conflict - {}" , cost);
                // check column conflict for all columns
                for i in 0..self.size_h {
                    let mut col: Vec<i8> = vec![];
                    for j in 0..self.size_v {
                        col.push(line[(i + j*self.size_h) as usize]);
                    }
                    cost = cost + self.check_column_conflict(i, col);

                }
                // println!("cost with h+v line conflict - {}" , cost);
                // check last move conflict
                let mut position1: i8;
                let mut position2: i8;
                position1 = line.iter().position(|&r| r == (self.size_h - 1) * self.size_v).unwrap() as i8;
                position2 = line.iter().position(|&r| r == self.size_h * self.size_v - 1).unwrap() as i8;
                // println!("{} , {}", (position2 + 1) % self.size_h, self.size_h*(self.size_v-1));
                if (position2 + 1) % self.size_h != 0 && (position1 < self.size_h*(self.size_v-1)) {
                    // println!("{} , {}", (position2 + 1) % self.size_h, self.size_h*(self.size_v-1));
                    cost +=2;
                }
                // println!("cost with line and last move conflict - {}" , cost);
                // check left top agle on conflict
                if line[1] == 2 && line[self.size_h as usize]==self.size_h +1 && line[0] != 1{
                    // Check use conflict in line or column
                        cost +=2;
                    // }
                }
                // check right top agle on conflict
                if line[(self.size_h -2) as usize] == self.size_h - 1 &&
                    line[(2*self.size_h -1) as usize] == self.size_h*2  &&
                    line[(self.size_h - 1) as usize] != self.size_h {
                    // if line[(self.size_h-1) as usize] < self.size_h {
                        cost +=2;                                               //if not use
                    // }

                }
                // check left bottom  agle on conflict
                if line[(self.size_h*(self.size_v - 2)) as usize] == self.size_h*(self.size_v - 2) &&
                    line[(self.size_h*(self.size_v-1) + 1) as usize] == self.size_h*(self.size_v-1) + 2 &&
                    line[(self.size_h*(self.size_v-1)) as usize] != self.size_h*(self.size_v-1) +1 {
                    // if line[(self.size_h*(self.size_v-1)) as usize] > self.size_h*(self.size_v-1) +1 {  //if not use
                        cost +=2;
                    // }
                }

            }
            // println!("result cost - {}" , cost);
            (true, cost)
        }
    }

}