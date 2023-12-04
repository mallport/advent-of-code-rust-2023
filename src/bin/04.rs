advent_of_code::solution!(4);
use std::{cmp::max, collections::HashMap};

use nom::{bytes::complete::take_while, combinator::map, IResult, Parser};

pub fn strip_game_number(input: &str) -> IResult<&str, ()> {
    // Removes the game number (we can use the index instead)
    map(
        take_while(|c| c != ':').and(take_while(|c: char| !c.is_numeric())),
        drop,
    )(input)
}

fn split_to_vec(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    // Splits input into a vector, where each element in the outer vec represents
    // a vector of (winning, nums) scratch-cards
    input
        .lines()
        .map(|line| strip_game_number(line).unwrap().0)
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(winning, nums)| (winning.split_whitespace(), nums.split_whitespace()))
        .map(|(winning, nums)| {
            (
                winning.map(|str| str.parse::<u32>().unwrap()),
                nums.map(|str| str.parse::<u32>().unwrap()),
            )
        })
        .map(|(winning, nums)| (winning.collect::<Vec<u32>>(), nums.collect::<Vec<u32>>()))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    // General strategy:
    // For each card, for each number, check if the number is in the winning card
    // If it is, double the current score

    let cards: Vec<(Vec<u32>, Vec<u32>)> = split_to_vec(input);
    let result: u32 =
        cards
            .iter()
            .map(|(winning, nums)| {
                nums.iter().fold(0, |sum, num| {
                    if winning.contains(num) {
                        max(1, sum * 2)
                    } else {
                        sum
                    }
                })
            })
            .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    // General strategy:
    // Maintain a hashmap of scratch cards,
    // where the value represents (original + copies) number of scratch_cards found
    // Iterate through each card, updating the hashmap with the number of copies
    // Finally, sum the entire hashmap to get the final count

    let input = split_to_list(input);
    let mut scratch_cards: HashMap<usize, u32> = HashMap::new();
    for i in 0..input.len() {
        // Initialize hashmap as having 1 original copy of a scratch card
        scratch_cards.insert(i, 1);
    }
    input.iter().enumerate().for_each(|(i, (winning, nums))| {
        let num_copies = scratch_cards.get(&i).unwrap().clone();
        let num_matches = nums.iter().filter(|num| winning.contains(num)).count();
        for cards_i in (i + 1)..(i + 1 + num_matches) {
            *scratch_cards.entry(cards_i).or_default() += num_copies;
        }
    });
    Some(scratch_cards.values().sum::<u32>())
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
