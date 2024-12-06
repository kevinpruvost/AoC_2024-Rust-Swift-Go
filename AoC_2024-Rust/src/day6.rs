use core::num;
use std::clone;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::Map;
use regex::Regex;

struct PuzzleMap {
    values: Vec<Vec<char>>,
    guardPosition: (usize, usize)
}

impl PuzzleMap {
    pub fn new(contents: &String) -> Self {
        let mut obj: PuzzleMap = PuzzleMap{ values: Vec::new(), guardPosition: (0, 0) };

        for (i, line) in contents.lines().enumerate() {
            obj.values.push(Vec::new());
            for (j, ch) in line.chars().enumerate() {
                obj.values[i].push(ch);
                if ch == '^' {
                    obj.guardPosition = (i, j);
                }
            }
        }
        return obj
    }

    pub fn solve1(&self) -> i32 {
        let mut num = 1; // 1 for the final case
        let mut guardPos = self.guardPosition.clone();
        let mut map = self.values.clone();
        // 0 = down, 1 = right, 2 = up, 3 = left
        let mut direction = 2;
        while true {
            let mut newPos = (guardPos.0 as i32, guardPos.1 as i32);
            match direction {
                0 => {newPos.0 += 1;},
                1 => { newPos.1 -= 1; },
                2 => { newPos.0 -= 1; },
                3 => { newPos.1 += 1; },
                _ => panic!("wrong")
            }
            if newPos.0 < 0 || newPos.1 < 0 || newPos.0 >= map.len() as i32 || newPos.1 >= map[0].len() as i32 {
                break;
            }
            if map[newPos.0 as usize][newPos.1 as usize] == '#' {
                direction = (direction + 1) % 4;
            } else {
                if map[guardPos.0][guardPos.1] != 'X' {
                    num += 1;
                }
                map[guardPos.0][guardPos.1] = 'X';
                guardPos = (newPos.0 as usize, newPos.1 as usize);
            }
        }
        num
    }

    pub fn solve_help(&self, map_to_test: &Vec<Vec<char>>) -> i32 {
        let mut guardPos = self.guardPosition.clone();
        let mut map = map_to_test.clone();
        // 0 = down, 1 = right, 2 = up, 3 = left
        let mut direction = 2;
        let mut axis = false;
        while true {
            let mut newPos = (guardPos.0 as i32, guardPos.1 as i32);
            match direction {
                0 => {newPos.0 += 1;},
                1 => { newPos.1 -= 1; },
                2 => { newPos.0 -= 1; },
                3 => { newPos.1 += 1; },
                _ => panic!("wrong")
            }
            if newPos.0 < 0 || newPos.1 < 0 || newPos.0 >= map.len() as i32 || newPos.1 >= map[0].len() as i32 {
                break;
            }
            if map[newPos.0 as usize][newPos.1 as usize] == '#' {
                if map[guardPos.0 as usize][guardPos.1 as usize] == '+' {
                    return 1;
                }
                direction = (direction + 1) % 4;
                axis = true;
            } else {
                map[guardPos.0][guardPos.1] = 'X';
                if axis {
                    axis = false;
                    map[guardPos.0][guardPos.1] = '+';
                }
                guardPos = (newPos.0 as usize, newPos.1 as usize);
            }
        }
        return 0;
    }

    pub fn solve(&self) -> i32 {
        let mut num = 0;
        let mut map_to_test = self.values.clone();

        for i in 0..map_to_test.len() {
            for j in 0..map_to_test.len() {
                if map_to_test[i][j] == '#' || map_to_test[i][j] == '^' { continue; }
                map_to_test[i][j] = '#';
                num += self.solve_help(&map_to_test);
                map_to_test[i][j] = '.';
            }
        }

        num
    }
}



fn load_day6() -> String {
    let mut file = File::open("../inputs/day6_1.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day6_1() {
    let cnt = load_day6();
    let puzzleMap: PuzzleMap = PuzzleMap::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day6_2() {
    let cnt = load_day6();
    let puzzleMap: PuzzleMap = PuzzleMap::new(&cnt);

    println!("{}", puzzleMap.solve())
}