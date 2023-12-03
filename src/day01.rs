use std::collections::HashMap;
use std::{fs, str::FromStr};

fn main() {
    let data = fs::read_to_string("data/day01.txt").unwrap();
    println!("day01_part01: {}", day01_part01(&data));
    println!("day01_part02: {}", day01_part02(&data));
}

fn day01_part01(data: &str) -> u32 {
    data.lines().map(number_from_boundaries).sum::<u32>()
}

fn day01_part02(data: &str) -> u32 {
    let mappings = HashMap::from([
        // ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    data.lines()
        .map(|s| replace_ascii_digits(&mappings, s))
        .map(|s| number_from_boundaries(&s))
        .sum::<u32>()
}

/// Form a `u32` by concatenating numbers from a `String`s left and right boundaries.
fn number_from_boundaries(s: &str) -> u32 {
    let mut filtered_data = s.trim().chars().filter(|c| c.is_ascii_digit());
    let l = filtered_data.next().unwrap();
    let r = filtered_data.last().unwrap_or(l);
    u32::from_str(&format!("{l}{r}")).unwrap()
}

/// Replace numbers (spelled out with letters) with the corresponding ascii digits.
fn replace_ascii_digits(mappings: &HashMap<&str, &str>, s: &str) -> String {
    let mut matches = mappings
        .iter()
        .flat_map(|(k, _)| s.match_indices(k)) // fetch all indices since there could be more than one occurrence
        .collect::<Vec<(usize, &str)>>();
    matches.sort_by(|(idx1, _), (idx2, _)| idx1.cmp(idx2));

    let mut converted_string = s.to_string();
    for (_, k) in &matches {
        converted_string = converted_string.replacen(k, mappings[k], 1);
    }
    converted_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_part01() {
        let data = r###"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"###;
        assert_eq!(day01_part01(data), 142);
    }

    #[test]
    fn test_day01_part02() {
        let data = r###"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"###;
        assert_eq!(day01_part02(data), 281);
    }
}
