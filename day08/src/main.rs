use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use shared::{get_day_input, lazy_input, Lazy};

lazy_input! {8}

struct Edges<'a> {
    left: &'a str,
    right: &'a str,
    // full_rotation: Option<&'a str>,
}

impl<'a> Edges<'a> {
    fn parse(s: &'a str) -> (&'a str, Self) {
        let (name, lr) = s.split_once(" = ").unwrap();
        let (mut left, mut right) = lr.split_once(", ").unwrap();
        left = &left[1..];
        right = &right[..right.len() - 1];
        (
            name,
            Self {
                left,
                right,
                // full_rotation: None,
            },
        )
    }
}

const _TEST_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

fn part1() {
    // let mut lines = _TEST_INPUT.lines();
    let mut lines = INPUT.lines();
    let mut instructions = lines.next().unwrap().chars().cycle().enumerate();
    let _ = lines.next();

    let map = lines.map(Edges::parse).collect::<HashMap<_, _>>();
    let mut current = "AAA";
    while current != "ZZZ" {
        let (_, instruction) = instructions.next().unwrap();
        match instruction {
            'L' => current = map[current].left,
            'R' => current = map[current].right,
            _ => unreachable!(),
        }
    }
    let (i, _) = instructions.next().unwrap();
    dbg!(map.len());
    dbg!(i);
}

fn part2_redo() {
    let mut lines = INPUT.lines();
    let instructions = lines.next().unwrap();
    let instruction_len = instructions.len();
    let mut instructions = instructions.chars().cycle().enumerate();
    let _ = lines.next();

    let mut currents = Vec::new();
    let map = lines
        .map(|line| {
            let (name, edges) = Edges::parse(line);
            if name.ends_with('A') {
                currents.push(name);
            }
            (name, edges)
        })
        .collect::<HashMap<_, _>>();
    let mut solutions = Vec::new();
    for mut current in currents {
        while !current.ends_with('Z') {
            let (_, instruction) = instructions.next().unwrap();
            match instruction {
                'L' => current = map[current].left,
                'R' => current = map[current].right,
                _ => unreachable!(),
            }
        }
        let (i, _) = instructions.next().unwrap();
        solutions.push(i);
    }
    // dbg!(least_common_multiple(&solutions));
    dbg!(lcm(&solutions));
    dbg!(solutions.iter().map(|elem| *elem as u128).product::<u128>() / lcm(&solutions) as u128);
}

fn least_common_multiple(seq: &[usize]) -> u128{
    let seq = seq.iter().map(|elem| *elem as u128).collect::<Vec<_>>();
    let mut working_seq = seq.clone();
    loop{
        if working_seq.iter().fold(true, |acc, elem| {
            acc && working_seq[0] == *elem
        }){
            break working_seq[0]
        }
        let index_of_min = working_seq
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;
        working_seq[index_of_min] += seq[index_of_min];
        dbg!(&working_seq);
    }
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    (a * b) / gcd_of_two_numbers(a, b)
    // a * (b / gcd_of_two_numbers(a, b))
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
// fn part2() {
//     let mut lines = INPUT.lines();
//     let instructions = lines.next().unwrap();
//     let _ = lines.next();

//     let mut map = lines.map(Edges::parse).collect::<HashMap<_, _>>();
//     let illegal_map = unsafe {
//         std::mem::transmute::<&mut HashMap<&str, Edges>, &mut HashMap<&str, Edges>>(&mut map)
//     };
//     let mut currents = Vec::new();
//     for (&(mut current), edge) in illegal_map {
//         if current.ends_with('A') {
//             currents.push(current);
//         }
//         for instruction in instructions.chars() {
//             match instruction {
//                 'L' => current = map[current].left,
//                 'R' => current = map[current].right,
//                 _ => unreachable!(),
//             }
//         }
//         edge.full_rotation = Some(current);
//     }

//     let mut rotations = 1;
//     dbg!(currents.len());
//     let mut seen = HashSet::new();
//     loop{
//         seen.insert(currents.clone());
//         let mut finished = true;
//         for current in &mut currents{
//             *current = map[current].full_rotation.unwrap();
//             finished &= current.ends_with('Z');
//             if current.ends_with('Z'){
//                 // n_finished += 1;
//             }
//         }

//         dbg!(&seen.len());
//         if finished{
//             break
//         }
//         // dbg!(rotations);
//         rotations += 1;
//     }
//     dbg!(rotations * instructions.len());
// }

fn main() {
    let now = Instant::now();
    part1();
    dbg!(now.elapsed());

    let now = Instant::now();
    part2_redo();
    dbg!(now.elapsed());
}
