use core::num;
use std::clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn XmasSolver1(contents: String) -> i32 {
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    
    fn CheckForPosition(lines: &Vec<String>, x: usize, y: usize, horizontal: i32, vertical: i32) -> bool {
        const XMAS_STRING: &str = "XMAS";
        for i in 0i32..XMAS_STRING.len() as i32 {
            let check_x = x as i32 + horizontal * i;
            let check_y = y as i32 + vertical * i;
            if check_x < 0 || check_y < 0 || check_x >= lines.len() as i32 || check_y >= lines[0].len() as i32 { return false; }
            if lines[check_x as usize].chars().nth(check_y as usize) != XMAS_STRING.chars().nth(i as usize) { return false; }
        }
        return true;
    }

    let mut total: i32 = 0;
    for x in 0..lines.len() {
        for y in 0..lines[x].len() {
            for h in -1..=1 {
                for v in -1..=1 {
                    if h == 0 && v == 0 { continue; }
                    total += CheckForPosition(&lines, x, y, h, v) as i32;
                }
            }
        }
    }
    total
}

fn XmasSolver(contents: String) -> i32 {
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    
    fn CheckForPosition(lines: &Vec<String>, x: usize, y: usize) -> bool {
        const XMAS_STRING: &str = "MAS";
        if lines[x].chars().nth(y).unwrap() != 'A' { return false; }
        let mut diagonalsFound = 0;
        for i in -1..=1 as i32 {
            if i == 0 {continue;}
            for j in -1..=1 as i32 {
                if j == 0 { continue; }
                if lines[(x as i32 +i) as usize].chars().nth((y as i32 + j) as usize).unwrap() != 'M' { continue; }
                if lines[(x as i32 -i) as usize].chars().nth((y as i32 - j) as usize).unwrap() != 'S' { continue; }
                diagonalsFound += 1;
                if diagonalsFound >= 2 {return true;}
            }
        }
        return false;
    }

    let mut total: i32 = 0;
    for x in 1..lines.len() - 1 {
        for y in 1..lines[x].len() - 1 {
            total += CheckForPosition(&lines, x, y) as i32;
        }
    }
    total
}


fn load_day4() -> String {
    let mut file = File::open("../inputs/day4_1.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day4_1() {
    let cnt = load_day4();

    println!("{}", XmasSolver(cnt))
}

pub fn day4_2() {
    let cnt = load_day4();

    println!("{}", XmasSolver(cnt))
}