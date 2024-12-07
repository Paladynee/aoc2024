use crate::solver::SolverSentinel;
use itertools::Itertools;

#[inline]
pub fn solve_part_1(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let (page_orderings, updates) = input.split("\n\n").collect_tuple::<(&str, &str)>().unwrap();

    let orderings = page_orderings
        .lines()
        .map(|line| {
            let (left, right) = line.split('|').collect_tuple::<(&str, &str)>().unwrap();
            let (left, right) = (left.parse::<u8>().unwrap(), right.parse::<u8>().unwrap());
            (left, right)
        })
        .collect::<Vec<_>>();

    updates
        .lines()
        .map(|update| {
            let update_nums: Vec<_> = update.split(',').map(|a| a.parse::<u8>().unwrap()).collect();
            update_nums
        })
        .filter(|update_nums| {
            let valid_orderings = orderings
                .iter()
                .filter(|&&(l, r)| update_nums.contains(&l) && update_nums.contains(&r))
                .collect::<Vec<_>>();

            for &(l, r) in valid_orderings {
                if update_nums.iter().position(|&x| x == l).unwrap() > update_nums.iter().position(|&x| x == r).unwrap() {
                    return false;
                }
            }

            true
        })
        .map(|upd| upd[upd.len() / 2] as i32)
        .sum::<i32>()
}

#[inline]
pub fn solve_part_2(_input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    todo!("i give up")
}
