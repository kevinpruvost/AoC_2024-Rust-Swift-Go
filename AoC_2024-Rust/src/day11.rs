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
        Puzzle {
            stones: contents
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            visited: HashMap::new()
        }
    }

    pub fn solve_rec(&mut self, stone: i64, i: i64) -> i64 {
        if let Some(&result) = self.visited.get(&(stone, i)) {
            return result;
        }

        if i == 75 { return 1; }

        let res = if stone == 0 {
            self.solve_rec(1, i+1)
        } else {
            let digitsCount = i64::ilog10(stone);
            if digitsCount % 2 == 0 {
                let mid = digitsCount / 2;
                let left_part = stone / 10_i64.pow(mid as u32);
                let right_part = stone % 10_i64.pow(mid as u32);
                self.solve_rec(left_part, i+1) + self.solve_rec(right_part, i+1)
            } else {
                self.solve_rec(stone * 2024, i+1)
            }
        };
        self.visited.insert((stone, i), res);
        res
    }

    pub fn solve(&mut self) -> i64 {
        let stones = self.stones.clone();
        stones
            .iter()
            .map(|&stone| self.solve_rec(stone, 0))
            .sum()
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