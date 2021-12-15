#![feature(destructuring_assignment)]
#![feature(test)]
extern crate test;

use std::collections::BinaryHeap;
use std::collections::HashSet;

pub trait Graph {
    fn is_valid(&self, point: &(i32, i32)) -> bool;
    fn target(&self) -> (i32, i32);
    fn risk(&self, point: &(i32, i32)) -> i32;
    fn w(&self) -> i32;
    fn h(&self) -> i32;
}

struct Part1 {
    risk: Vec<Vec<i32>>,
}

impl Graph for Part1 {
    fn w(&self) -> i32 {
        self.risk[0].len() as i32
    }
    fn h(&self) -> i32 {
        self.risk.len() as i32
    }
    fn is_valid(&self, point: &(i32, i32)) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.w() && point.1 < self.h()
    }
    fn target(&self) -> (i32, i32) {
        (self.w() - 1, self.h() - 1)
    }
    fn risk(&self, point: &(i32, i32)) -> i32 {
        self.risk[point.1 as usize][point.0 as usize]
    }
}

struct Part2 {
    risk: Vec<Vec<i32>>,
}

impl Graph for Part2 {
    fn w(&self) -> i32 {
        self.risk[0].len() as i32 * 5
    }
    fn h(&self) -> i32 {
        self.risk.len() as i32 * 5
    }
    fn is_valid(&self, point: &(i32, i32)) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.w() && point.1 < self.h()
    }
    fn target(&self) -> (i32, i32) {
        (self.w() - 1, self.h() - 1)
    }
    fn risk(&self, point: &(i32, i32)) -> i32 {
        let increment = point.1 / (self.h()/5) + point.0 / (self.w()/5);
        let original = self.risk[(point.1 % (self.h()/5)) as usize][(point.0 % (self.w()/5)) as usize];
        let overflowed = original + increment;
        let wrapped = (overflowed - 1) % 9 + 1;
        wrapped
    }
}

pub fn shortest_path(graph: &dyn Graph) -> usize {
    let target = graph.target();

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push((0i32, (0, 0)));

    let mut p1 = 0;
    while !queue.is_empty() {
        let node = queue.pop().unwrap();

        if visited.contains(&node.1) {
            continue;
        }

        visited.insert(node.1);

        if node.1 == target {
            p1 = node.0.abs() as usize;
            break;
        }

        let next = (node.1.0 - 1, node.1.1); if graph.is_valid(&next) && !visited.contains(&next) { queue.push((node.0 - graph.risk(&next), next)); }
        let next = (node.1.0 + 1, node.1.1); if graph.is_valid(&next) && !visited.contains(&next) { queue.push((node.0 - graph.risk(&next), next)); }
        let next = (node.1.0, node.1.1 - 1); if graph.is_valid(&next) && !visited.contains(&next) { queue.push((node.0 - graph.risk(&next), next)); }
        let next = (node.1.0, node.1.1 + 1); if graph.is_valid(&next) && !visited.contains(&next) { queue.push((node.0 - graph.risk(&next), next)); }
    }

    p1
}


pub fn day15(input: String) -> (usize, usize) {
    let lines = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>()).collect::<Vec<_>>();

    let g1 = Part1 { risk: lines.clone() };
    let p1 = shortest_path(&g1);

    let g2 = Part2 { risk: lines.clone() };
    let p2 = shortest_path(&g2);

    (p1, p2)
}

aoc2021::day!(day15, "day15.in", bench_day15);
