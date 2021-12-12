#![feature(test)]
extern crate test;

pub fn score(board: &[usize], marked: usize, n: usize) -> usize {
  n * board.iter().enumerate().filter(|(j, _n)| marked & 1 << j == 0).map(|(_j, x)| x).sum::<usize>()
}

pub fn day04(input: String) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();

    let sequence = lines[0].split(",").map(|x| x.parse::<usize>().unwrap());

    let boards = lines[2..].iter().flat_map(|line| line.split_whitespace()).map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let boards = boards.chunks(25).collect::<Vec<_>>();

    let mut state: Vec<usize> = (0..boards.len()).map(|_| 0).collect::<Vec<_>>();
    let mut done = vec![];
    let mut winners = vec![];
    done.resize(boards.len(), false);

    for n in sequence {
        for (i, &b) in boards.iter().enumerate() {
            if !done[i] {
                if let Some(p) = b.iter().position(|e| *e == n) {
                    state[i] |= 1 << p;

                    let h_mask = 31 << (p / 5 * 5);
                    let v_mask = 0b00001_00001_00001_00001_00001 << (p % 5);

                    let winner = (state[i] & h_mask) == h_mask || (state[i] & v_mask) == v_mask;

                    if winner {
                        done[i] = true;
                        winners.push((i, n));
                    }
                }
            }
        }
    }

    let first = winners[0];
    let last = *winners.last().unwrap();

    let p1 = score(&boards[first.0], state[first.0], first.1);
    let p2 = score(&boards[last.0], state[last.0], last.1);

    (p1, p2)
}

aoc2021::day!(day04, "day04.in", bench_day04);
