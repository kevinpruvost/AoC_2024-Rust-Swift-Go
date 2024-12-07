use core::num;
use std::clone;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::Map;
use regex::Regex;

struct PuzzleRow {
    result: i64,
    numbers: Vec<i64>
}

struct PuzzleInput {
    rows: Vec<PuzzleRow>
}

impl PuzzleInput {
    pub fn new(contents: &str) -> Self {
        let mut obj = PuzzleInput{rows: Vec::new()};

        for line in contents.lines() {
            obj.rows.push(PuzzleRow{result: 0, numbers: Vec::new()});
            let mut lastRow = obj.rows.last_mut().unwrap();
            let splt: Vec<&str> = line.split(':').collect();
            if splt.len() == 2 {
                // Assign first split to result
                lastRow.result = splt[0].parse::<i64>().unwrap();
                // Assign second split to numbers
                let second_splt = splt[1].trim_start().split(' ').collect::<Vec<&str>>();
                for num in second_splt {
                    lastRow.numbers.push(num.parse::<i64>().unwrap());
                }
            }
        }

        obj
    }

    pub fn solve1(&self) -> i64 {
        let mut cnt = 0;

        for row in &self.rows {
            let mut sum = row.numbers[0];
            fn checkSum(row: &PuzzleRow, sum: i64, i: i64) -> bool {
                if i >= row.numbers.len() as i64 {
                    return sum == row.result;
                }
                // Addition
                if checkSum(row, sum + row.numbers[i as usize], i + 1) {
                    return true;
                }
                // Multiplication
                if checkSum(row, sum * row.numbers[i as usize], i + 1) {
                    return true;
                }
                return false;
            }
            if checkSum(&row, sum, 1) {
                cnt += row.result;
            }
        }

        cnt
    }


    pub fn solve(&self) -> i64 {
        let mut cnt = 0;

        for row in &self.rows {
            let mut sum = row.numbers[0];
            fn checkSum(row: &PuzzleRow, sumInput: i64, i: i64) -> bool {
                if i >= row.numbers.len() as i64 {
                    return sumInput == row.result;
                }
                let mut sum_input_str = sumInput.to_string();
                sum_input_str.push_str(&row.numbers[i as usize].to_string());

                // Addition
                if checkSum(row, sumInput + row.numbers[i as usize], i + 1) {
                    return true;
                }
                // Multiplication
                if checkSum(row, sumInput * row.numbers[i as usize], i + 1) {
                    return true;
                }
                // Concatenation
                if checkSum(row, sum_input_str.parse::<i64>().unwrap(), i + 1) {
                    return true;
                }
                return false;
            }
            if checkSum(&row, sum, 1) {
                cnt += row.result;
            }
        }

        cnt
    }
}

fn load_day7() -> String {
    let mut file = File::open("../inputs/day7_1.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day7_1() {
    let cnt = load_day7();
    let puzzleMap: PuzzleInput = PuzzleInput::new(&cnt);

    println!("{}", puzzleMap.solve())
}

pub fn day7_2() {
    let cnt = load_day7();
    let puzzleMap: PuzzleInput = PuzzleInput::new(&cnt);

    println!("{}", puzzleMap.solve())
}