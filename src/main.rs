use puzzles::*;
use solver::SolverSentinel;
use std::sync::LazyLock;

macro_rules! lazy_file_include {
    ($name:ident, $path:expr) => {
        static $name: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string($path).expect(format!("Failed to read file {}", $path).as_str()));
    };
}

lazy_file_include!(INPUT1, "input1.txt");
lazy_file_include!(INPUT2, "input2.txt");
lazy_file_include!(INPUT3, "input3.txt");
lazy_file_include!(INPUT4, "input4.txt");
lazy_file_include!(INPUT5, "input5.txt");
lazy_file_include!(INPUT6, "input6.txt");
lazy_file_include!(INPUT7, "input7.txt");

mod puzzles;
mod solver;

fn main() {
    let mut sentinel = SolverSentinel::new();

    // day 1
    sentinel.solve(1, 1, &INPUT1, puzzle1::solve_part_1);
    sentinel.solve(1, 2, &INPUT1, puzzle1::solve_part_2);

    // day 2
    sentinel.solve(2, 1, &INPUT2, puzzle2::solve_part_1);
    sentinel.solve(2, 2, &INPUT2, puzzle2::solve_part_2);

    // day 3
    sentinel.solve(3, 1, &INPUT3, puzzle3::solve_part_1);
    sentinel.solve(3, 2, &INPUT3, puzzle3::solve_part_2);

    // day 4
    sentinel.solve(4, 1, &INPUT4, puzzle4::solve_part_1);
    sentinel.solve(4, 2, &INPUT4, puzzle4::solve_part_2);

    // day 5
    sentinel.solve(5, 1, &INPUT5, puzzle5::solve_part_1);
    // unsolved
    // sentinel.solve(5, 2, &INPUT5, puzzle5::solve_part_2);

    // day 6
    sentinel.solve(6, 1, &INPUT6, puzzle6::solve_part_1);
    sentinel.solve(6, 2, &INPUT6, puzzle6::solve_part_2);

    // day 7
    sentinel.solve(7, 1, &INPUT7, puzzle7::solve_part_1);
    // sentinel.solve(7, 2, &INPUT7, puzzle7::solve_part_2);

    sentinel.finalize();
}
