use std::clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Report {
    values: Vec<i32>
}

impl Report {
    pub fn new(line: &str) -> Self {
        let mut newObj = Self { values: Vec::new() };

        // Parse str elements
        let parts: Vec<&str> = line
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();

        for part in parts {
            newObj.values.push(part.parse::<i32>().unwrap());
        }
        
        newObj
    }

    pub fn isSafe1(&self) -> (bool, usize) {
        let mut diff: i32 = (self.values[0] - self.values[1]).abs();
        if diff > 3 || diff == 0 || (self.values[0] < self.values[1]) != (self.values[1] < self.values[2]) {
            return (false, 0);
        }

        let direction: bool = self.values[0] < self.values[1];

        for i in 1..self.values.len() - 1 {
            diff = (self.values[i] - self.values[i+1]).abs();
            if diff > 3 || diff == 0 || (self.values[i] < self.values[i+1]) != direction {
                return (false, i);
            }
        }
        return (true, 0);
    }

    pub fn isSafe2(&self) -> bool {
        let (mut res, mut i) = self.isSafe1();
        if res {return true;}

        // Remove wrong index, test 3 potential replacements
        for x in 0..3 {
            let mut testSelf: Report = self.clone();
            testSelf.values.remove(i + x);
            (res, _) = testSelf.isSafe1();
            if res {return true;}
        }
        return false;
    }
}

pub struct Reports {
    reports: Vec<Report>
}

impl Reports {
    pub fn new(contents: String) -> Self {
        let mut newObj = Self { reports: Vec::new() };

        for line in contents.lines() {
            newObj.reports.push(Report::new(line));
        }

        newObj
    }

    pub fn checkSafety(&self) -> i32 {
        let mut total: i32 = 0;
        for report in self.reports.iter() {
            let res = report.isSafe2() as i32;
            total += res;
        }
        total
    }
}

fn load_day2() -> Reports {
    let contents = {
        let mut file = File::open("../inputs/day2_1.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    };

    Reports::new(contents)
}

pub fn day2_1() {
    let reports: Reports = load_day2();
    let total = reports.checkSafety();

    println!("{}", total)
}

pub fn day2_2() {
    let reports: Reports = load_day2();
    let total = reports.checkSafety();

    println!("{}", total)
}