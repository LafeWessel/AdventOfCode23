pub trait Puzzle {
    fn solve(&self, input_path: &str) -> String;
}
