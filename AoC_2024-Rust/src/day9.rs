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
    pairs: Vec<FilesystemPair>,
}

impl Puzzle {
    pub fn new(contents: &str) -> Self {
        let mut obj = Puzzle{pairs: Vec::new()};

        let mut i = 0;
        for x in (0..contents.len()).step_by(2) {
            if let Some(files) = contents.chars().nth(x).unwrap().to_digit(10) {
                if let Some(parseFreeSpace) = contents.chars().nth(x+1) {
                    if let Some(freeSpace) = parseFreeSpace.to_digit(10) {
                        obj.pairs.push(FilesystemPair { files: files, freeSpace: freeSpace, freeFiles: 0});
                    }
                } else {
                    obj.pairs.push(FilesystemPair { files: files, freeSpace: 0, freeFiles: 0});
                }
                i += 1;
            }
        }

        obj
    }

    pub fn solve1(&mut self) -> i64 {
        let mut count = 0;

        let mut i = 0;
        let mut iterIndex = 0;
        let mut lastIterIndex = self.pairs.len() - 1;
        loop {
            if iterIndex <= lastIterIndex {
                // Counting the already used files
                while self.pairs[iterIndex].files != 0 {
                    count += i * iterIndex as i64;
                    i += 1;
                    self.pairs[iterIndex].files -= 1;
                    //print!("{} ", iterIndex);
                }
                // Moving the last files to the current iteration
                while self.pairs[iterIndex].freeSpace != 0 {
                    while lastIterIndex >= iterIndex && self.pairs[lastIterIndex].files == 0 {
                        lastIterIndex -= 1;
                    }
                    if lastIterIndex < iterIndex {
                        break;
                    }
                    count += i * lastIterIndex as i64;
                    i += 1;
                    self.pairs[iterIndex].freeSpace -= 1;
                    self.pairs[lastIterIndex].files -= 1;
                    //print!("{} ", lastIterIndex);
                }
                iterIndex += 1;
            } else {
                break;
            }
        }

        count
    }

    pub fn solve(&mut self) -> i64 {
        let mut count = 0;

        let mut i = 0;
        for x in 0..self.pairs.len() {
            i += self.pairs[x].freeFiles as i64;
            while self.pairs[x].files != 0 {
                count += i * x as i64;
                //print!("{} ", x as i64);
                i += 1;
                self.pairs[x].files -= 1;
            }
            while self.pairs[x].freeSpace > 0 {
                let mut y = self.pairs.len() - 1;
                while self.pairs[y].files > self.pairs[x].freeSpace || self.pairs[y].files == 0 {
                    if (y == x) {
                        break;
                    }
                    y -= 1;
                }
                if y <= x { break; }
                while self.pairs[y].files != 0 {
                    count += i * y as i64;
                    //print!("{} ", y as i64);
                    i += 1;
                    self.pairs[x].freeSpace -= 1;
                    self.pairs[y].files -= 1;
                    self.pairs[y].freeFiles += 1;
                }
            }
            i += self.pairs[x].freeSpace as i64;
        }
        count
    }
}

fn load_day9() -> String {
    let mut file = File::open("../inputs/day9_1.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day9_1() {
    let cnt = load_day9();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day9_2() {
    let cnt = load_day9();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}