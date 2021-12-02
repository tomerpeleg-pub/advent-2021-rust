use std::fs;

struct Position {
    x: i32,
    y: i32,
}

fn get_input() -> String {
    println!("In file {}", "inputs/day2/input");

    let contents =
        fs::read_to_string("inputs/day2/input").expect("Something went wrong reading the file");

    contents
}

fn parse_instruction(instruction: String) -> Position {
    let mut parts = instruction.split_whitespace();
    let direction = parts.next();
    let distance = match parts.next() {
        Some(x) => x.parse::<i32>().unwrap(),
        None => 0,
    };

    match direction {
        Some("down") => Position { x: 0, y: distance },
        Some("up") => Position { x: 0, y: -distance },
        Some("forward") => Position { x: distance, y: 0 },
        Some(_) => Position { x: 0, y: 0 },
        None => Position { x: 0, y: 0 },
    }
}

fn p1(directions: &Vec<Position>) -> Position {
    let mut position = Position { x: 0, y: 0 };

    for dir in directions {
        position.x += dir.x;
        position.y += dir.y;
    }

    position
}

fn p2(directions: &Vec<Position>) -> Position {
    let mut position = Position { x: 0, y: 0 };
    let mut aim = 0;

    for dir in directions {
        aim += dir.y;
        position.x += dir.x;
        position.y += aim * dir.x;
    }

    position
}

pub fn init() {
    println!("Day 2 -----------------");
    let input: Vec<Position> = get_input()
        .lines()
        .map(|v| parse_instruction(v.to_string()))
        .collect();
    println!("Found {} instructions", input.len());
    let p1_result = p1(&input);
    let p1_total = p1_result.x * p1_result.y;
    println!(
        "P1: x: {}, y: {}, tot: {}",
        p1_result.x, p1_result.y, p1_total
    );
    let p2_result = p2(&input);
    let p2_total = p2_result.x * p2_result.y;
    println!(
        "P2: x: {}, y: {}, tot: {}",
        p2_result.x, p2_result.y, p2_total
    );
}
