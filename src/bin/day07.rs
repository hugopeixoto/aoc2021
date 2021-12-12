#![feature(test)]
extern crate test;

pub fn day07(input: String) -> (usize, usize) {
    let state = input.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let range = 0 ..= state[state.len() - 1];

    let p1 = range.clone().map(|breakpoint| state.iter().map(|v| (v - breakpoint).abs()).sum::<i32>() as usize).min().unwrap();

    let p2 = range.clone().map(|breakpoint| state.iter().map(|v| (v - breakpoint).abs()).map(|v| v*(v+1)/2).sum::<i32>() as usize).min().unwrap();

    (p1, p2)
}

aoc2021::day!(day07, "day07.in", bench_day07);
