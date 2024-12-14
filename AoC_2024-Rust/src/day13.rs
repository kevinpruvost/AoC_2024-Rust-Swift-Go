use std::{clone, iter};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, Lines};
use std::iter::Map;
use regex::Regex;
use std::num;

struct ClawMachine
{
    buttonA: (i64, i64),
    buttonB: (i64, i64),
    prize: (i64, i64)
}

struct Puzzle {
    pairs: Vec<ClawMachine>
}

fn parse_expression(haystack: &str) -> i64 {
    let re = Regex::new(r"([A-Za-z]+)=?([+-=]?\d+)").unwrap();
    if let Some(caps) = re.captures(haystack) {
        let num = caps.get(2).unwrap().as_str().parse().unwrap();
        return num;
    }
    0
}

impl Puzzle {

    pub fn new(contents: &str) -> Self {
        let mut obj = Puzzle {
            pairs: Vec::new()
        };

        let mut pair = ClawMachine {
            buttonA: (0, 0),
            buttonB: (0, 0),
            prize: (0, 0)
        };
        for (i, line) in contents.lines().enumerate() {
            if let Some(coords) = line.split(": ").collect::<Vec<&str>>().get(1) {
                if let Some(coordsXY) = coords.split(",").collect::<Vec<&str>>().get(0..2) {
                    if let Some(&x) = coordsXY.get(0) {
                        if let Some(&y) = coordsXY.get(1) {
                            let numX = parse_expression(x);
                            let numY = parse_expression(y);
                            if i % 4 == 0 {
                                pair.buttonA = (numX, numY);
                            } else if i % 4 == 1 {
                                pair.buttonB = (numX, numY);
                            } else if i % 4 == 2 {
                                // Part 2 asks for 10000000000000 in addition
                                pair.prize = (numX + 10000000000000, numY + 10000000000000);
                                obj.pairs.push(pair);
                                pair = ClawMachine {
                                    buttonA: (0, 0),
                                    buttonB: (0, 0),
                                    prize: (0, 0)
                                };
                            }
                        }
                    }
                }
            }
        }

        obj
    }

    pub fn solve_clawMachine1(&self, claw: &ClawMachine) -> i64 {
        let mut res = 0;

        let maxTimesB = (claw.prize.1 / claw.buttonB.1).min(claw.prize.0 / claw.buttonB.0);
        for i in (0..=maxTimesB).rev() {
            let remaining = (claw.prize.0 - i * claw.buttonB.0, claw.prize.1 - i * claw.buttonB.1);
            let timesA = (remaining.0 / claw.buttonA.0).min(remaining.1 / claw.buttonA.1);
            let remainingFinal = (remaining.0 - timesA * claw.buttonA.0, remaining.1 - timesA * claw.buttonA.1);
            if remainingFinal.0 == 0 && remainingFinal.1 == 0 {
                res = i * 1 + timesA * 3;
                break;
            }
        }

        res
    }

    pub fn solve1(&self) -> i64 {
        let mut total = 0;

        for claw in self.pairs.iter() {
            total += self.solve_clawMachine1(claw);
        }

        total
    }

    pub fn solve_clawMachine(&self, claw: &ClawMachine) -> i64 {
        let mut res = 0;

        let ca = (claw.prize.0 * claw.buttonB.1 - claw.prize.1 * claw.buttonB.0) / (claw.buttonA.0 * claw.buttonB.1 - claw.buttonA.1 * claw.buttonB.0);
        let cb = (claw.prize.0 - ca * claw.buttonA.0) / claw.buttonB.0;
        if ca % 1 == cb % 1 && ca % 1 == 0 {
            res = ca * 3 + cb;
        }

        //println!("{:?}", res);
        res
    }

    pub fn solve(&self) -> i64 {
        let mut total = 0;

        for claw in self.pairs.iter() {
            total += self.solve_clawMachine(claw);
        }

        total
    }
}

fn load_day13() -> String {
    let mut file = File::open("../inputs/day13.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day13_1() {
    let cnt = load_day13();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day13_2() {
    let cnt = load_day13();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}