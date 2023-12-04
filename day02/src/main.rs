use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::terminated,
    IResult,
};
use shared::{get_day_input, lazy_input, Lazy};
lazy_input! {2}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Default, Clone, Copy, Debug)]
struct Outcome {
    red: u32,
    green: u32,
    blue: u32,
}

impl Outcome{
    fn max(self, other: Self) -> Self{
        Self{
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(self) -> u64{
        self.red as u64 * self.green as u64 * self.blue as u64
    }
}

fn parse_u32(inp: &str) -> IResult<&str, u32> {
    Ok(map_res(digit1, str::parse::<u32>)(inp)?)
}

fn parse_color(inp: &str) -> IResult<&str, Color> {
    let (inp, amount) = terminated(parse_u32, tag(" "))(inp)?;
    let (inp, color) = alt((
        map(tag("blue"), |_| Color::Blue(amount)),
        map(tag("red"), |_| Color::Red(amount)),
        map(tag("green"), |_| Color::Green(amount)),
    ))(inp)?;

    Ok((inp, color))
}

fn parse_outcome(mut inp: &str) -> IResult<&str, Outcome> {
    let mut outcome = Outcome::default();
    while let Ok((mut new_inp, color)) = parse_color(inp) {
        match color {
            Color::Red(amount) => outcome.red = amount,
            Color::Green(amount) => outcome.green = amount,
            Color::Blue(amount) => outcome.blue = amount,
        }
        if let Ok((newest_inp, _)) = tag::<_, _, nom::error::Error<_>>(", ")(new_inp){
            new_inp = newest_inp;
        }
        inp = new_inp;
    }
    Ok((inp, outcome))
}

#[derive(Debug)]
struct Game {
    id: u32,
    outcomes: Vec<Outcome>,
}

fn parse_game(inp: &str) -> IResult<&str, Game> {
    let (inp, _) = tag("Game ")(inp)?;
    let (inp, id) = terminated(parse_u32, tag(": "))(inp)?;
    let (inp, outcomes) = separated_list0(tag("; "), parse_outcome)(inp)?;

    let game = Game { id, outcomes };
    Ok((inp, game))
}

fn parse_input(inp: &str) -> Vec<Game> {
    let (_, games) = separated_list0(tag("\n"), parse_game)(inp).unwrap();
    games
}

fn outcome_is_possible(condition: Outcome, outcome: Outcome) -> bool {
    outcome.red <= condition.red
        && outcome.green <= condition.green
        && outcome.blue <= condition.blue
}

fn part1() {
    let games = parse_input(&INPUT);
    let condition = Outcome {
        red: 12,
        green: 13,
        blue: 14,
    };

    let sum = games
        .iter()
        .flat_map(|game| {
            game.outcomes
                .iter()
                .fold(true, |acc, &outcome| {
                    acc && outcome_is_possible(condition, outcome)
                })
                .then_some(game.id)
        })
        .sum::<u32>();

    dbg!(sum);
}

fn part2() {
    let games = parse_input(&INPUT);
    let sum = games
        .iter()
        .map(|game| {
            game.outcomes
                .iter()
                .copied()
                .reduce(|acc, outcome| {
                    acc.max(outcome)
                }).unwrap_or_default().power()
        })
        .sum::<u64>();

    dbg!(sum);
}

fn main() {
    part1();
    part2();
}
