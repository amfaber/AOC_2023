use shared::{get_day_input, lazy_input, Lazy};
lazy_input!{1}

fn part1(){
    let mut total = 0;
    for line in INPUT.lines(){
        let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
        total += format!("{first}{last}").parse::<usize>().unwrap();
    }
    dbg!(total);
}

fn starts_with_alias(s: &str, pats: &[&str]) -> bool{
    pats.iter().fold(false, |acc, pat| acc || s.starts_with(pat))
}

fn identify_number(s: &str) -> Option<usize>{
    if starts_with_alias(s, &["one", "1"]){
        return Some(1)
    }
    if starts_with_alias(s, &["two", "2"]){
        return Some(2)
    }
    if starts_with_alias(s, &["three", "3"]){
        return Some(3)
    }
    if starts_with_alias(s, &["four", "4"]){
        return Some(4)
    }
    if starts_with_alias(s, &["five", "5"]){
        return Some(5)
    }
    if starts_with_alias(s, &["six", "6"]){
        return Some(6)
    }
    if starts_with_alias(s, &["seven", "7"]){
        return Some(7)
    }
    if starts_with_alias(s, &["eight", "8"]){
        return Some(8)
    }
    if starts_with_alias(s, &["nine", "9"]){
        return Some(9)
    }
    None
}

fn part2(){
    let mut total = 0;
    for mut line in INPUT.lines(){
        let mut numbers = Vec::new();
        while !line.is_empty(){
            if let Some(number) = identify_number(line){
                numbers.push(number);
            }
            line = &line[1..];
        }
        let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()).parse::<usize>().unwrap();
        total += number;
    }
    dbg!(total);
}

fn main() {
    part1();
    part2();
}
