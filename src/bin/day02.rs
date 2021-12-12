#![feature(test)]
extern crate test;

pub fn day02(input: String) -> (usize, usize) {
    let commands: Vec<_> = input.lines().map(|x| {
        let mut pieces = x.split(" ");
        let direction = pieces.next().unwrap();
        let amount = pieces.next().unwrap().parse::<usize>().unwrap();

        (direction.to_string(), amount)
    }).collect();

    let mut position = (0, 0);

    for (direction, amount) in commands.iter() {
        match direction.as_str() {
            "down" => { position.1 += amount; },
            "up" => { position.1 -= amount; },
            "forward" => { position.0 += amount; },
            _ => { panic!("invalid direction"); },
        }
    }

    let mut position2 = (0, 0, 0);

    for (direction, amount) in commands.iter() {
        match direction.as_str() {
            "down" => { position2.2 += amount; },
            "up" => { position2.2 -= amount; },
            "forward" => { position2.0 += amount; position2.1 += amount * position2.2; },
            _ => { panic!("invalid direction"); },
        }
    }

    (position.0 * position.1, position2.0 * position2.1)
}

aoc2021::day!(day02, "day02.in", bench_day02);
