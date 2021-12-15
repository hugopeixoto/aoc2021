#![feature(destructuring_assignment)]
#![feature(test)]
extern crate test;

use std::collections::BinaryHeap;

pub trait Graph {
    fn is_valid(&self, point: &(i32, i32)) -> bool;
    fn target(&self) -> (i32, i32);
    fn risk(&self, point: &(i32, i32)) -> usize;
    fn nodes(&self) -> usize;
    fn index(&self, point: &(i32, i32)) -> usize;
}

struct Part1 {
    risk: Vec<Vec<usize>>,
    w: i32,
    h: i32,
}

impl Part1 {
    fn new(risk: &Vec<Vec<usize>>) -> Self {
        Self { risk: risk.clone(), w: risk.len() as i32, h: risk.len() as i32 }
    }
}

impl Graph for Part1 {
    fn nodes(&self) -> usize {
        self.risk.len() * self.risk[0].len()
    }
    fn is_valid(&self, point: &(i32, i32)) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.w && point.1 < self.h
    }
    fn target(&self) -> (i32, i32) {
        (self.w - 1, self.h - 1)
    }
    fn risk(&self, point: &(i32, i32)) -> usize {
        self.risk[point.1 as usize][point.0 as usize]
    }
    fn index(&self, point: &(i32, i32)) -> usize {
        (point.1 * self.w + point.0) as usize
    }
}

struct Part2 {
    risk: Vec<Vec<usize>>,
    w: i32,
    h: i32,
    sw: usize,
    sh: usize,
}

impl Part2 {
    fn new(risk: &Vec<Vec<usize>>) -> Self {
        Self {
            risk: risk.clone(),
            w: risk[0].len() as i32 * 5,
            h: risk.len() as i32 * 5,
            sw: risk[0].len(),
            sh: risk.len(),
        }
    }
}

impl Graph for Part2 {
    fn nodes(&self) -> usize {
        self.risk.len() * self.risk[0].len() * 25
    }
    fn is_valid(&self, point: &(i32, i32)) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.w && point.1 < self.h
    }
    fn target(&self) -> (i32, i32) {
        (self.w - 1, self.h - 1)
    }
    fn risk(&self, point: &(i32, i32)) -> usize {
        let x = point.0 as usize;
        let y = point.1 as usize;
        let increment = x / self.sw + y / self.sh;
        let original = self.risk[y % self.sh][x % self.sw];
        let overflowed = original + increment;
        let wrapped = (overflowed - 1) % 9 + 1;
        wrapped
    }
    fn index(&self, point: &(i32, i32)) -> usize {
        (point.0 * self.h + point.1) as usize
    }
}

#[derive(Eq)]
struct Node {
    risk: usize,
    point: (i32, i32),
}

impl Ord for Node { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.risk.cmp(&other.risk).reverse() } }
impl PartialOrd for Node { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
impl PartialEq for Node { fn eq(&self, other: &Self) -> bool { self.risk == other.risk } }

pub fn shortest_path(graph: &dyn Graph) -> usize {
    let target = graph.target();

    let mut visited = vec![];
    let mut queue = BinaryHeap::new();

    visited.resize(graph.nodes(), false);
    queue.reserve(graph.nodes());

    queue.push(Node { risk: 0, point: (0, 0) });

    let mut p1 = 0;
    while !queue.is_empty() {
        let node = queue.pop().unwrap();

        if visited[graph.index(&node.point)] {
            continue;
        }

        visited[graph.index(&node.point)] = true;

        if node.point == target {
            p1 = node.risk;
            break;
        }

        let next = (node.point.0 - 1, node.point.1); if graph.is_valid(&next) && !visited[graph.index(&next)] { queue.push(Node { risk: node.risk + graph.risk(&next), point: next }); }
        let next = (node.point.0 + 1, node.point.1); if graph.is_valid(&next) && !visited[graph.index(&next)] { queue.push(Node { risk: node.risk + graph.risk(&next), point: next }); }
        let next = (node.point.0, node.point.1 - 1); if graph.is_valid(&next) && !visited[graph.index(&next)] { queue.push(Node { risk: node.risk + graph.risk(&next), point: next }); }
        let next = (node.point.0, node.point.1 + 1); if graph.is_valid(&next) && !visited[graph.index(&next)] { queue.push(Node { risk: node.risk + graph.risk(&next), point: next }); }
    }

    p1
}

pub fn day15(input: String) -> (usize, usize) {
    let lines = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>();

    let g1 = Part1::new(&lines);
    let p1 = shortest_path(&g1);

    let g2 = Part2::new(&lines);
    let p2 = shortest_path(&g2);

    (p1, p2)
}

aoc2021::day!(day15, "day15.in", bench_day15);
