#![feature(test)]
extern crate test;

pub fn calc(edges: &Vec<(bool, Vec<usize>)>, current: usize, mut path: usize, target: usize) -> usize {
    if current == target {
        return 1;
    }

    let mut total = 0;
    path |= 1 << current;
    for next in edges[current].1.iter() {
        if edges[*next].0 || path & (1 << next) == 0 {
            total += calc(edges, *next, path, target);
        }
    }

    total
}

pub fn calc2(edges: &Vec<(bool, Vec<usize>)>, current: usize, mut path: usize, dobles: bool, start: usize, target: usize) -> usize {
    if current == target {
        return 1;
    }

    let mut total = 0;
    path |= 1 << current;
    for next in edges[current].1.iter() {
        let seen = path & (1 << next) != 0;
        if *next != start && (edges[*next].0 || !seen || !dobles) {
            total += calc2(edges, *next, path, dobles || (edges[*next].0 == false && seen), start, target);
        }
    }

    total
}

pub fn day12(input: String) -> (usize, usize) {
    let mut ids = std::collections::HashMap::<&str, usize>::new();
    let mut edges = vec![];
    edges.resize(100, (false, vec![]));

    for line in input.lines() {
        let x = line.split('-').collect::<Vec<_>>();
        let p1 = x[0];
        let p2 = x[1];

        if !ids.contains_key(p1) { ids.insert(p1, ids.len()); }
        if !ids.contains_key(p2) { ids.insert(p2, ids.len()); }

        let id1 = ids[p1];
        let id2 = ids[p2];

        edges[id1].0 = p1.chars().next().unwrap().is_ascii_uppercase();
        edges[id2].0 = p2.chars().next().unwrap().is_ascii_uppercase();

        edges[id1].1.push(id2);
        edges[id2].1.push(id1);
    }

    let p1 = calc(&edges, ids["start"], 0, ids["end"]);
    let p2 = calc2(&edges, ids["start"], 0, false, ids["start"], ids["end"]);

    (p1, p2)
}

aoc2021::day!(day12, "day12.in", bench_day12);
