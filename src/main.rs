use std::sync::LazyLock;

macro_rules! lazy_file_include {
    ($name:ident, $path:expr) => {
        static $name: LazyLock<String> = LazyLock::new(|| std::fs::read_to_string($path).expect(format!("Failed to read file {}", $path).as_str()));
    };
}

lazy_file_include!(INPUT1, "input1.txt");

mod puzzle1;
mod solver;

fn main() {
    let mut sentinel = solver::SolverSentinel::new();

    sentinel.solve(1, 1, &INPUT1, puzzle1::solve_part_1);
    sentinel.solve(1, 2, &INPUT1, puzzle1::solve_part_2);
}
