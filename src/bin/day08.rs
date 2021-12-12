#![feature(test)]
extern crate test;

pub fn expect_one<'a, T: std::iter::Iterator<Item=&'a u32>>(text: &str, mut iterator: T) -> u32 {
    let n = iterator.next().unwrap();
    if iterator.next().is_some() {
        panic!("more than one: {}", text);
    } else {
        *n
    }
}

pub fn day08(input: String) -> (usize, usize) {
    let state = input.lines().map(|x| {
        let mut parts = x.split(" | ");
        let digits = parts.next().unwrap();
        let output = parts.next().unwrap();

        let digits = digits.split(" ").map(|d| d.chars().map(|c| c as u8 - 'a' as u8).map(|c| 1u32 << c).reduce(|a, b| a | b).unwrap());
        let output = output.split(" ").map(|d| d.chars().map(|c| c as u8 - 'a' as u8).map(|c| 1u32 << c).reduce(|a, b| a | b).unwrap());

        (digits.collect::<Vec<_>>(), output.collect::<Vec<_>>())
    }).collect::<Vec<_>>();

    let mut p2 = 0usize;
    for (digits, output) in state.iter() {
        let one   = expect_one("1", digits.iter().filter(|&&x| x.count_ones() == 2));
        let seven = expect_one("7", digits.iter().filter(|&&x| x.count_ones() == 3));
        let four  = expect_one("4", digits.iter().filter(|&&x| x.count_ones() == 4));
        let eight = expect_one("8", digits.iter().filter(|&&x| x.count_ones() == 7));

        let nine  = expect_one("9", digits.iter().filter(|&&x| x.count_ones() == 6 && (x & four) == four));
        let zero  = expect_one("0", digits.iter().filter(|&&x| x.count_ones() == 6 && (x & seven) == seven && x != nine));
        let six   = expect_one("6", digits.iter().filter(|&&x| x.count_ones() == 6 && x != zero && x != nine));

        let five  = expect_one("5", digits.iter().filter(|&&x| x.count_ones() == 5 && (x & six) == x));
        let three = expect_one("3", digits.iter().filter(|&&x| x.count_ones() == 5 && (x & one) == one));
        let two   = expect_one("2", digits.iter().filter(|&&x| x.count_ones() == 5 && (x & five).count_ones() == 3));

        p2 += output.iter().enumerate().map(|(idx, &unknown)|
          if unknown == zero { 0 }
          else if unknown == one { 1 }
          else if unknown == two { 2 }
          else if unknown == three { 3 }
          else if unknown == four { 4 }
          else if unknown == five { 5 }
          else if unknown == six { 6 }
          else if unknown == seven { 7 }
          else if unknown == eight { 8 }
          else if unknown == nine { 9 }
          else { panic!("aaah"); } * 10usize.pow(3-idx as u32)
        ).sum::<usize>();
    }

    let p1 = state.iter().map(|(_, output)| output.iter().filter(|x| x.count_ones() == 2 || x.count_ones() == 3 || x.count_ones() == 4 || x.count_ones() == 7).count()).sum();

    (p1, p2)
}

aoc2021::day!(day08, "day08.in", bench_day08);
