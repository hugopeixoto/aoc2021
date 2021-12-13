#![feature(test)]
extern crate test;

use std::collections::HashSet;

pub fn fold_x(p: (i32, i32), along: i32) -> (i32, i32) {
    if p.0 > along {
        (-p.0 + 2*along, p.1)
    } else {
        p
    }
}

pub fn fold_y(p: (i32, i32), along: i32) -> (i32, i32) {
    if p.1 > along {
        (p.0, -p.1 + 2*along)
    } else {
        p
    }
}

pub fn fold(p: (i32, i32), along: (char, i32)) -> (i32, i32) {
    if along.0 == 'x' {
        fold_x(p, along.1)
    } else {
        fold_y(p, along.1)
    }
}

pub fn draw(points: &Vec<(i32, i32)>) {
    let ll = points.iter().fold((0, 0), |acc, p| (acc.0.min(p.0), acc.1.min(p.1)));
    let tr = points.iter().fold((0, 0), |acc, p| (acc.0.max(p.0), acc.1.max(p.1)));

    for y in ll.1 ..= tr.1 {
        for x in ll.0 ..= tr.0 {
            if points.iter().find(|&&(px,py)| px == x && py == y).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}


pub fn day13(input: String) -> (usize, usize) {
    let mut points = vec![];

    let mut folds = vec![];

    for line in input.lines() {
        match line.chars().next() {
            Some('f') => {
                let axis = line.chars().nth(11).unwrap();
                let n = line[13..].parse::<i32>().unwrap();

                folds.push((axis, n));
            },
            Some(_) => {
                let mut parts = line.split(',');
                let x = parts.next().unwrap().parse::<i32>().unwrap();
                let y = parts.next().unwrap().parse::<i32>().unwrap();

                points.push((x, y));
            },
            None => {},
        }
    }

    for f in folds[0..=0].iter() {
        for p in points.iter_mut() {
            *p = fold(*p, *f);
        }
    }

    let p1 = points.iter().collect::<HashSet<_>>().len();

    for f in folds[1..].iter() {
        for p in points.iter_mut() {
            *p = fold(*p, *f);
        }
    }

    draw(&points);

    (p1, 0)
}

aoc2021::day!(day13, "day13.in", bench_day13);
