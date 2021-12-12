#![feature(test)]
extern crate test;

pub fn day01(input: String) -> (usize, usize) {
    let depths: Vec<_> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();

    let mut increases = 0;
    let mut previous = depths[0];

    for &depth in depths.iter() {
        if previous < depth {
            increases += 1;
        }

        previous = depth;
    }

    let mut window_increases = 0;
    let mut previous = depths[0] + depths[1] + depths[2];

    for index in 2..depths.len() {
        let depth = depths[index - 2] + depths[index - 1] + depths[index];
        if previous < depth {
            window_increases += 1;
        }

        previous = depth;
    }

    (increases, window_increases)
}

aoc2021::day!(day01, "day01.in", bench_day01);
