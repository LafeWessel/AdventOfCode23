use std::path::Path;

pub trait Puzzle {
    fn solve(&self, input_path: &str) -> String;
}
