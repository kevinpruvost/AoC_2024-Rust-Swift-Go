use std::{array, clone, iter};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, Lines};
use std::iter::Map;
use regex::Regex;
use std::num;

struct Robot
{
    pos: (i64, i64),
    velocity: (i64, i64)
}

struct Puzzle {
    robots: Vec<Robot>,
    mapSize: (i64, i64)
}

impl Puzzle {

    pub fn new(contents: &str) -> Self {
        let mut obj = Puzzle {
            robots: Vec::new(),
            mapSize: (101, 103)
        };

        for line in contents.lines() {
            let mut robot = Robot {
                pos: (0, 0),
                velocity: (0, 0)
            };
            let re = Regex::new(r"p=([+-]?\d+),([+-]?\d+) v=([+-]?\d+),([+-]?\d+)").unwrap();
            if let Some(caps) = re.captures(line) {
                robot.pos = (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap());
                robot.velocity = (caps.get(3).unwrap().as_str().parse().unwrap(), caps.get(4).unwrap().as_str().parse().unwrap());


            }
            obj.robots.push(robot);
        }

        obj
    }

    fn solve_robot(&self, robot: &Robot) -> (i64, i64) {
        let mut cnt = 1;
        let mut quad_pos: i64 = 0;
        let mut pos = robot.pos.clone();
        let velocity = robot.velocity;
        const step: i64 = 100;

        let test = 203 % 7;
        pos.0 = (pos.0 + velocity.0 * step) % self.mapSize.0;
        pos.1 = (pos.1 + velocity.1 * step) % self.mapSize.1;
        if pos.0 < 0 {
            pos.0 = self.mapSize.0 - pos.0.abs();
        }
        if pos.1 < 0 {
            pos.1 = self.mapSize.1 - pos.1.abs();
        }
        let middleX = self.mapSize.0 / 2;
        let middleY = self.mapSize.1 / 2;
        match pos {
            (x, y) if x < middleX && y < middleY => quad_pos = 0,
            (x, y) if x < middleX && y > middleY => quad_pos = 2,
            (x, y) if x > middleX && y < middleY => quad_pos = 1,
            (x, y) if x > middleX && y > middleY => quad_pos = 3,
            _ => {cnt = 0}
        }
        return (quad_pos, cnt);
    }

    pub fn solve(&self) -> i64 {
        let mut total: [i64; 4] = [0, 0, 0, 0];

        for robot in self.robots.iter() {
            let (i, res) = self.solve_robot(robot);
            total[i as usize] += res;
        }

        // return product of all quadrants
        total.iter().product()
    }
}

fn load_day14() -> String {
    let mut file = File::open("../inputs/day14.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day14_1() {
    let cnt = load_day14();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day14_2() {
    let cnt = load_day14();
    let mut puzzleMap: Puzzle = Puzzle::new(&cnt);

    println!("{}", puzzleMap.solve())
}