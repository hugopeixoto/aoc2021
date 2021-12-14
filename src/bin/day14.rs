#![feature(destructuring_assignment)]
#![feature(test)]
extern crate test;

use std::collections::HashMap;

type Rules = HashMap<(char, char), char>;
type Pairs = HashMap<(char, char), usize>;
type Counts = HashMap<char, usize>;

pub fn steps(rules: &Rules, pairs: &Pairs) -> Pairs {
    let mut npairs = Pairs::with_capacity(100);

    for (k, v) in pairs.iter() {
        let i = rules[k];
        *npairs.entry((k.0, i)).or_insert(0) += v;
        *npairs.entry((i, k.1)).or_insert(0) += v;
    }

    npairs
}

pub fn solution(template: &Vec<char>, pairs: &Pairs) -> usize {
    let mut counts = Counts::new();
    for (&(a,b), k) in pairs.iter() {
        *counts.entry(a).or_insert(0) += k;
        *counts.entry(b).or_insert(0) += k;
    }

    *counts.entry(template[0]).or_insert(0) += 1;
    *counts.entry(template[template.len() - 1]).or_insert(0) += 1;

    (counts.values().max().unwrap() - counts.values().min().unwrap()) / 2
}

pub fn day14(input: String) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();

    let template = lines[0].chars().collect::<Vec<_>>();
    let mut pairs = Pairs::new();
    for i in 1..template.len() {
        let a1 = template[i - 1];
        let a2 = template[i];

        *pairs.entry((a1, a2)).or_insert(0) += 1;
    }

    let mut rules = Rules::new();
    for r in &lines[2..] {
        let mut p = r.split(" -> ");
        let mut a = p.next().unwrap().chars();
        let a1 = a.next().unwrap();
        let a2 = a.next().unwrap();
        let b = p.next().unwrap().chars().next().unwrap();

        rules.insert((a1, a2), b);
    }

    for _ in 0..10 {
        pairs = steps(&rules, &pairs);
    }
    let p1 = solution(&template, &pairs);

    for _ in 10..40 {
        pairs = steps(&rules, &pairs);
    }
    let p2 = solution(&template, &pairs);

    (p1, p2)
}

aoc2021::day!(day14, "day14.in", bench_day14);
