use std::fs;

fn get_input() -> String {
    // --snip--
    println!("In file {}", "inputs/day1/p1");

    let contents =
        fs::read_to_string("inputs/day1/p1").expect("Something went wrong reading the file");

    return contents;
}

fn parse_depths(inputs: String) -> Vec<i32> {
    let depths = inputs
        .lines()
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .collect();
    return depths;
}

fn p1(depths: &Vec<i32>) -> u32 {
    let mut count = 0u32;

    for n in 1..depths.len() {
        if depths[n] > depths[n - 1] {
            count += 1;
        }
    }

    count
}

fn p2(depths: &Vec<i32>) -> u32 {
    let mut count = 0u32;

    for n in 3..depths.len() {
        if depths[n] + depths[n - 1] + depths[n - 2] > depths[n - 1] + depths[n - 2] + depths[n - 3]
        {
            count += 1;
        }
    }

    count
}

pub fn init() {
    println!("Day 1 -----------------");
    let input_str = get_input();
    let depths = parse_depths(input_str);
    println!("Found {} depths", depths.len());
    let p1_result = p1(&depths);
    println!("P1: Found {} increased", p1_result);
    let p2_result = p2(&depths);
    println!("P2: Found {} increased", p2_result);
}
