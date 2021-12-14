#![feature(test)]
extern crate test;

use std::collections::HashMap;

pub fn day14(input: String) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();

    let template = lines[0];
    let mut rules = HashMap::new();

    let mut values = HashMap::new();
    let mut counts = HashMap::new();
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
        let mut nvalues = HashMap::new();
        let mut ncounts = counts.clone();

        for (k, v) in values.iter() {
            let i = rules[k];
            *nvalues.entry((k.0, i)).or_insert(0) += v;
            *nvalues.entry((i, k.1)).or_insert(0) += v;
            *ncounts.entry(i).or_insert(0) += v;
        }

        values = nvalues;
        counts = ncounts;
    }

    let mut quantities = counts.values().collect::<Vec<_>>();
    quantities.sort();
    let p1 = quantities[quantities.len()-1] - quantities[0];

    for _ in 10..40 {
        let mut nvalues = HashMap::new();
        let mut ncounts = counts.clone();

        for (k, v) in values.iter() {
            let i = rules[k];
            *nvalues.entry((k.0, i)).or_insert(0) += v;
            *nvalues.entry((i, k.1)).or_insert(0) += v;
            *ncounts.entry(i).or_insert(0) += v;
        }

        values = nvalues;
        counts = ncounts;
    }

    let mut quantities = counts.values().collect::<Vec<_>>();
    quantities.sort();
    let p2 = quantities[quantities.len()-1] - quantities[0];

    (p1, p2)
}

aoc2021::day!(day14, "day14.in", bench_day14);
