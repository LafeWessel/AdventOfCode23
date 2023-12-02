use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::puzzle::Puzzle;

#[derive(Clone, Copy)]
pub struct Puzzle011;

impl Puzzle for Puzzle011 {
    fn solve(&self, input_file: &str) -> String {
        let fl = File::open(input_file).expect("Input file cannot be opened");
        let rdr = BufReader::new(fl);

        let v = rdr
            .lines()
            .filter_map(|ln| ln.ok())
            .map(|ln| {
                let vals = ln
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<_>>();
                10 * *vals.get(0).unwrap_or(&0)
                    + *vals.get(vals.len().saturating_sub(1)).unwrap_or(&0)
            })
            .sum::<u32>();
        format!("{v}")
    }
}

#[derive(Clone, Copy)]
pub struct Puzzle012;

impl Puzzle012 {
    fn find_digit(line: &str) -> Option<u32> {
        if line.contains("one") {
            return Some(1);
        }
        if line.contains("two") {
            return Some(2);
        }
        if line.contains("three") {
            return Some(3);
        }
        if line.contains("four") {
            return Some(4);
        }
        if line.contains("five") {
            return Some(5);
        }
        if line.contains("six") {
            return Some(6);
        }
        if line.contains("seven") {
            return Some(7);
        }
        if line.contains("eight") {
            return Some(8);
        }
        if line.contains("nine") {
            return Some(9);
        }
        None
    }
}

impl Puzzle for Puzzle012 {
    fn solve(&self, input_file: &str) -> String {
        let fl = File::open(input_file).expect("Input file cannot be opened");
        let rdr = BufReader::new(fl);

        let v = rdr
            .lines()
            .filter_map(|ln| ln.ok())
            .map(|ln| {
                println!("{ln}");
                let mut strn = String::new();
                let mut first = 0;
                for c in ln.chars() {
                    if let Some(v) = c.to_digit(10) {
                        first = v;
                        break;
                    } else {
                        strn.push(c);
                        if let Some(v) = Self::find_digit(&strn) {
                            first = v;
                            strn.clear();
                            break;
                        }
                    }
                }
                let mut last = 0;
                for c in ln.chars().rev() {
                    if let Some(v) = c.to_digit(10) {
                        last = v;
                        break;
                    } else {
                        strn.insert(0, c);
                        if let Some(v) = Self::find_digit(&strn) {
                            last = v;
                            strn.clear();
                            break;
                        }
                    }
                }

                let res = 10 * first + last;
                println!("res: {res}");
                res
            })
            .sum::<u32>();
        format!("{v}")
    }
}
