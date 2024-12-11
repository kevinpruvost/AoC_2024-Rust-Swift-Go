use core::num;
use std::{clone, iter};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, Lines};
use std::iter::Map;
use regex::Regex;


struct FilesystemPair {
    files: u32,
    freeSpace: u32,
    freeFiles: u32
}

struct Puzzle {
    map: Vec<Vec<i32>>,
    zeros: Vec<(usize, usize)>,
    visited1: HashSet<(usize, usize)>,
    visited: HashMap<(usize, usize), i64>
}

impl Puzzle {
    pub fn new(contents: &str) -> Self {
        let mut obj = Puzzle{map: Vec::new(), zeros: Vec::new(), visited1: HashSet::new(), visited: HashMap::new()};

        let mut i = 0;
        for line in contents.lines() {
            let mut row: Vec<i32> = Vec::new();
            let mut j = 0;
            for c in line.chars() {
                if let Some(nb) = c.to_digit(10) {
                    row.push(nb as i32);
                } else {
                    row.push(-1);
                }
                if c == '0' {
                    obj.zeros.push((i, j));
                }
                j += 1;
            }
            obj.map.push(row);
            i += 1;
        }

        obj
    }

    fn solve_rec1(&mut self, i: i32, pos: (usize, usize)) -> i64 {
        if i == 9 {
            self.visited1.insert(pos);
            return 1;
        }

        let mut total = 0;
        for gap in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let newPos = (pos.0 as i32 + gap.0, pos.1 as i32 + gap.1);
            if newPos.0 < 0 || newPos.0 >= self.map.len() as i32 || newPos.1 < 0 || newPos.1 >= self.map[0].len() as i32 {
                continue;
            }
            if self.map[newPos.0 as usize][newPos.1 as usize] == i+1 {
                total += self.solve_rec1(i + 1, (newPos.0 as usize, newPos.1 as usize));
            }
        }

        total
    }

    pub fn solve1(&mut self) -> i64 {
        let mut total = 0;
        for zero in self.zeros.clone().iter() {
            self.solve_rec1(0, *zero);
            total += self.visited.len() as i64;
            self.visited.clear();
        }
        total
    }

    fn solve_rec(&mut self, i: i32, pos: (usize, usize)) -> i64 {
        if i == 9 {
            return 1;
        }
        if self.visited.contains_key(&pos) {
            return self.visited[&pos];
        }

        let mut total = 0;
        for gap in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let newPos = (pos.0 as i32 + gap.0, pos.1 as i32 + gap.1);
            if newPos.0 < 0 || newPos.0 >= self.map.len() as i32 || newPos.1 < 0 || newPos.1 >= self.map[0].len() as i32 {
                continue;
            }
            if self.map[newPos.0 as usize][newPos.1 as usize] == i+1 {
                total += self.solve_rec(i + 1, (newPos.0 as usize, newPos.1 as usize));
            }
        }

        self.visited.insert(pos, total);
        total
    }

    pub fn solve(&mut self) -> i64 {
        let mut total = 0;
        for zero in self.zeros.clone().iter() {
            total += self.solve_rec(0, *zero);
        }
        total
    }
}

fn load_day10() -> String {
    let mut file = File::open("../inputs/day10.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day10_1() {
    let cnt = load_day10();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day10_2() {
    let cnt = load_day10();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}