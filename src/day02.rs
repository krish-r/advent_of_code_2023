use std::fs;

fn main() {
    let data = fs::read_to_string("data/day02.txt").unwrap();
    println!("day02_part01: {}", day02_part01(&data));
    println!("day02_part02: {}", day02_part02(&data));
}

fn day02_part01(data: &str) -> i32 {
    data.replace("Game", "")
        .replace(' ', "")
        .lines()
        .filter_map(|l| valid_game(l, &(12, 13, 14)))
        .sum()
}

fn day02_part02(data: &str) -> i32 {
    data.replace("Game", "")
        .replace(' ', "")
        .lines()
        .map(power_of_sets)
        .sum()
}

/// Returns the id if the subsets are valid.
fn valid_game(data: &str, rgb: &(i32, i32, i32)) -> Option<i32> {
    let (id, subsets) = data.split_once(':').unwrap();

    let splits = subsets.split(';').collect::<Vec<&str>>();
    for s in splits {
        let map = as_rgb(s);
        if map.0 > rgb.0 || map.1 > rgb.1 || map.2 > rgb.2 {
            return None;
        }
    }
    Some(id.parse::<i32>().unwrap())
}

/// Parse and Return tuple of RGB from data.
fn as_rgb(data: &str) -> (i32, i32, i32) {
    // split and iterate since each subset has a different size.
    data.split(',')
        .map(|s| {
            // check for double-digit number
            if s.chars().nth(1).unwrap().is_ascii_digit() {
                s.split_at(2)
            } else {
                s.split_at(1)
            }
        })
        .fold((0, 0, 0), |acc, (count, color)| {
            let count = count.parse::<i32>().unwrap();
            match color {
                "red" => (acc.0 + count, acc.1, acc.2),
                "green" => (acc.0, acc.1 + count, acc.2),
                "blue" => (acc.0, acc.1, acc.2 + count),
                _ => panic!(),
            }
        })
}

/// Calculate the power of sets.
fn power_of_sets(data: &str) -> i32 {
    let (_, subsets) = data.split_once(':').unwrap();

    let splits = subsets.split(';').collect::<Vec<&str>>();
    let min_required = splits
        .iter()
        .map(|s| as_rgb(s))
        .fold((0, 0, 0), |acc, rgb| {
            (
                std::cmp::max(acc.0, rgb.0),
                std::cmp::max(acc.1, rgb.1),
                std::cmp::max(acc.2, rgb.2),
            )
        });
    min_required.0 * min_required.1 * min_required.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_part01() {
        let data = r###"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"###;

        assert_eq!(day02_part01(data), 8);
    }

    #[test]
    fn test_day02_part02() {
        let data = r###"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"###;

        assert_eq!(day02_part02(data), 2286);
    }
}
