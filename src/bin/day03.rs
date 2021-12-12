#![feature(test)]
extern crate test;

pub fn find_by_bias(entries: &Vec<usize>, length: usize, bias: usize) -> usize {
    let mut prefix = 0;
    let mut candidates = entries.len();

    let mut idx = length;
    while candidates > 1 {
        let ones = entries.iter().filter(|&x| x >> idx == prefix).map(|x| (x >> (idx - 1)) & 1).sum::<usize>();
        prefix <<= 1;

        if 2 * ones / candidates == bias {
            prefix |= 1;
            candidates = ones;
        } else {
            candidates -= ones;
        }

        idx -= 1;
    }

    *entries.iter().find(|&x| (x >> idx) == prefix).unwrap()
}

pub fn day03(input: String) -> (usize, usize) {
    let lines = input.lines().map(|n| usize::from_str_radix(n, 2).unwrap()).collect::<Vec<_>>();

    let bits = (0usize.leading_zeros() - lines.iter().map(|x| x.leading_zeros()).min().unwrap()) as usize;

    let mut a = 0;
    let mut b = 0;
    for bit in 0..bits {
        if lines.iter().map(|x| (x >> bit) & 1).sum::<usize>()*2 > lines.len() {
            a |= 1 << bit;
        } else {
            b |= 1 << bit;
        }
    }

    let p1 = a * b;
    let p2 = find_by_bias(&lines, bits, 0) * find_by_bias(&lines, bits, 1);

    (p1, p2)
}

aoc2021::day!(day03, "day03.in", bench_day03);
