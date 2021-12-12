#![feature(test)]
extern crate test;

pub fn day10(input: String) -> (usize, usize) {
    let state = input.lines().map(|x| x.trim()).collect::<Vec<_>>();

    let mut mapping = std::collections::HashMap::new();
    mapping.insert('}', '{');
    mapping.insert(']', '[');
    mapping.insert(')', '(');
    mapping.insert('>', '<');

    let mut error_scores = std::collections::HashMap::new();
    error_scores.insert('}', 1197);
    error_scores.insert(']', 57);
    error_scores.insert(')', 3);
    error_scores.insert('>', 25137);

    let mut autocomplete_scores = std::collections::HashMap::new();
    autocomplete_scores.insert('(', 1usize);
    autocomplete_scores.insert('[', 2usize);
    autocomplete_scores.insert('{', 3usize);
    autocomplete_scores.insert('<', 4usize);

    let mut error_score = 0;
    let mut autocomplete_scores_by_line = vec![];
    for line in state.iter() {
        let mut stack = vec![];
        let mut good = true;
        for c in line.chars() {
            match c {
                '[' | '(' | '{' | '<' => { stack.push(c) },
                ']' | ')' | '}' | '>' => {
                    if stack.pop().unwrap() != mapping[&c] {
                        good = false;
                        error_score += error_scores[&c];
                    }
                },
                _ => { panic!("wat"); },
            }
        }

        if good {
            autocomplete_scores_by_line.push(stack.iter().rev().map(|c| { autocomplete_scores[c] }).fold(0, |score, v| score * 5 + v));
        }
    }

    autocomplete_scores_by_line.sort();

    (error_score, autocomplete_scores_by_line[autocomplete_scores_by_line.len() / 2])
}

aoc2021::day!(day10, "day10.in", bench_day10);
