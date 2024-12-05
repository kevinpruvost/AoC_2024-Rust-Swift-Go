use core::num;
use std::clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

// Overengineering here but wanna discover a bit
trait CalculatorOperation {
    fn calculate(&self) -> i32;
    fn IsDo(&self) -> bool;
    fn IsDont(&self) -> bool;
}

struct CalculatorMultiplication {
    pub x1: i32,
    pub x2: i32
}

struct CalculatorDo {
}

struct CalculatorDont {

}

impl CalculatorOperation for CalculatorMultiplication {
    fn calculate(&self) -> i32 {
        return self.x1 * self.x2
    }
    fn IsDo(&self) -> bool { false }
    fn IsDont(&self) -> bool { false }
}

impl CalculatorOperation for CalculatorDo {
    fn calculate(&self) -> i32 {
        return 0
    }
    fn IsDo(&self) -> bool { true }
    fn IsDont(&self) -> bool { false }
}

impl CalculatorOperation for CalculatorDont {
    fn calculate(&self) -> i32 {
        return 0
    }
    fn IsDo(&self) -> bool { false }
    fn IsDont(&self) -> bool { true }
}

fn filter1(content: String) -> i32 {
    let mut operations: Vec<Box<dyn CalculatorOperation>> = Vec::new();
    let mut total: i32 = 0;
    
    for (i, _) in content.match_indices("mul(") {
        let calc_part: &str = content.split_at(i).1;
        let parenthesis_pos_option = calc_part.find(")");
        if parenthesis_pos_option.is_some() {
            let parenthesis_pos = parenthesis_pos_option.unwrap();

            let numbers_string = &calc_part[4..parenthesis_pos];
            let numbers: Vec<&str> = numbers_string.split(",").collect();
            if numbers.len() != 2 {
                continue;
            }
            if let (Ok(x1), Ok(x2)) = (numbers[0].trim().parse::<i32>(), numbers[1].trim().parse::<i32>()) {
                operations.push(Box::new(CalculatorMultiplication { x1, x2 }));
            }
        }
    }

    let mut doCalculate: bool = true;
    for ope in operations.iter() {
        if doCalculate {
            total += ope.calculate()
        }
        if ope.IsDo() { doCalculate = true; }
        if ope.IsDont() { doCalculate = false;}
    }
    total
}

fn filter(content: String) -> i32 {
    let mut operations: Vec<Box<dyn CalculatorOperation>> = Vec::new();
    let mut total: i32 = 0;
    
    let re = Regex::new(r"(mul\(|do\(\)|don't\(\))").unwrap();

    for caps in re.captures_iter(&content) {
        if let Some(matched) = caps.get(0) {
            let i = matched.start();
            let test: &str = &content[i..];
            match matched.as_str() {
                "mul(" => {
                    let calc_part: &str = content.split_at(i).1;
                    let parenthesis_pos_option = calc_part.find(")");
                    if parenthesis_pos_option.is_some() {
                        let parenthesis_pos = parenthesis_pos_option.unwrap();

                        let numbers_string = &calc_part[4..parenthesis_pos];
                        let numbers: Vec<&str> = numbers_string.split(",").collect();
                        if numbers.len() != 2 {
                            continue;
                        }
                        if let (Ok(x1), Ok(x2)) = (numbers[0].trim().parse::<i32>(), numbers[1].trim().parse::<i32>()) {
                            operations.push(Box::new(CalculatorMultiplication { x1, x2 }));
                        }
                    }
                },
                "do()" => { 
                    operations.push(Box::new(CalculatorDo{}));
                },
                "don't()" => {
                    operations.push(Box::new(CalculatorDont{}));
                },
                _ => (),
            }
        }
    }

    let mut doCalculate: bool = true;
    for ope in operations.iter() {
        if doCalculate {
            total += ope.calculate()
        }
        if ope.IsDo() { doCalculate = true; }
        if ope.IsDont() { doCalculate = false; }
    }
    total
}


fn load_day3() -> String {
    let mut file = File::open("../inputs/day3_1.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day3_1() {
    let cnt = load_day3();

    println!("{}", filter(cnt))
}

pub fn day3_2() {
    let cnt = load_day3();

    println!("{}", filter(cnt))
}