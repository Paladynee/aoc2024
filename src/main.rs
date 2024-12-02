use std::sync::LazyLock;

macro_rules! lazy_file_include {
    ($name:ident, $path:expr) => {
        static $name: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string($path).expect(format!("Failed to read file {}", $path).as_str()));
    };
}

lazy_file_include!(INPUT1, "input1.txt");
lazy_file_include!(INPUT2, "input2.txt");

mod puzzles;
mod solver;

use puzzles::*;

fn main() {
    let mut sentinel = solver::SolverSentinel::new();

    // day 1
    sentinel.solve(1, 1, &INPUT1, puzzle1::solve_part_1);
    sentinel.solve(1, 2, &INPUT1, puzzle1::solve_part_2);

    // day 2
    sentinel.solve(2, 1, &INPUT2, puzzle2::solve_part_1);
    sentinel.solve(2, 2, &INPUT2, puzzle2::solve_part_2);
}
