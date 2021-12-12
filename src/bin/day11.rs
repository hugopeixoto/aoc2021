#![feature(test)]
extern crate test;

pub fn day11(input: String) -> (usize, usize) {
    let state = input.lines().map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut p1state = state.clone();
    let mut flashes = 0;
    let mut first_to_100 = None;
    for i in 0.. {
        let mut queue = std::collections::VecDeque::new();
        for x in 0..10 {
            for y in 0..10 {
                p1state[x][y] += 1;
                if p1state[x][y] == 10 {
                    queue.push_back((x, y));
                }
            }
        }

        while !queue.is_empty() {
            let (px, py) = queue.pop_front().unwrap();

            if px > 0 && py < 9 { p1state[px-1][py+1] += 1; if p1state[px-1][py+1] == 10 { queue.push_back((px-1, py+1)); } }
            if px > 0           { p1state[px-1][py-0] += 1; if p1state[px-1][py-0] == 10 { queue.push_back((px-1, py-0)); } }
            if px > 0 && py > 0 { p1state[px-1][py-1] += 1; if p1state[px-1][py-1] == 10 { queue.push_back((px-1, py-1)); } }
            if           py > 0 { p1state[px-0][py-1] += 1; if p1state[px-0][py-1] == 10 { queue.push_back((px-0, py-1)); } }
            if           py < 9 { p1state[px-0][py+1] += 1; if p1state[px-0][py+1] == 10 { queue.push_back((px-0, py+1)); } }
            if px < 9 && py < 9 { p1state[px+1][py+1] += 1; if p1state[px+1][py+1] == 10 { queue.push_back((px+1, py+1)); } }
            if px < 9           { p1state[px+1][py-0] += 1; if p1state[px+1][py-0] == 10 { queue.push_back((px+1, py-0)); } }
            if px < 9 && py > 0 { p1state[px+1][py-1] += 1; if p1state[px+1][py-1] == 10 { queue.push_back((px+1, py-1)); } }
        }

        let mut step_flashes = 0;
        for x in 0..10 {
            for y in 0..10 {
                if p1state[x][y] >= 10 {
                    p1state[x][y] = 0;
                    step_flashes += 1;
                }
            }
        }

        if step_flashes == 100 && first_to_100.is_none() { first_to_100 = Some(i + 1); }
        if i < 100 { flashes += step_flashes; }

        if i >= 100 && first_to_100.is_some() { break; }
    }

    (flashes, first_to_100.unwrap())
}

aoc2021::day!(day11, "day11.in", bench_day11);
