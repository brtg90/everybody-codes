use std::collections::HashMap;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let (char_map, names) = parse_input("inputs/day07pt1.txt");
    let name = &check_names(&char_map, &names)[0];
    println!("Part 1: {:?}", name);
}

fn part_2() {
    let (char_map, names) = parse_input("inputs/day07pt2.txt");
    let valid_names = check_names(&char_map, &names);
    let score = sum_indices_valid_names(&names, &valid_names);
    println!("Part 2: {:?}", score);
}

fn part_3() {
    // Old approach via HashSet worked but was very slow. Took some inspiration from
    // https://github.com/maneatingape/everybody-codes-rust/blob/main/src/event2025/quest07.rs
    // to optimize this.
    // First get the valid prefixes
    let (char_map, prefixes) = parse_input("inputs/day07pt3.txt");
    let valid_prefixes = check_names(&char_map, &prefixes);
    // Filter for longer prefixes that start with another one (they will give duplicate results),
    // and apply the recursive formula to the results
    let valid_names_count: usize = valid_prefixes.iter()
        .filter(|&prefix| {
            valid_prefixes.iter()
                .all(|other| prefix == other || !prefix.starts_with(other))
        })
        .map(|prefix| get_number_unique_names(&char_map, prefix.chars().last().unwrap(), prefix.len()))
        .sum();
    println!("Part 3: {:?}", valid_names_count);
}

fn parse_input(filename: &str) -> (HashMap<char, Vec<char>>, Vec<String>) {
    let mut char_map: HashMap<char, Vec<char>> = HashMap::new();
    let mut names: Vec<String> = vec![];

    read_lines(filename).into_iter()
        .for_each(|l| {
            let split_string = l.split(" > ").collect::<Vec<&str>>();
            if split_string.len() == 1 {
                names = split_string[0].split(",")
                    .map(|x| x.to_string())
                    .collect();
            }
            else {
                let key = split_string[0].chars().next().unwrap();
                let letters: Vec<char> = split_string[1].split(',')
                    .filter_map(|s| s.chars().next()) // Get first char
                    .collect();
                char_map.insert(key, letters);
            }
        });

    (char_map, names)
}

fn check_names(char_map: &HashMap<char, Vec<char>>, names: &[String]) -> Vec<String> {
    names.iter()
        .filter(|&name| is_valid(char_map, name))
        .map(|name| name.to_string())
        .collect()
}

fn is_valid(char_map: &HashMap<char, Vec<char>>, name: &str) -> bool {
    let name = name.chars().collect::<Vec<char>>();
    name.windows(2).all(|w| char_map.get(&w[0]).unwrap().contains(&w[1]))
}

fn sum_indices_valid_names(names: &[String], valid_names: &[String]) -> usize {
    valid_names.iter()
        .map(|name| names.iter().position(|x| x == name).unwrap() + 1)
        .sum()
}

fn get_number_unique_names(char_map: &HashMap<char, Vec<char>>, last: char, size: usize) -> usize {
    let mut total = 0;
        if size >= 7 {
            total += 1;
        }

    if size < 11 {
        total + char_map.get(&last).unwrap_or(&vec![]).iter()
            .map(|new| get_number_unique_names(char_map, *new, size + 1))
            .sum::<usize>()
    }
    else {
        total
    }
}