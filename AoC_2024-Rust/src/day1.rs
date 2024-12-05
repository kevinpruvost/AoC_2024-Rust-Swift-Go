use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn day1_1() {
    // Read the entire file into a string
    let contents = {
       let mut file = File::open("../inputs/day1_1.txt").unwrap();
       let mut buffer = String::new();
       file.read_to_string(&mut buffer).unwrap();
       buffer
   };
   // Left array and Right array
   let mut leftArray: Vec<i32> = Vec::new();
   let mut rightArray: Vec<i32> = Vec::new();
   for line in contents.lines() {
       let parts: Vec<&str> = line
           .trim()
           .split(" ")
           .filter(|s| !s.is_empty())
           .collect();
       let left: i32 = parts[0].parse().unwrap();
       let right: i32 = parts[1].parse().unwrap();
       leftArray.push(left);
       rightArray.push(right);
   }
   // Sort arrays
   leftArray.sort();
   rightArray.sort();
   let mut sumDistance: i32 = 0;
   for (left_iter, right_iter) in leftArray.iter().zip(rightArray.iter()) {
       sumDistance += (right_iter - left_iter).abs();
   }
   println!("Sum of distances: {}", sumDistance);

}

pub fn day1_2() {
   let contents = {
       let mut file = File::open("../inputs/day1_1.txt").unwrap();
       let mut buffer = String::new();
       file.read_to_string(&mut buffer).unwrap();
       buffer
   };

   let mut leftMap: HashMap<i32, i32> = HashMap::new();
   let mut rightMap: HashMap<i32, i32> = HashMap::new();

   for line in contents.lines() {
       let parts: Vec<&str> = line
           .trim()
           .split(" ")
           .filter(|s| !s.is_empty())
           .collect();
       let left: i32 = parts[0].parse().unwrap();
       let right: i32 = parts[1].parse().unwrap();
       let leftCount = leftMap.entry(left).or_insert(0);
       *leftCount += 1;
       let rightCount = rightMap.entry(right).or_insert(0);
       *rightCount += 1;
   }

   let mut similarityIndex: i32 = 0;
   for ((keyLeft, occurenceLeft)) in leftMap.iter() {
       if rightMap.contains_key(keyLeft) {
           let occurenceRight = rightMap.get_key_value(keyLeft).unwrap().1;
           similarityIndex += occurenceLeft * occurenceRight * keyLeft;
       }
   }
   println!("Similarity: {}", similarityIndex);
}