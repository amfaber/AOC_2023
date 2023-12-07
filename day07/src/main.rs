use std::{collections::BTreeMap, cmp::Reverse, time::Instant};

use shared::{get_day_input, lazy_input, Lazy};
lazy_input! {7}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Card(u8);

#[derive(Clone, Copy)]
enum Part{
    One,
    Two,
}

impl Card{
    fn parse(c: char, part: Part) -> Self{
        match c{
            'T' => Self(10),
            'J' => match part{
                Part::One => Self(11),
                Part::Two => Self(0),
            },
            'Q' => Self(12),
            'K' => Self(13),
            'A' => Self(14),
            _ => Self(c.to_digit(10).unwrap() as u8),
        }
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    sorted_count: Vec<i32>,
    cards: Vec<Card>,
}

impl Hand{
    fn parse(s: &str, part: Part) -> Self{
        let cards = s.chars().map(|c| Card::parse(c, part)).collect::<Vec<_>>();
        let mut map = BTreeMap::new();
        for card in &cards{
            *map.entry(card).or_insert(0) += 1;
        }
        let sorted_count = match part{
            Part::One => {
                let mut count = map.into_iter().map(|(_, count)| count).collect::<Vec<_>>();
                count.sort_unstable_by_key(|count| Reverse(*count));
                count
            },
            Part::Two => {
                let mut count = map.into_iter().collect::<Vec<_>>();
                count.sort_unstable_by_key(|(_, count)| Reverse(*count));
                let jack = Card::parse('J', Part::Two);
                if let Some(idx) = count.iter().position(|(card, _)|{
                    **card == jack
                }){
                    if count.len() > 1{
                        let (_, jack_count) = count.remove(idx);
                        count.first_mut().unwrap().1 += jack_count;
                    }
                }
                count.into_iter().map(|(_, count)| count).collect::<Vec<_>>()
            },
        };
        Self{
            cards,
            sorted_count,
        }
    }
}

const _TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

fn execute(part: Part){
    // let mut games = _TEST_INPUT.lines().map(|line| {
    let mut games = INPUT.lines().map(|line| {
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = Hand::parse(hand, part);
        let bid = bid.parse::<usize>().unwrap();
        (hand, bid)
    }).collect::<Vec<_>>();

    games.sort_unstable_by(|(hand1, _), (hand2, _)|{
        hand1.cmp(hand2)
    });

    let out = games.into_iter().enumerate().map(|(rank, (_, bid))| (rank + 1) * bid).sum::<usize>();

    dbg!(out);
}

fn main() {
    Lazy::force(&INPUT);
    let now = Instant::now();
    execute(Part::One);
    dbg!(now.elapsed());
    let now = Instant::now();
    execute(Part::Two);
    dbg!(now.elapsed());
}
