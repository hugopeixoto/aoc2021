#![feature(test)]
extern crate test;

pub fn day05(input: String) -> (usize, usize) {
    let segments = input.lines().map(|x| {
        let mut parts = x.split(" -> ");
        let start = parts.next().unwrap();
        let finish = parts.next().unwrap();

        let start = start.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let finish = finish.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

        (start[0], start[1], finish[0], finish[1])
    })
    .map(|(a,b,c,d)| (a, b, c as i32 - a as i32, d as i32 - b as i32))
    .map(|(x,y,dx,dy)| (x as i32, y as i32, dx.signum(), dy.signum(), dx.abs().max(dy.abs())))
    .collect::<Vec<_>>();

    let len = segments.iter().map(|(x,y,dx,dy,l)| (x+dx*l).max(y+dy*l)).max().unwrap() + 1;

    let mut counts = vec![];
    counts.resize(len.pow(2) as usize, 0);

    for &(x,y,dx,dy,l) in segments.iter().filter(|(_,_,dx,dy,_)| *dx == 0 || *dy == 0) {
        for step in 0..=l {
            counts[((x+dx*step) * len + (y+dy*step)) as usize] += 1;
        }
    }

    let p1 = counts.iter().filter(|&&v| v > 1).count();

    for &(x,y,dx,dy,l) in segments.iter().filter(|(_,_,dx,dy,_)| *dx != 0 && *dy != 0) {
        for step in 0..=l {
            counts[((x+dx*step) * len + (y+dy*step)) as usize] += 1;
        }
    }

    let p2 = counts.iter().filter(|&&v| v > 1).count();

    (p1, p2)
}

aoc2021::day!(day05, "day05.in", bench_day05);
