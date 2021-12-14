#![feature(destructuring_assignment)]
#![feature(test)]
extern crate test;

use std::collections::HashMap;

type Pairs = HashMap<(char, char), usize>;
type Counts = HashMap<char, usize>;

pub fn steps(rules: &HashMap<(char, char), char>, pairs: &Pairs, counts: &Counts) -> (Pairs, Counts) {
    let mut npairs = HashMap::new();
    let mut ncounts = counts.clone();

    for (k, v) in pairs.iter() {
        let i = rules[k];
        *npairs.entry((k.0, i)).or_insert(0) += v;
        *npairs.entry((i, k.1)).or_insert(0) += v;
        *ncounts.entry(i).or_insert(0) += v;
    }

    (npairs, ncounts)
}

pub fn solution(counts: &Counts) -> usize {
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn day14(input: String) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();

    let template = lines[0];
    let mut rules = HashMap::new();

    let mut values = Pairs::new();
    let mut counts = Counts::new();
    for i in 1..template.len() {
        let a1 = template.chars().nth(i - 1).unwrap();
        let a2 = template.chars().nth(i).unwrap();

        *values.entry((a1, a2)).or_insert(0) += 1;
    }
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for r in &lines[2..] {
        let mut p = r.split(" -> ");
        let mut a = p.next().unwrap().chars();
        let a1 = a.next().unwrap();
        let a2 = a.next().unwrap();
        let b = p.next().unwrap().chars().next().unwrap();

        rules.insert((a1, a2), b);
    }

    for _ in 0..10 {
        (values, counts) = steps(&rules, &values, &counts);
    }
    let p1 = solution(&counts);

    for _ in 10..40 {
        (values, counts) = steps(&rules, &values, &counts);
    }
    let p2 = solution(&counts);

    (p1, p2)
}

aoc2021::day!(day14, "day14.in", bench_day14);
