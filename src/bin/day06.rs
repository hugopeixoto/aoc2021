#![feature(test)]
extern crate test;

pub fn day06(input: String) -> (usize, usize) {
    let mut state = std::collections::VecDeque::new();
    state.resize(9, 0);

    for fish in input.trim().split(",").map(|x| x.parse::<usize>().unwrap()) {
        state[fish] += 1;
    }

    for _ in 1..=80 {
        let ready = state.pop_front().unwrap();
        state[6] += ready;
        state.push_back(ready);
    }

    let p1 = state.iter().sum();

    for _ in 81..=256 {
        let ready = state.pop_front().unwrap();
        state[6] += ready;
        state.push_back(ready);
    }
    let p2 = state.iter().sum();

    (p1, p2)
}

aoc2021::day!(day06, "day06.in", bench_day06);
