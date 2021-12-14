#![feature(test)]
extern crate test;

pub fn calc(edges: &Vec<(bool, Vec<usize>)>, start: usize, target: usize) -> usize {
    let mut stack = Vec::with_capacity(100);

    stack.push((start, 1 << start));
    let mut total = 0;
    while !stack.is_empty() {
        let (current, path) = stack.pop().unwrap();
        for next in edges[current].1.iter().rev() {
            let npath = path | 1 << next;
            if edges[*next].0 || npath != path {
                if *next == target {
                    total += 1;
                } else {
                    stack.push((*next, npath));
                }
            }
        }
    }

    total
}

pub fn calc2(edges: &Vec<(bool, Vec<usize>)>, start: usize, target: usize) -> usize {
    let mut stack = Vec::with_capacity(100);

    stack.push((start, 1 << start, false));
    let mut total = 0;
    while !stack.is_empty() {
        let (current, path, dobles) = stack.pop().unwrap();

        for next in edges[current].1.iter().rev() {
            if *next != start {
                let npath = path | 1 << next;
                let seen = npath == path;
                let requires_doble = edges[*next].0 == false && seen;
                if !requires_doble || !dobles {
                    if *next == target {
                        total += 1;
                    } else {
                        stack.push((*next, npath, dobles || requires_doble));
                    }
                }
            }
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

    for e in edges.iter_mut() {
        e.1.sort();
    }

    let p1 = calc(&edges, ids["start"], ids["end"]);
    let p2 = calc2(&edges, ids["start"], ids["end"]);

    (p1, p2)
}

aoc2021::day!(day12, "day12.in", bench_day12);
