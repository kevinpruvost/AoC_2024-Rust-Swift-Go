use core::num;
use std::{clone, iter};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, Lines};
use std::iter::Map;
use regex::Regex;

struct Puzzle {
    stones: Vec<i64>,
    visited: HashMap<(i64, i64), i64>
}

impl Puzzle {
    pub fn new(contents: &str) -> Self {
        let mut obj = Puzzle {
            stones: vec![],
            visited: HashMap::new()
        };

        for nb in contents.split(" ") {
            obj.stones.push(nb.parse().unwrap());
        }

        obj
    }

    pub fn solve_rec(&mut self, stone: i64, i: i64) -> i64 {
        if (i == 75) { return 1; }

        if self.visited.contains_key(&(stone, i)) {
            return self.visited.get(&(stone, i)).unwrap().clone();
        }

        let mut res = 0;
        if (stone == 0) {
            res = self.solve_rec(1, i+1);
        } else {
            let stoneStr = stone.to_string();
            if stoneStr.len() % 2 == 0 {
                let left_part = &stoneStr[0..stoneStr.len()/2];
                let right_part = &stoneStr[stoneStr.len()/2..stoneStr.len()];
                res = self.solve_rec(left_part.parse::<i64>().unwrap(), i+1) + self.solve_rec(right_part.parse::<i64>().unwrap(), i+1);
            } else {
                res = self.solve_rec(stone * 2024, i+1);
            }
        }
        self.visited.insert((stone, i), res);
        return res;
    }

    pub fn solve(&mut self) -> i64 {
        let mut total = 0;
        
        for i in 0..self.stones.len() {
            total += self.solve_rec(self.stones[i], 0);
        }

        total
    }
}

fn load_day11() -> String {
    let mut file = File::open("../inputs/day11.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day11_1() {
    let cnt = load_day11();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day11_2() {
    let cnt = load_day11();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}