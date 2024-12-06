use core::num;
use std::clone;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::Map;
use regex::Regex;

struct PageRule {
    l: i32,
    r: i32
}

struct PageUpdate {
    pagesMap: HashMap<i32, i32>,
    midValue: i32,
    pages: Vec<i32>
}

struct PageUpdatesChecker {
    pageUpdates: Vec<PageUpdate>,
    pageRules: Vec<PageRule>,
    pageRulesMap: HashMap<i32, HashSet<i32>>,
    pageRulesSimplified: Vec<i32>
}

impl PageUpdatesChecker {
    pub fn new(contents: &String) -> Self {
        let mut ret = Self { pageUpdates: Vec::new(), pageRules: Vec::new(), pageRulesMap: HashMap::new(), pageRulesSimplified: Vec::new() };

        let mut parts = contents.splitn(2, "\n\n");
        let firstPart = parts.next().unwrap_or("");
        let secondPart = parts.next().unwrap_or("");

        for line in firstPart.lines() {
            if let left_and_right = line.split('|').collect::<Vec<&str>>() {
                if let Ok(left_int) = left_and_right[0].trim().parse::<i32>() {
                    if let Ok(right_int) = left_and_right[1].trim().parse::<i32>() {
                        ret.pageRules.push(PageRule { l: left_int, r: right_int });
                        if (!ret.pageRulesMap.contains_key(&left_int)) {
                            ret.pageRulesMap.insert(left_int, HashSet::new());
                        }
                        ret.pageRulesMap.get_mut(&left_int).unwrap().insert(right_int);
                    }
                }
            }
        }

        for line in secondPart.lines() {
            if let pages = line.split(',').collect::<Vec<&str>>() {
                ret.pageUpdates.push(PageUpdate { pagesMap: HashMap::new(), midValue: 0, pages: Vec::new() });
                let mut i = 0;
                for page in pages.iter() {
                    if let Ok(page_int) = page.trim().parse::<i32>() {
                        if let Some(last) = ret.pageUpdates.last_mut() {
                            last.pagesMap.insert(page_int, i);
                            last.pages.push(page_int);
                            if i as usize == pages.len() / 2 {
                                last.midValue = page_int;
                            }
                            i += 1;
                        }
                    }
                }
            }
        }

        ret
    }

    pub fn check_updates1(&self) -> i32 {
        let mut total = 0;

        let mut invalidUpdates: Vec<&PageUpdate> = Vec::new();

        for pageUpdate in self.pageUpdates.iter() {
            let mut valid = true;
            for pageRule in self.pageRules.iter() {
                if let Some((keyLeft, indexLeft)) = pageUpdate.pagesMap.get_key_value(&pageRule.l) {
                    if let Some((keyRight, indexRight)) = pageUpdate.pagesMap.get_key_value(&pageRule.r) {
                        if indexLeft > indexRight {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            if valid {
                total += pageUpdate.midValue;
            }
        }

        total
    }

    pub fn check_updates(&self) -> i32 {
        let mut total = 0;

        let mut invalidUpdates: Vec<&PageUpdate> = Vec::new();

        for pageUpdate in self.pageUpdates.iter() {
            let mut valid = true;
            for pageRule in self.pageRules.iter() {
                if let Some((keyLeft, indexLeft)) = pageUpdate.pagesMap.get_key_value(&pageRule.l) {
                    if let Some((keyRight, indexRight)) = pageUpdate.pagesMap.get_key_value(&pageRule.r) {
                        if indexLeft > indexRight {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            if !valid {
                invalidUpdates.push(&pageUpdate);
            }
        }

        // Brute Force
        // Checking every rule but from the end of the page update
        // to avoid a majority of the loops: runtime of 1.5ms
        for pageUpdate in invalidUpdates.iter() {
            let mut correctedPageUpdate = pageUpdate.pages.clone();
            for i in 0..correctedPageUpdate.len() {
                for mut j in (0..correctedPageUpdate.len()).rev() {
                    if j <= i {break;}
                    if let Some(ruleMap1) = self.pageRulesMap.get(&correctedPageUpdate[j]) {
                        if let Some(ruleMap2) = ruleMap1.get(&correctedPageUpdate[i]) {
                            correctedPageUpdate.swap(i, j);
                            j = correctedPageUpdate.len() - 1;
                        }
                    }
                }
            }
            total += correctedPageUpdate[correctedPageUpdate.len() / 2];
        }

        total
    }

}

fn load_day5() -> String {
    let mut file = File::open("../inputs/day5_1.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn day5_1() {
    let cnt = load_day5();
    let pageUpdateChecker: PageUpdatesChecker = PageUpdatesChecker::new(&cnt);

    println!("{}", pageUpdateChecker.check_updates())
}

pub fn day5_2() {
    let cnt = load_day5();
    let pageUpdateChecker: PageUpdatesChecker = PageUpdatesChecker::new(&cnt);

    println!("{}", pageUpdateChecker.check_updates())
}