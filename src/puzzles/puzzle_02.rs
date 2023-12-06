use std::io::{BufRead, BufReader};

use super::puzzle::Puzzle;

#[derive(Clone, Debug)]
pub struct Reveal {
    pub red: Option<u32>,
    pub green: Option<u32>,
    pub blue: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub id: usize,
    pub reveals: Vec<Reveal>,
}

impl Game {
    pub fn parse_from_file(path: &str) -> Vec<Self> {
        let file = std::fs::File::open(path).unwrap();
        let rdr = BufReader::new(file);

        let mut games = vec![];

        for ln in rdr.lines().map_while(Result::ok) {
            let mut gm = Self {
                id: ln
                    .strip_prefix("Game ")
                    .unwrap()
                    .split(':')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
                reveals: vec![],
            };

            for gi in ln.split(':').nth(1).unwrap().split(';') {
                let mut rev = Reveal {
                    blue: None,
                    red: None,
                    green: None,
                };
                for i in gi.split(',') {
                    if i.ends_with("blue") {
                        rev.blue = i.trim().split(' ').next().unwrap().parse::<u32>().ok();
                    } else if i.ends_with("red") {
                        rev.red = i.trim().split(' ').next().unwrap().parse::<u32>().ok();
                    } else if i.ends_with("green") {
                        rev.green = i.trim().split(' ').next().unwrap().parse::<u32>().ok();
                    }
                }
                gm.reveals.push(rev);
            }

            games.push(gm);
        }

        games
    }
}

#[derive(Clone, Copy)]
pub struct Puzzle021;

impl Puzzle for Puzzle021 {
    fn solve(&self, input_path: &str) -> String {
        let max_blue = 14;
        let max_green = 13;
        let max_red = 12;

        let sum = Game::parse_from_file(input_path)
            .into_iter()
            .filter(|g| {
                !g.reveals.iter().any(|r| {
                    r.red.unwrap_or(0) > max_red
                        || r.green.unwrap_or(0) > max_green
                        || r.blue.unwrap_or(0) > max_blue
                })
            })
            .map(|g| g.id)
            .sum::<usize>();

        format!("{sum}")
    }
}

#[derive(Clone, Copy)]
pub struct Puzzle022;

impl Puzzle for Puzzle022 {
    fn solve(&self, input_path: &str) -> String {
        let sum = Game::parse_from_file(input_path)
            .into_iter()
            .map(|g| {
                (
                    g.reveals.iter().filter_map(|r| r.red).max().unwrap_or(0),
                    g.reveals.iter().filter_map(|r| r.green).max().unwrap_or(0),
                    g.reveals.iter().filter_map(|r| r.blue).max().unwrap_or(0),
                )
            })
            .map(|(r, g, b)| r as u64 * g as u64 * b as u64)
            .sum::<u64>();

        format!("{sum}")
    }
}
