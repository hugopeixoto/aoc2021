#![feature(destructuring_assignment)]
#![feature(test)]
extern crate test;

#[derive(Debug)]
pub enum Packet {
    Literal(usize, usize),
    Operator(usize, usize, Vec<Packet>),
}

pub struct BitStream {
    bits: Vec<u8>,
    index: usize,
}

impl BitStream {
    pub fn consume(&mut self, n: usize) -> usize {
        let r = self.bits[self.index .. self.index + n]
            .iter()
            .fold(0, |a, b| a * 2 + *b as usize);

        self.index += n;
        r
    }
}

pub fn parse(stream: &mut BitStream) -> Packet {
    let version = stream.consume(3);
    let ptype   = stream.consume(3);

    if ptype == 4 {
        let mut value = 0;
        loop {
            let part = stream.consume(5);
            value = (value << 4) | (part & 0xF);
            if part & 0x10 == 0 {
                break;
            }
        }

        Packet::Literal(version, value)
    } else {
        let mut subpackets = vec![];
        if stream.consume(1) == 0 {
            let total_length = stream.consume(15);
            let end = stream.index + total_length;

            while stream.index < end {
                subpackets.push(parse(stream));
            }
        } else {
            let packet_count = stream.consume(11);
            for _ in 0..packet_count {
                subpackets.push(parse(stream));
            }
        }

        Packet::Operator(version, ptype, subpackets)
    }
}

pub fn sum_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(v, _) => *v,
        Packet::Operator(v, _, subpackets) => *v + subpackets.iter().map(|s| sum_versions(s)).sum::<usize>(),
    }
}

pub fn calc(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(_, value) => *value,
        Packet::Operator(_, ptype, subpackets) => {
            let subcalc = subpackets.iter().map(calc);
            match *ptype {
                0 => subcalc.sum(),
                1 => subcalc.product(),
                2 => subcalc.min().unwrap(),
                3 => subcalc.max().unwrap(),
                5 => subcalc.reduce(|acc, v| if acc > v { 1 } else { 0 }).unwrap(),
                6 => subcalc.reduce(|acc, v| if acc < v { 1 } else { 0 }).unwrap(),
                7 => subcalc.reduce(|acc, v| if acc == v { 1 } else { 0 }).unwrap(),
                _ => { panic!("wat"); },
            }
        }
    }
}

pub fn day16(input: String) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let mut bits = vec![];

        for c in line.trim().chars() {
            let byte = c.to_digit(16).unwrap() as u8;
            for i in [3,2,1,0] {
                bits.push((byte >> i) & 1);
            }
        }

        let mut stream = BitStream { bits, index: 0 };
        let packet = parse(&mut stream);

        p1 += sum_versions(&packet);
        p2 += calc(&packet);
    }

    (p1, p2)
}

aoc2021::day!(day16, "day16.in", bench_day16);
