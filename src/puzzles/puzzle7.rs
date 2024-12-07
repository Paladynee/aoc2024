use crate::solver::SolverSentinel;

struct GoalKeeper {
    goal: usize,
    keepers: Vec<u16>,
}

enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

impl GoalKeeper {
    #[inline]
    pub fn find_solution(&self) -> Vec<Operator> {
        let mut solution = Vec::new();

        solution
    }
}

#[inline]
pub fn solve_part_1(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let keepers = input
        .lines()
        .map(|l| {
            let mut iter = l.split(':');
            let goal = iter.next().unwrap().parse::<usize>().unwrap();
            let keepers = iter
                .next()
                .unwrap()
                .split(',')
                .map(|k| k.trim().parse::<u16>().unwrap())
                .collect::<Vec<_>>();
            GoalKeeper { goal, keepers }
        })
        .collect::<Vec<_>>();
    0
}

#[inline]
pub fn solve_part_2(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    todo!();
}
