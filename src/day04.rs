use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let data = fs::read_to_string("data/day04.txt").unwrap();
    println!("day04_part01: {}", day04_part01(&data));
    println!("day04_part02: {}", day04_part02(&data));
}

fn day04_part01(data: &str) -> u32 {
    data.lines()
        .map(|s| s.split_once(':').unwrap().1)
        .map(find_matches)
        .map(|n| match n {
            0 => 0,
            _ => 2_u32.pow(n - 1),
        })
        .sum::<u32>()
}

fn day04_part02(data: &str) -> u32 {
    let mut card_counts = HashMap::new();

    let total_cards = data.lines().count();

    for (idx, s) in data
        .lines()
        .enumerate()
        .map(|(idx, s)| (idx, s.split_once(':').unwrap().1))
    {
        let m = find_matches(s) as usize;

        let copies = *(card_counts.entry(idx).or_insert(1));

        for i in (idx + 1)..=(idx + m).min(total_cards) {
            *(card_counts.entry(i).or_insert(1)) += copies;
        }
    }

    card_counts.values().sum::<u32>()
}

/// For a given string, find the count of values which are common on either side of the delimiter (`|`)
fn find_matches(s: &str) -> u32 {
    let (left, right) = s.split_once('|').unwrap();

    let left = parse_numbers(left);
    let right = parse_numbers(right);

    (left.intersection(&right).collect::<HashSet<&u32>>().len()) as u32
}

/// Parse numbers from a given string delimited by whitespace
fn parse_numbers(data: &str) -> HashSet<u32> {
    data.split_ascii_whitespace()
        .map(|n| n.parse::<u32>())
        .filter_map(|n| n.ok())
        .collect::<HashSet<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04_part01() {
        let data = r###"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"###;

        assert_eq!(day04_part01(data), 13)
    }

    #[test]
    fn test_day04_part02() {
        let data = r###"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"###;

        assert_eq!(day04_part02(data), 30)
    }
}
