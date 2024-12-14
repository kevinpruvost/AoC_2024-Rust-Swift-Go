use core::num;
use std::{clone, iter};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, Lines};
use std::iter::Map;
use regex::Regex;

struct Puzzle {
    map: Vec<Vec<char>>,
    visited: HashSet<(i32, i32)>
}

impl Puzzle {
    pub fn new(contents: &str) -> Self {
        let mut obj = Puzzle {
            map: contents
                .lines()
                .map(|x| x.chars().collect())
                .collect(),
            visited: HashSet::new()
        };
        obj
    }

    fn solve_plot1(&mut self, ch: char, pos: (i32, i32)) -> (i64, i64) {
        if self.visited.contains(&pos) {
            return (0, 0);
        }
        self.visited.insert((pos.0, pos.1));
        let mut area = 1;
        let mut perim = 4;
        let leftPos = (pos.0, pos.1-1);
        let topPos = (pos.0-1, pos.1);
        let rightPos = (pos.0, pos.1+1);
        let bottomPos = (pos.0+1, pos.1);

        // -1 because each square around will remove a border
        for &pos in [leftPos, topPos, rightPos, bottomPos].iter() {
            if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.map.len() as i32 || pos.1 >= self.map[0].len() as i32 {
                continue;
            }
            if let Some(row) = self.map.get(pos.0 as usize) {
                if let Some(&cell) = row.get(pos.1 as usize) {
                    if cell == ch {
                        let (resArea, resPerim) = self.solve_plot(ch, pos);
                        area += resArea;
                        perim += resPerim - 1;
                    }
                }
            }
        }
        (area, perim)
    }

    pub fn solve1(&mut self) -> i64 {
        let mut total = 0;
        let map = self.map.clone();
        for i in 0..map.len() {
            for (j, ch) in map[i].iter().enumerate() {
                let pos = (i as i32, j as i32);
                let (area, perim) = self.solve_plot(*ch, pos);
                total += area * perim;
                //println!("{}: {} * {} = {}", ch, area, perim, area * perim);
            }
        }
        total
    }

    fn solve_plot(&mut self, ch: char, pos: (i32, i32)) -> (i64, i64) {
        if self.visited.contains(&pos) {
            return (0, 0);
        }
        self.visited.insert((pos.0, pos.1));
        let mut area = 1;
        let mut perim = 4;
        let leftPos = (pos.0, pos.1-1);
        let topPos = (pos.0-1, pos.1);
        let rightPos = (pos.0, pos.1+1);
        let bottomPos = (pos.0+1, pos.1);

        // -1 because each square around will remove a border
        for &pos in [leftPos, topPos, rightPos, bottomPos].iter() {
            if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.map.len() as i32 || pos.1 >= self.map[0].len() as i32 {
                continue;
            }
            if let Some(row) = self.map.get(pos.0 as usize) {
                if let Some(&cell) = row.get(pos.1 as usize) {
                    if cell == ch {
                        let (resArea, resPerim) = self.solve_plot(ch, pos);
                        area += resArea;
                        perim += resPerim - 1;
                    }
                }
            }
        }
        (area, perim)
    }

    pub fn solve(&mut self) -> i64 {
        let mut total = 0;
        let map = self.map.clone();
        for i in 0..map.len() {
            for (j, ch) in map[i].iter().enumerate() {
                let pos = (i as i32, j as i32);
                let (area, perim) = self.solve_plot(*ch, pos);
                total += area * perim;
                //println!("{}: {} * {} = {}", ch, area, perim, area * perim);
            }
        }
        total
    }
}

fn load_day12() -> String {
    let mut file = File::open("../inputs/day12.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day12_1() {
    let cnt = load_day12();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day12_2() {
    let cnt = load_day12();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}