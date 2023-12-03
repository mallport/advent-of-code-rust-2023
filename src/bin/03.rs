use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

advent_of_code::solution!(3);

fn is_gear_symbol(c: char, is_part_one: bool) -> bool {
    if is_part_one {
        !c.is_numeric() && c != '.'
    } else {
        c == '*'
    }
}

fn is_adjacent(
    input: &Vec<String>,
    x_range: (usize, usize),
    y: usize,
    is_part_one: bool,
) -> Option<(usize, usize)> {
    // Given a single string in the input, find out whether it's adjacent to a gear
    // If adjacent, return Some(x,y)
    for x in x_range.0..x_range.1 {
        for x_inner in x.saturating_sub(1)..min(x + 2, input[y].len()) {
            // Iterate over the 3x3 grid around the current character (if possible)
            for y_inner in y.saturating_sub(1)..min(y + 2, input.len()) {
                let c = input[y_inner].chars().nth(x_inner).unwrap();
                if is_gear_symbol(c, is_part_one) {
                    return Some((x_inner, y_inner));
                }
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut _gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new(); // Unused in this part
    let r: u32 = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let y = &y;
            num_regex
                .find_iter(line)
                .map(|found| {
                    if is_adjacent(&input, (found.start(), found.end()), *y, true).is_some() {
                        found.as_str().parse::<u32>().unwrap()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum();
    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    input.iter().enumerate().for_each(|(y, line)| {
        let y = &y;
        num_regex.find_iter(line).for_each(|found| {
            if let Some((x, y)) = is_adjacent(&input, (found.start(), found.end()), *y, false) {
                let num = found.as_str().parse::<u32>().unwrap();
                gears.entry((x, y)).or_default().push(num);
            }
        });
    });

    let result = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
