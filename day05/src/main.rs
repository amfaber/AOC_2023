use std::ops::Range;

use shared::{get_day_input, lazy_input, Lazy};
lazy_input!{5}

#[derive(Debug)]
struct MapEntry{
    input_range: Range<i64>,
    output_delta: i64,
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

fn part1(){
    let mut categories = INPUT.split("\n\n");
    // let mut categories = _TEST_INPUT.split("\n\n");
    let seeds = categories.next().unwrap().split(" ").filter_map(|seed| seed.parse::<i64>().ok());

    let maps = categories.map(|category|{
        let mut lines = category.lines();
        lines.next();
        let map = lines.map(|line| {
            let mut elems = line.split(" ").map(|num| num.parse::<i64>().unwrap());
            let dest_start = elems.next().unwrap();
            let source_start = elems.next().unwrap();
            let n = elems.next().unwrap();
            MapEntry{
                input_range: source_start..source_start + n,
                output_delta: dest_start - source_start,
            }
        }).collect::<Vec<_>>();
        // map.sort_by_key(|entry| entry.input_range.start);
        // dbg!(&map);
        map
    }).collect::<Vec<_>>();

    let out = seeds.map(|mut input|{
        for map in &maps{
            for entry in map{
                if entry.input_range.contains(&input){
                    input += entry.output_delta;
                    break
                }
            }
            // dbg!(input);
            // dbg!(());
            // dbg!(());
        }
        // dbg!(input);
        input
    }).min();
    

    dbg!(out);
}

fn main() {
    part1();
}
