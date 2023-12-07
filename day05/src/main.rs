use std::{
    collections::HashSet,
    ops::{Range, RangeInclusive},
    time::Instant,
};

use shared::{get_day_input, lazy_input, Lazy};
lazy_input! {5}

#[derive(Debug, Clone)]
struct MapEntry {
    input_range: RangeInclusive<i64>,
    output_delta: i64,
}

impl MapEntry {
    fn apply(&self) -> RangeInclusive<i64> {
        *self.input_range.start() + self.output_delta..=self.input_range.end() + self.output_delta
    }
}

const _TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

fn part1() {
    let mut categories = INPUT.split("\n\n");
    let seeds = categories
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|seed| seed.parse::<i64>().ok());

    let maps = categories
        .map(|category| {
            let mut lines = category.lines();
            lines.next();
            let map = lines
                .map(|line| {
                    let mut elems = line.split(" ").map(|num| num.parse::<i64>().unwrap());
                    let dest_start = elems.next().unwrap();
                    let source_start = elems.next().unwrap();
                    let n = elems.next().unwrap();
                    MapEntry {
                        input_range: source_start..=source_start + n - 1,
                        output_delta: dest_start - source_start,
                    }
                })
                .collect::<Vec<_>>();
            map
        })
        .collect::<Vec<_>>();

    let out = seeds
        .map(|mut input| {
            for map in &maps {
                for entry in map {
                    if entry.input_range.contains(&input) {
                        input += entry.output_delta;
                        break;
                    }
                }
            }
            input
        })
        .min().unwrap();

    dbg!(out);
}

fn range_contains_range(seeds: &RangeInclusive<i64>, input: &RangeInclusive<i64>) -> (bool, bool) {
    // input.end
    // seeds.end()
    (seeds.contains(input.start()), seeds.contains(input.end()))
}

fn find_unused_rec(
    to_search: RangeInclusive<i64>,
    mut ranges: &[MapEntry],
) -> (
    Option<RangeInclusive<i64>>,
    Option<(&[MapEntry], RangeInclusive<i64>)>,
) {
    let mut start_point = *to_search.start();
    let end = *to_search.end();
    loop {
        if ranges.is_empty() {
            return (Some(start_point..=end), None);
        }
        if start_point == *ranges[0].input_range.start() {
            if end == *ranges[0].input_range.end() {
                return (None, None);
            }
            start_point = *ranges[0].input_range.end() + 1;
            ranges = &ranges[1..];
        } else {
            return (
                Some(start_point..=*ranges[0].input_range.start() - 1),
                Some((&ranges[1..], *ranges[0].input_range.start()..=end)),
            );
        }
    }
}

fn find_unused(
    mut to_search: RangeInclusive<i64>,
    mut ranges: &[MapEntry],
) -> Vec<RangeInclusive<i64>> {
    let mut unused = Vec::new();
    loop {
        let (elem, new_ranges) = find_unused_rec(to_search, ranges);
        if let Some(elem) = elem {
            unused.push(elem);
        }
        match new_ranges {
            Some((new_ranges, new_to_search)) => {
                to_search = new_to_search;
                ranges = new_ranges;
            }
            None => break,
        }
    }
    unused
}

fn part2() {
    let mut categories = INPUT.split("\n\n");
    let seeds = categories
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|seed| seed.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let seeds_ranges = seeds.chunks_exact(2).map(|chunk| {
        let &[start, n] = chunk else { panic!() };
        vec![start..=start + n - 1]
    });

    let maps = categories
        .map(|category| {
            let mut lines = category.lines();
            lines.next();
            let map = lines
                .map(|line| {
                    let mut elems = line.split(" ").map(|num| num.parse::<i64>().unwrap());
                    let dest_start = elems.next().unwrap();
                    let source_start = elems.next().unwrap();
                    let n = elems.next().unwrap();
                    MapEntry {
                        input_range: source_start..=source_start + n - 1,
                        output_delta: dest_start - source_start,
                    }
                })
                .collect::<Vec<_>>();
            map
        })
        .collect::<Vec<_>>();

    let mut i = 0;

    let out = seeds_ranges
        .map(|mut input| {
            for map in maps.iter()
            {
                let mut new_input = Vec::new();
                for range in input.iter() {
                    let mut used = Vec::new();
                    for entry in map {
                        match range_contains_range(&range, &entry.input_range) {
                            (true, true) => {
                                used.push(
                                    entry.clone(),
                                );
                            }
                            (true, false) => {
                                used.push(
                                    MapEntry {
                                        input_range: *entry.input_range.start()..=*range.end(),
                                        output_delta: entry.output_delta,
                                    },
                                );
                            }
                            (false, true) => {
                                used.push(
                                    MapEntry {
                                        input_range: *range.start()..=*entry.input_range.end(),
                                        output_delta: entry.output_delta,
                                    }
                                );
                            }
                            (false, false) => {
                                if range.start() > entry.input_range.start() && range.end() < entry.input_range.end(){
                                    used.push(
                                        MapEntry {
                                            input_range: *range.start()..=*range.end(),
                                            output_delta: entry.output_delta,
                                        },
                                    );
                                }
                            }
                        }
                    }
                    used.sort_by_key(|range| *range.input_range.start());
                    let unused = find_unused(range.clone(), &used);
                    new_input.extend(unused);
                    new_input.extend(used.into_iter().map(|entry| entry.apply()));
                }
                input = new_input;
                i += 1;
            }

            input.into_iter().map(|range| *range.start()).min().unwrap()
        })
        .min().unwrap();

    dbg!(out);
}

fn main() {
    Lazy::force(&INPUT);
    let now = Instant::now();
    part1();
    dbg!(now.elapsed());
    let now = Instant::now();
    part2();
    dbg!(now.elapsed());
}
