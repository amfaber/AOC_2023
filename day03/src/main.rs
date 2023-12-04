use std::ops::{Range, RangeInclusive};

use nom::character::complete::digit1;
use shared::{get_day_input, lazy_input, Lazy};
lazy_input! {3}

#[derive(Clone, Debug)]
struct Number {
    value: u32,
    span: RangeInclusive<usize>,
}

struct Symbol {
    position: usize,
    ty: SymbolType,
}

enum SymbolType {
    Gear(Vec<Number>),
    Other,
}

struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

// struct RollingLines<const N: usize>([Option<Line>; N]);

// impl<const N: usize> Default for RollingLines<N>{
//     fn default() -> Self {
//         Self([(); N].map(|_| None))
//     }
// }

enum ParseResult {
    Number(Number),
    Symbol(Symbol),
    None,
    EOL,
}

fn parse_symbol_or_number(mut s: &str, offset: usize) -> (&str, ParseResult) {
    if s.is_empty() {
        return (s, ParseResult::EOL);
    }
    if s.starts_with(".") {
        return (&s[1..], ParseResult::None);
    }

    match digit1::<_, nom::error::Error<_>>(s) {
        Ok((new_s, digit_str)) => {
            s = new_s;
            let value = digit_str.parse::<u32>().unwrap();
            let number = Number {
                value,
                span: offset.checked_sub(1).unwrap_or(0)..=offset + digit_str.len(),
            };
            return (s, ParseResult::Number(number));
        }
        Err(_) => {}
    }
    let ty = match s.starts_with("*") {
        true => SymbolType::Gear(Vec::new()),
        false => SymbolType::Other,
    };

    return (
        &s[1..],
        ParseResult::Symbol(Symbol {
            position: offset,
            ty,
        }),
    );
}

fn parse_line(mut line: &str, only_gears: bool) -> Line {
    let line_len = line.len();
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    loop {
        let offset = line_len - line.len();
        let (rest_of_line, result) = parse_symbol_or_number(line, offset);
        line = rest_of_line;
        match result {
            ParseResult::Number(num) => numbers.push(num),
            ParseResult::Symbol(sym) => {
                if only_gears {
                    match sym.ty {
                        SymbolType::Gear(_) => symbols.push(sym),
                        SymbolType::Other => {}
                    }
                } else {
                    symbols.push(sym)
                }
            }
            ParseResult::None => {}
            ParseResult::EOL => break,
        }
    }
    Line { numbers, symbols }
}

const _TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn part1() {
    let mut current_lines = [(); 3].map(|_| None);
    let mut lines = INPUT.lines();
    let mut idx = 0usize;
    let mut sum = 0;
    loop {
        current_lines[idx % 3] = lines.next().map(|line| parse_line(line, false));
        if let Some(prev) = idx.checked_sub(1) {
            let check_numbers = match &current_lines[prev % 3] {
                Some(line) => &line.numbers,
                None => break,
            };
            let valid_numbers = check_numbers.iter().filter_map(|number| {
                current_lines
                    .iter()
                    .flatten()
                    .any(|line| {
                        line.symbols
                            .iter()
                            .any(|symbol| number.span.contains(&symbol.position))
                    })
                    .then_some(number.value)
            });
            sum += valid_numbers.sum::<u32>()
        }
        idx += 1;
    }
    dbg!(sum);
}

fn part2() {
    let mut current_lines = [(); 3].map(|_| None::<Line>);
    let mut lines = INPUT.lines();
    let mut idx = 0usize;
    let mut sum = 0;
    let mut first = true;
    loop {
        if let Some(line) = &current_lines[idx % 3] {
            line.symbols.iter().for_each(|symbol| match &symbol.ty {
                SymbolType::Gear(numbers) => if numbers.len() == 2 {
                    sum += numbers[0].value * numbers[1].value;
                },
                SymbolType::Other => unreachable!(),
            });
            first = false;
        } else {
            if !first{
                break
            }
        }
        current_lines[idx % 3] = lines.next().map(|line| parse_line(line, true));
        if let Some(prev) = idx.checked_sub(1) {
            let check_numbers = match &current_lines[prev % 3] {
                Some(line) => &line.numbers,
                None => continue,
            };
            let check_numbers =
                unsafe { std::mem::transmute::<&Vec<Number>, &Vec<Number>>(check_numbers) };
            for number in check_numbers {
                current_lines
                    .iter_mut()
                    .flatten()
                    .flat_map(|line| &mut line.symbols)
                    .for_each(|symbol| match &mut symbol.ty {
                        SymbolType::Gear(vec) => {
                            if number.span.contains(&symbol.position) {
                                vec.push(number.clone());
                            }
                        }
                        SymbolType::Other => {
                            unreachable!()
                        }
                    });
            }
        }
        idx += 1;
    }
    dbg!(sum);
}

fn main() {
    part2();
}
