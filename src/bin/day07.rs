#![feature(int_abs_diff)]
#![feature(test)]
extern crate test;

pub fn day07(input: String) -> (usize, usize) {
    let mut state = input.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    state.sort();

    let median = state[state.len()/2];
    let average = state.iter().sum::<usize>() as f64 / state.len() as f64;

    let p1 = state.iter().map(|x| x.abs_diff(median)).sum();

    let p2a: usize = state.iter().map(|&x| x.abs_diff(average.floor() as usize)).map(|x| x*(x+1)/2).sum();
    let p2b: usize = state.iter().map(|&x| x.abs_diff(average.ceil() as usize)).map(|x| x*(x+1)/2).sum();

    let p2 = p2a.min(p2b);

    (p1, p2)
}

aoc2021::day!(day07, "day07.in", bench_day07);
