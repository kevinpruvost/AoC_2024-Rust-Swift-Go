use core::num;
use std::clone;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, Lines};
use std::iter::Map;
use regex::Regex;

struct Puzzle {
    antennas: HashMap<char, Vec<(i32, i32)>>,
    map: Vec<Vec<char>>,
    size_x: i32,
    size_y: i32
}

impl Puzzle {
    pub fn new(contents: &str) -> Self {
        let mut obj = Puzzle{antennas: HashMap::new(), map: Vec::new(), size_x: 0, size_y: 0};

        obj.size_x = contents.lines().count() as i32;
        obj.size_y = contents.lines().next().unwrap().len() as i32;
        for (i, line) in contents.lines().enumerate() {
            obj.map.push(Vec::new());
            for (j, c) in line.chars().enumerate() {
                obj.map[i].push(c);
                if c != '.' {
                    if !obj.antennas.contains_key(&c) {
                        obj.antennas.insert(c, Vec::new());
                    }
                    obj.antennas.get_mut(&c).unwrap().push((i as i32, j as i32));
                }
            }
        }

        obj
    }

    pub fn solve1(&self) -> i64 {
        let mut antiNodesSet: HashSet<(i32, i32)> = HashSet::new();

        let mut map = self.map.clone();
        for (antennaChar, coordsList) in &self.antennas {
            for (i, coords1) in coordsList.iter().enumerate() {
                for j in i+1..coordsList.len() {
                    let coords2 = &coordsList[j];
                    // Goes through every pair of antennas to check antinodes
                    let dist = (coords2.0 - coords1.0, coords2.1 - coords1.1);
                    let antinode1 = (coords1.0 - dist.0, coords1.1 - dist.1);
                    let antinode2 = (coords2.0 + dist.0, coords2.1 + dist.1);

                    // Check if antennas are here
                    for antinode in [antinode1, antinode2].iter() {
                        if (0..self.size_x).contains(&antinode.0) && (0..self.size_y).contains(&antinode.1) {
                            antiNodesSet.insert(*antinode);
                        }
                    }
                }
            }
        }

        antiNodesSet.len() as i64
    }

    pub fn solve(&self) -> i64 {
        let mut antiNodesSet: HashSet<(i32, i32)> = HashSet::new();

        let mut map = self.map.clone();
        for (antennaChar, coordsList) in &self.antennas {
            for (i, coords1) in coordsList.iter().enumerate() {
                for j in i+1..coordsList.len() {
                    let coords2 = &coordsList[j];
                    // Goes through every pair of antennas to check antinodes
                    let dist = (coords2.0 - coords1.0, coords2.1 - coords1.1);
                    // Check for direction behind coords1 and then coords2
                    for (coords, diff) in [(coords1, (-dist.0, -dist.1)), (coords1, dist)].iter() {
                        let mut x = 0;
                        loop {
                            let antinode = (coords.0 + x * diff.0, coords.1 + x * diff.1);
                            if !(0..self.size_x).contains(&antinode.0) || !(0..self.size_y).contains(&antinode.1) {
                                break;
                            }
                            antiNodesSet.insert(antinode);
                            x += 1;
                        }
                    }
                }
            }
        }

        antiNodesSet.len() as i64
    }
}

fn load_day8() -> String {
    let mut file = File::open("../inputs/day8_1.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day8_1() {
    let cnt = load_day8();
    let puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day8_2() {
    let cnt = load_day8();
    let puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}