use rayon::prelude::*;

use crate::solver::SolverSentinel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gradually {
    Increasing,
    Decreasing,
    None,
}

#[inline]
pub fn solve_part_1(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    input
        .lines()
        .filter(|line| {
            let mut last_number = None;
            let mut sequence_type = Gradually::None;

            for num_lit in line.trim().split_ascii_whitespace() {
                let num_val = num_lit.parse::<u32>().unwrap();

                match last_number {
                    None => last_number = Some(num_val),
                    Some(lst_num) => {
                        let diff = lst_num.abs_diff(num_val);
                        if !(1..=3).contains(&diff) {
                            return false;
                        }
                        match sequence_type {
                            Gradually::None => {
                                sequence_type = if lst_num < num_val {
                                    Gradually::Increasing
                                } else {
                                    Gradually::Decreasing
                                };
                            }

                            Gradually::Increasing => {
                                if lst_num > num_val {
                                    return false;
                                }
                            }

                            Gradually::Decreasing => {
                                if lst_num < num_val {
                                    return false;
                                }
                            }
                        };

                        last_number = Some(num_val);
                    }
                };
            }

            true
        })
        .count() as i32
}

//  use std::hint::{assert_unchecked, unreachable_unchecked};
// // failed attempt here lol
// /// SAFETY: caller must uphold that all numbers in input are valid u32 parsable
// /// otherwise it's UB to call this function
// #[inline]
// pub unsafe fn solve_part_2(input: &str) -> i32 {
//     input
//         .lines()
//         .filter(|line| {
//             let num_vals = line
//                 .trim()
//                 .split_ascii_whitespace()
//                 .map(|lit| unsafe { lit.parse::<u32>().unwrap_unchecked() })
//                 .collect::<Vec<_>>();

//             let mut fault_tolerance = false;
//             let mut sequence = Gradually::None;

//             num_vals.windows(3).all(|wind| {
//                 unsafe {
//                     assert_unchecked(wind.len() == 3);
//                 };

//                 let [ancient, old, young] = wind else {
//                     unreachable_unchecked();
//                 };

//                 let diff = old.abs_diff(*young);

//                 if !(1..=3).contains(&diff) {
//                     if fault_tolerance {
//                         return false;
//                     }

//                     let ancient_diff = ancient.abs_diff(*young);

//                     if !(1..=3).contains(&ancient_diff) {
//                         return false;
//                     } else {
//                         fault_tolerance = true;
//                         match sequence {
//                             Gradually::None => {
//                                 sequence = if ancient < young { Gradually::Increasing } else { Gradually::Decreasing };
//                             }

//                             Gradually::Decreasing => {
//                                 if ancient < young {
//                                     return false;
//                                 }
//                             }

//                             Gradually::Increasing => {
//                                 if ancient > young {
//                                     return false;
//                                 }
//                             }
//                         }
//                     }
//                 }

//                 match sequence {
//                     Gradually::None => {
//                         sequence = if ancient < young { Gradually::Increasing } else { Gradually::Decreasing };
//                     }

//                     Gradually::Decreasing => {
//                         if ancient < young {
//                             return false;
//                         }
//                     }

//                     Gradually::Increasing => {
//                         if ancient > young {
//                             return false;
//                         }
//                     }
//                 }

//                 true
//             })
//         })
//         .count() as i32
// }

// brute force approach
#[inline]
pub fn solve_part_2(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    input
        .par_lines()
        .filter(|line| {
            let nums = line
                .trim()
                .split_ascii_whitespace()
                .map(|lit| lit.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            (0..nums.len()).any(|item| {
                let mut nums = nums.clone();
                nums.remove(item);
                check(nums.as_slice())
            })
        })
        .count() as i32
}

#[inline]
pub fn check(nums: &[u32]) -> bool {
    let mut last_number = None;
    let mut sequence_type = Gradually::None;

    for num in nums {
        match last_number {
            None => last_number = Some(num),
            Some(lst_num) => {
                let diff = lst_num.abs_diff(*num);
                if !(1..=3).contains(&diff) {
                    return false;
                }
                match sequence_type {
                    Gradually::None => {
                        sequence_type = if lst_num < num { Gradually::Increasing } else { Gradually::Decreasing };
                    }

                    Gradually::Increasing => {
                        if lst_num > num {
                            return false;
                        }
                    }

                    Gradually::Decreasing => {
                        if lst_num < num {
                            return false;
                        }
                    }
                };

                last_number = Some(num);
            }
        };
    }

    true
}
