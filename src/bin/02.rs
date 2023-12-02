advent_of_code::solution!(2);
use nom::bytes::complete::{take_until, take_while1};
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::Parser;
use nom::{
    bytes::complete::{tag, take},
    *,
};

const MAX_RED: u32 = 12;
const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;

pub fn parse_game_number(input: &str) -> IResult<&str, ()> {
    // Removes the game number (we can use the index instead)
    // "Game 1: 3 blue, 4 red" -> "3 blue, 4 red"
    map(take_until(":").and(take(2_usize)), drop)(input)
}

pub fn split_game(input: &str) -> IResult<&str, Vec<&str>> {
    // Splits a game into a vector of balls shown
    // "3 blue, 4 red; 2 green, 1 red" -> ["3 blue", "4 red", "2 green", "1 red"]
    terminated(
        separated_list0(
            tag("; ").or(tag(", ")),
            take_while1(|x| x != ';' && x != ','),
        ),
        opt(tag(";").or(tag(","))),
    )(input)
}

pub fn parse_game(input: &str) -> Vec<(u32, &str)> {
    let (remaining, _) = parse_game_number(input).unwrap();
    let balls: Vec<(u32, &str)> = split_game(remaining)
        .unwrap()
        .1
        .iter()
        .map(|ball| ball.split_once(' ').expect("no space found"))
        .map(|(value, color)| {
            (
                value.parse::<u32>().expect("could not parse value to u32"),
                color,
            )
        })
        .collect();
    balls
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let balls = parse_game(line);
            balls
                .iter()
                .all(|(value, color)| {
                    let is_possible = match (value, color) {
                        (value, &"red") if value > &MAX_RED => false,
                        (value, &"blue") if value > &MAX_BLUE => false,
                        (value, &"green") if value > &MAX_GREEN => false,
                        _ => true,
                    };
                    is_possible
                })
                .then(|| (index + 1) as u32)
        })
        .flatten()
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let balls = parse_game(line);

            let mut curr_red: u32 = 0;
            let mut curr_blue: u32 = 0;
            let mut curr_green: u32 = 0;

            balls.iter().for_each(|(value, color)| {
                match (value, color) {
                    (value, &"red") if value > &curr_red => curr_red = *value,
                    (value, &"blue") if value > &curr_blue => curr_blue = *value,
                    (value, &"green") if value > &curr_green => curr_green = *value,
                    _ => (),
                };
            });
            curr_red * curr_blue * curr_green
        })
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
