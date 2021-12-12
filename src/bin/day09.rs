#![feature(test)]
extern crate test;

const DELTAS: [(i32, i32); 4] = [
    ( 0, -1),
    (-1,  0),
    ( 1,  0),
    ( 0,  1),
];

pub fn day09(input: String) -> (usize, usize) {
    let board = input.lines().map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>();

    let xrange = 0..board.len() as i32;
    let yrange = 0..board[0].len() as i32;

    let mut risk = 0;
    let mut lowpoints = vec![];

    for x in xrange.clone() {
        for y in yrange.clone() {
            let v = board[x as usize][y as usize];
            if DELTAS.iter()
                .map(|(dx,dy)| (x + dx, y + dy))
                .filter(|(nx,ny)| xrange.contains(nx) && yrange.contains(ny))
                .filter(|&(nx,ny)| board[nx as usize][ny as usize] <= v).count() == 0 {
                lowpoints.push((x, y));
                risk += 1 + v;
            }
        }
    }

    let mut seen = vec![];
    seen.resize(board.len() * board[0].len(), false);

    let mut counts = vec![];
    counts.resize(lowpoints.len(), 0);

    let mut queue = std::collections::VecDeque::new();
    for lowpoint in lowpoints.into_iter().enumerate() {
        queue.push_back(lowpoint);
    }

    while !queue.is_empty() {
        let (id, point) = queue.pop_front().unwrap();

        let idx = point.0 as usize + point.1 as usize * board.len();

        if seen[idx] { continue; }
        seen[idx] = true;
        counts[id] += 1;

        for (dx, dy) in DELTAS.iter() {
            let nx = point.0 + dx;
            let ny = point.1 + dy;

            if xrange.contains(&nx) && yrange.contains(&ny) {
                let nv = board[nx as usize][ny as usize];

                if nv != 9 && nv >= board[point.0 as usize][point.1 as usize] {
                    queue.push_back((id, (nx, ny)));
                }
            }
        }
    }

    counts.sort();

    let p2 = counts.iter().rev().take(3).fold(1, |a,b| a*b);

    (risk, p2)
}

aoc2021::day!(day09, "day09.in", bench_day09);
