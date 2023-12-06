use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Range, RangeInclusive},
};

use super::puzzle::Puzzle;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NON_SPECIAL: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

#[derive(Debug, Clone)]
pub struct Num {
    pub val: u32,
    pub location: Range<usize>,
    pub line: usize,
}

pub struct Parser;

impl Parser {
    pub fn parse_from_file(path: &str) -> (Vec<String>, Vec<Num>) {
        let f = File::open(path).unwrap();
        let rdr = BufReader::new(f);

        let mut nums = vec![];
        let mut lines = vec![];

        for (lc, ln) in rdr.lines().map_while(Result::ok).enumerate() {
            lines.push(ln.clone());
            let mut num = String::new();
            let mut num_start = None;

            for (i, cr) in ln.chars().enumerate() {
                if DIGITS.contains(&cr) {
                    if num_start.is_none() {
                        num_start = Some(i);
                    }
                    num.push(cr);
                } else {
                    if !num.is_empty() {
                        let val = num.parse().expect("Could not parse number");

                        nums.push(Num {
                            val,
                            line: lc,
                            location: (num_start.unwrap()..i),
                        })
                    }
                    num.clear();
                    num_start = None;
                }
            }

            if !num.is_empty() {
                nums.push(Num {
                    val: num.parse().expect("Could not parse number"),
                    location: (num_start.unwrap()..ln.len()),
                    line: lc,
                })
            }
        }

        (lines, nums)
    }
}

pub struct Puzzle031 {}

impl Puzzle for Puzzle031 {
    fn solve(&self, input_path: &str) -> String {
        let (strs, vals) = Parser::parse_from_file(input_path);

        let str_ct = strs.len();

        println!(
            "Max number (all values accepted): {}",
            vals.iter().map(|v| v.val as usize).sum::<usize>()
        );

        let result = vals
            .iter()
            .filter(|v| {
                // println!("{v:?}");

                let mut adjacents = vec![];

                // check the 8 locations around each point
                for i in v.location.clone() {
                    // above line
                    if v.line > 0 {
                        // above left
                        if i > 0 {
                            adjacents.push(strs[v.line - 1].chars().nth(i - 1).unwrap());
                        }
                        // above
                        adjacents.push(strs[v.line - 1].chars().nth(i).unwrap());
                        // above right
                        if i < strs[v.line - 1].len() - 1 {
                            adjacents.push(strs[v.line - 1].chars().nth(i + 1).unwrap());
                        }
                    }
                    // same line left
                    if i > 0 {
                        adjacents.push(strs[v.line].chars().nth(i - 1).unwrap());
                    }
                    // same line right
                    if i < strs[v.line].len() - 1 {
                        adjacents.push(strs[v.line].chars().nth(i + 1).unwrap());
                    }
                    // below line
                    if v.line < str_ct - 2 {
                        // below left
                        if i > 0 {
                            adjacents.push(strs[v.line + 1].chars().nth(i - 1).unwrap());
                        }
                        // below
                        adjacents.push(strs[v.line + 1].chars().nth(i).unwrap());
                        // below right
                        if i < strs[v.line + 1].len() - 1 {
                            adjacents.push(strs[v.line + 1].chars().nth(i + 1).unwrap());
                        }
                    }
                }
                adjacents.iter().any(|c| !NON_SPECIAL.contains(c))
            })
            .map(|v| v.val as usize)
            .sum::<usize>();

        format!("{result}")
    }
}

pub struct Puzzle032 {}

impl Puzzle for Puzzle032 {
    fn solve(&self, input_path: &str) -> String {
        let (strs, vals) = Parser::parse_from_file(input_path);

        let mut stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        let str_ct = strs.len();

        for v in vals.iter() {
            // check the 8 locations around each point
            for i in v.location.clone() {
                // above line
                if v.line > 0 {
                    // above left
                    if i > 0 && strs[v.line - 1].chars().nth(i - 1).unwrap() == '*' {
                        let e = stars.entry((v.line - 1, i - 1)).or_default();
                        e.push(v.val);
                        break;
                    }
                    // above
                    if strs[v.line - 1].chars().nth(i).unwrap() == '*' {
                        let e = stars.entry((v.line - 1, i)).or_default();
                        e.push(v.val);
                        break;
                    }
                    // above right
                    if i < strs[v.line - 1].len() - 1
                        && strs[v.line - 1].chars().nth(i + 1).unwrap() == '*'
                    {
                        let e = stars.entry((v.line - 1, i + 1)).or_default();
                        e.push(v.val);
                        break;
                    }
                }
                // same line left
                if i > 0 && strs[v.line].chars().nth(i - 1).unwrap() == '*' {
                    let e = stars.entry((v.line, i - 1)).or_default();
                    e.push(v.val);
                    break;
                }
                // same line right
                if i < strs[v.line].len() - 1 && strs[v.line].chars().nth(i + 1).unwrap() == '*' {
                    let e = stars.entry((v.line, i + 1)).or_default();
                    e.push(v.val);
                    break;
                }
                // below line
                if v.line < str_ct - 2 {
                    // below left
                    if i > 0 && strs[v.line + 1].chars().nth(i - 1).unwrap() == '*' {
                        let e = stars.entry((v.line + 1, i - 1)).or_default();
                        e.push(v.val);
                        break;
                    }
                    // below
                    if strs[v.line + 1].chars().nth(i).unwrap() == '*' {
                        let e = stars.entry((v.line + 1, i)).or_default();
                        e.push(v.val);
                        break;
                    }
                    // below right
                    if i < strs[v.line + 1].len() - 1
                        && strs[v.line + 1].chars().nth(i + 1).unwrap() == '*'
                    {
                        let e = stars.entry((v.line + 1, i + 1)).or_default();
                        e.push(v.val);
                        break;
                    }
                }
            }
        }

        for (k, v) in stars.iter() {
            println!("{:?}: {:?}", k, v);
        }

        let result = stars
            .iter()
            .filter(|f| f.1.len() == 2)
            .map(|f| f.1[0] as usize * f.1[1] as usize)
            .sum::<usize>();

        format!("{result}")
    }
}
