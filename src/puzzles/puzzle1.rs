use rayon::prelude::*;
use std::io::Read;

use voxell_timer::time;

use crate::solver::SolverSentinel;

pub struct Lists {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Lists {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            left: Vec::with_capacity(capacity),
            right: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn into_inner(self) -> (Vec<i32>, Vec<i32>) {
        (self.left, self.right)
    }
}

#[inline]
pub fn solve_part_1(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let (aux, _loop_time) = time!(parse(input));
    let (mut left, mut right) = aux.into_inner();

    let ((), _sort_time) = time!({
        left.par_sort_unstable();
        right.par_sort_unstable();
    });

    let (ret, _sum_time) = time!(left.into_iter().zip(right).map(|(a, b)| (a - b).abs()).sum::<i32>());

    // // warning! this will take a lot of time
    // threaded_println!("Loop time: {:?}", loop_time);
    // threaded_println!("Sort time: {:?}", sort_time);
    // threaded_println!("Sum time: {:?}", sum_time);

    ret
}

#[inline]
pub fn solve_part_2(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let aux = parse(input);
    // discriminants 0..=9999 cannot be constructed, so we can reduce the allocation by 10% (99999 -> 90000)
    let mut occurence_buckets: Vec<i32> = vec![0; 90000];
    let (left, right) = aux.into_inner();

    for elem in right {
        occurence_buckets[elem as usize - 10000] += 1;
    }

    left.clone().into_iter();

    left.into_iter()
        .map(|elem| {
            let occurences = occurence_buckets[elem as usize - 10000];
            occurences * elem
        })
        .sum::<i32>()

    // let mut occurence_buckets: AHashMap<i32, usize> = AHashMap::new();

    // let (left, right) = aux.into_inner();

    // // we only care about the right list
    // for elem in right {
    //     let counter = occurence_buckets.entry(elem).or_insert(0);
    //     *counter += 1;
    // }

    // left.iter()
    //     .map(|elem| {
    //         let occurences = occurence_buckets.get(elem).unwrap_or(&0);
    //         *occurences as i32 * elem
    //     })
    //     .sum::<i32>()
}

#[inline]
pub fn parse(input: &str) -> Lists {
    let elems = input.len() / 6;
    let mut aux: Lists = Lists::with_capacity(elems);

    // // 85~ ms total execution for part1
    // for line in input.lines() {
    //     let mut iter = line.split_ascii_whitespace();

    //     let first_num = iter.next().unwrap().parse::<i32>().unwrap();
    //     let second_num = iter.next().unwrap().parse::<i32>().unwrap();

    //     aux.left.push(first_num);
    //     aux.right.push(second_num);
    // }

    // 60~ ms total execution for part1
    let reader = &mut input.as_bytes();

    loop {
        let mut tmp: [u8; 5] = [0; 5];
        match reader.read(&mut tmp) {
            Err(_) | Ok(0) => break,
            Ok(_) => {}
        }

        let first_num: i32 = tmp
            .iter()
            .enumerate()
            .map(|(exp, ascii)| {
                // ascii is ascii digit, so we have to do u8 ascii to u8 decimal convertion
                let decimal = ascii - b'0';
                decimal as i32 * 10_i32.pow(4 - exp as u32)
            })
            .sum();

        // advance by 3 bytes
        *reader = &reader[3..];

        match reader.read(&mut tmp) {
            Err(_) | Ok(0) => break,
            Ok(_) => {}
        }

        let second_num: i32 = tmp
            .iter()
            .enumerate()
            .map(|(exp, ascii)| {
                // ascii is ascii digit, so we have to do u8 ascii to u8 decimal convertion
                let decimal = ascii - b'0';
                decimal as i32 * 10_i32.pow(4 - exp as u32)
            })
            .sum();

        aux.left.push(first_num);
        aux.right.push(second_num);

        // advance the reader by 1 byte (\n)
        *reader = &reader[1..];
    }
    aux
}
