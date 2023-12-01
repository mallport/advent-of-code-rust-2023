use std::cmp::min;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let nums = line.chars().filter(|c| c.is_numeric());
                format!(
                    "{}{}",
                    nums.clone().next().expect("no numeric found"),
                    nums.last().expect("only found one numeric")
                )
                .parse::<u32>()
                .expect("failed to parse concatenated numerics")
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // General idea:
    // For each line, iterate through the chars
    // 1) If char is numeric, push it to a vector
    // 2) Else, search for every number that is spelled out starting at the current index.

    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    Some(
        input
            .lines()
            .map(|line| {
                let mut nums: Vec<char> = Vec::with_capacity(line.len());
                for (i, c) in line.chars().enumerate() {
                    if c.is_numeric() {
                        nums.push(c);
                    }
                    // Search for numbers that are spelled out starting at index 'i'
                    else if let Some((num_i, _)) = NUMBERS
                        .iter()
                        .enumerate()
                        .map(|(num_i, n)| {
                            (
                                num_i,
                                // We use min() to avoid reading chars past the end of the line
                                line[i..min(i + n.len(), i + line[i..].len())].find(n),
                            )
                        })
                        .filter(|(_, n)| n.is_some())
                        .next()
                    {
                        nums.push(
                            char::from_digit((num_i + 1) as u32, 10)
                                .expect("unable to parse as digit"),
                        );
                    }
                }
                format!(
                    "{}{}",
                    nums.first().expect("no numeric found"),
                    nums.last().expect("only found one numeric")
                )
                .parse::<u32>()
                .unwrap()
            })
            .sum(),
    )
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
