use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct PartNum {
    value: u32,
    position: Position,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Location {
    line: usize,
    position: usize,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Pattern {
    value: char,
    location: Location,
}

impl Position {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Location {
    fn new(line: usize, position: usize) -> Self {
        Self { line, position }
    }
}

fn main() {
    let data = fs::read_to_string("data/day03.txt").unwrap();
    println!("day03_part01: {}", day03_part01(&data));
    println!("day03_part02: {}", day03_part02(&data));
}

fn day03_part01(data: &str) -> u32 {
    let patterns = data
        .lines()
        .enumerate()
        .flat_map(|(l, s)| (parse_pattern_positions(l, s)))
        .collect::<Vec<Pattern>>();

    let part_numbers = data
        .lines()
        .enumerate()
        .map(|(l, s)| (l, parse_part_num(s)))
        .collect::<HashMap<usize, HashSet<PartNum>>>();

    let possible_positions = generate_possible_positions(&patterns);

    let mut out = 0;
    let mut visited_parts = HashSet::new();
    for (pattern, Location { line, position }) in possible_positions {
        if let Some(part_number) = part_numbers.get(&line) {
            for part_num in part_number {
                if part_num.position.start <= position
                    && position <= part_num.position.end
                    && !visited_parts.contains(&(pattern, part_num))
                {
                    out += part_num.value;
                    visited_parts.insert((pattern, part_num));
                }
            }
        }
    }

    out
}

fn day03_part02(data: &str) -> u32 {
    let patterns = data
        .lines()
        .enumerate()
        .flat_map(|(l, s)| (parse_pattern_positions(l, s)))
        .collect::<Vec<Pattern>>();

    let part_numbers = data
        .lines()
        .enumerate()
        .map(|(l, s)| (l, parse_part_num(s)))
        .collect::<HashMap<usize, HashSet<PartNum>>>();

    let possible_positions = generate_possible_positions(&patterns);

    let mut visited_parts: HashSet<(&Pattern, &PartNum)> = HashSet::new();
    let mut pattern_map = HashMap::new();
    for (pattern, Location { line, position }) in possible_positions {
        if let Some(part_number) = part_numbers.get(&line) {
            for part_num in part_number {
                if pattern.value == '*'
                    && part_num.position.start <= position
                    && position <= part_num.position.end
                    && !visited_parts.contains(&(pattern, part_num))
                {
                    visited_parts.insert((pattern, part_num));
                    let entry = pattern_map.entry(pattern).or_insert(Vec::new());
                    (*entry).push(part_num.value);
                }
            }
        }
    }

    pattern_map
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum::<u32>()
}

/// Parse pattern positions from a given string.
fn parse_pattern_positions(line_number: usize, s: &str) -> Vec<Pattern> {
    s.chars()
        .enumerate()
        .filter(|(_, pattern)| !(pattern.is_ascii_alphanumeric() || pattern == &'.'))
        .map(|(idx, pattern)| Pattern {
            value: pattern,
            location: Location::new(line_number, idx),
        })
        .collect()
}

/// For a given pattern, generate a list of possible positions.
///
/// x x x
/// x p x
/// x x x
///
fn generate_possible_positions(patterns: &Vec<Pattern>) -> Vec<(&Pattern, Location)> {
    let mut possible_positions = Vec::new();

    for pattern in patterns {
        let line = pattern.location.line;
        let position = pattern.location.position;
        if line != 0 {
            possible_positions.push((pattern, Location::new(line - 1, position)));
            possible_positions.push((pattern, Location::new(line - 1, position + 1)));
        }

        if line != 0 && position != 0 {
            possible_positions.push((pattern, Location::new(line - 1, position - 1)));
        }

        if position != 0 {
            possible_positions.push((pattern, Location::new(line, position - 1)));
            possible_positions.push((pattern, Location::new(line + 1, position - 1)));
        }

        possible_positions.push((pattern, Location::new(line, position + 1)));
        possible_positions.push((pattern, Location::new(line + 1, position)));
        possible_positions.push((pattern, Location::new(line + 1, position + 1)));
    }

    possible_positions
}

/// Parse part numbers from a given string.
fn parse_part_num(s: &str) -> HashSet<PartNum> {
    let mut curr_idx = usize::MAX;
    let mut part_numbers = HashSet::new();

    for (idx, ch) in s.chars().enumerate() {
        match (ch.is_ascii_digit(), curr_idx == usize::MAX) {
            (true, true) => curr_idx = idx,
            (false, false) => {
                part_numbers.insert(PartNum {
                    value: (s[curr_idx..idx]).parse::<u32>().unwrap(),
                    position: Position::new(curr_idx, idx - 1),
                });
                curr_idx = usize::MAX; // reset current idx
            }
            _ => (),
        }
    }

    // add the last set of numbers - if any
    if curr_idx != usize::MAX {
        part_numbers.insert(PartNum {
            value: (s[curr_idx..]).parse::<u32>().unwrap(),
            position: Position::new(curr_idx, s.len() - 1),
        });
    }

    part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03_part01() {
        let data = r###"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"###;

        assert_eq!(day03_part01(data), 4361)
    }

    #[test]
    fn test_day03_part02() {
        let data = r###"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"###;

        assert_eq!(day03_part02(data), 467835)
    }
}
