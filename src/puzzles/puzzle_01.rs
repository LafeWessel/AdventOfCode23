use super::puzzle::Puzzle;

#[derive(Clone, Copy)]
pub struct Puzzle01;

impl Puzzle for Puzzle01 {
    fn solve(&self, input_file: &str) -> String {
        format!("{}", 0)
    }
}
