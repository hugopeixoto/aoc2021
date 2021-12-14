#![feature(test)]
extern crate test;

type Edges = Vec<(bool, Vec<usize>)>;

pub trait State<T> {
    fn starting(node: usize) -> Self;
    fn node(&self) -> usize;
    fn next(&self, edges: &Edges, node: usize) -> Option<T>;
}

#[derive(Clone, Copy, Default)]
pub struct State1 {
    node: usize,
    path: usize,
}

impl State<State1> for State1 {
    fn starting(node: usize) -> Self {
        State1 { node, path: 1 << node }
    }

    fn node(&self) -> usize {
        self.node
    }

    fn next(&self, edges: &Edges, node: usize) -> Option<Self> {
        let npath = self.path | 1 << node;
        if edges[node].0 || npath != self.path {
            Some(State1 { node, path: npath })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct State2 {
    node: usize,
    path: usize,
    dobles: bool,
}

impl State<State2> for State2 {
    fn starting(node: usize) -> Self {
        State2 { node, path: 1 << node, dobles: false }
    }

    fn node(&self) -> usize {
        self.node
    }

    fn next(&self, edges: &Edges, node: usize) -> Option<Self> {
        let npath = self.path | 1 << node;
        let requires_doble = edges[node].0 == false && npath == self.path;
        if !requires_doble || !self.dobles {
            Some(State2 { node, path: npath, dobles: self.dobles || requires_doble })
        } else {
            None
        }
    }
}

pub fn calc<S: State<S> + Default + Copy>(edges: &Edges, start: usize, target: usize) -> usize {
    let mut stack = [S::default(); 100]; // (:
    let mut ss = 0;

    stack[0] = S::starting(start);
    ss += 1;

    let mut total = 0;
    while ss > 0 {
        ss -= 1;
        let state = stack[ss];

        for next in edges[state.node()].1.iter() {
            if *next != start {
                match state.next(&edges, *next) {
                    Some(nstate) => {
                        if *next == target {
                            total += 1;
                        } else {
                            stack[ss] = nstate;
                            ss += 1;
                        }
                    },
                    None => {}
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

    let p1 = calc::<State1>(&edges, ids["start"], ids["end"]);
    let p2 = calc::<State2>(&edges, ids["start"], ids["end"]);

    (p1, p2)
}

aoc2021::day!(day12, "day12.in", bench_day12);
