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

fn identify_number(s: &str, ) -> Option<usize>{
    if s.starts_with("one") || s.starts_with("1"){
        return Some(1)
    }
    if s.starts_with("two") || s.starts_with("2"){
        return Some(2)
    }
    if s.starts_with("three") || s.starts_with("3"){
        return Some(3)
    }
    if s.starts_with("four") || s.starts_with("4"){
        return Some(4)
    }
    if s.starts_with("five") || s.starts_with("5"){
        return Some(5)
    }
    if s.starts_with("six") || s.starts_with("6"){
        return Some(6)
    }
    if s.starts_with("seven") || s.starts_with("7"){
        return Some(7)
    }
    if s.starts_with("eight") || s.starts_with("8"){
        return Some(8)
    }
    if s.starts_with("nine") || s.starts_with("9"){
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
