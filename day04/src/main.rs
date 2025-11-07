use utils::read_input;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let ratio = get_gear_ratio("inputs/day04pt1.txt");
    let turns = (2025.0 * ratio) as usize;
    println!("Part 1: {}", turns);
}

fn part_2() {
    let ratio = get_gear_ratio("inputs/day04pt2.txt");
    let turns_first_gear = (1e13 / ratio).ceil() as usize;
    println!("Part 2: {}", turns_first_gear);
}

fn part_3() {
    let ratio = get_gear_ratio("inputs/day04pt3.txt");
    let turns = (100.0 * ratio) as usize;
    println!("Part 3: {}", turns);
}

fn get_gear_ratio(filename: &str) -> f64 {
    // General function to get the total gear ratio of a setup, with or without common shafts
    let input = read_input(filename);
    let shaft_split: Vec<&str> = input.split('|').collect();
    let mut ratio = 1.0;
    shaft_split
        .iter()
        .for_each(|s| {
            let split = s.split_whitespace().collect::<Vec<&str>>();
            let first = split[0].parse::<f64>().expect("Couldn't parse ratio");
            let last = split[split.len() - 1].parse::<f64>().expect("Couldn't parse ratio");
            ratio *= first / last;
        });
   ratio
}