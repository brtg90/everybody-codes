use std::collections::HashMap;

use utils::read_input;

#[derive(Debug, Clone)]
struct Plant {
    id: usize,
    thickness: isize,
    energy: isize,
    parents: Vec<usize>,
}

impl Plant {
    fn from_description(description: &str, plants: &HashMap<usize, Plant>, instruction_map: Option<&HashMap<usize, bool>>) -> Plant {
        let mut lines = description.lines();
        let first_line = lines.next().unwrap();
        let numbers = first_line[6..first_line.len() - 1]
            .split("with thickness")
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let id = numbers[0] as usize;
        let thickness = numbers[1];

        let mut energy = 0;
        let mut parents = Vec::new();

        for line in lines {
            if line.contains("free") {
                if let Some(instruction) = instruction_map
                    && instruction.get(&id) == Some(&false) {
                        continue;
                }
                let branch_thickness = line[29..].parse::<isize>().unwrap();
                energy += branch_thickness;
            }
            else {
                let numbers = line[18..].split("with thickness ")
                    .map(|x| x.trim().parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                let parent = numbers[0] as usize;
                parents.push(parent);
                let parent_thickness = numbers[1];

                energy += plants.get(&parent).unwrap().energy * parent_thickness;
            }
        }

        if energy < thickness {
            energy = 0;
        }

        Plant { id, thickness, energy, parents }

    }
}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day18pt1.txt");
    let mut plants: HashMap<usize, Plant> = HashMap::new();
    input.iter()
        .for_each(|descr| {
            let plant = Plant::from_description(descr, &plants, None);
            plants.insert(plant.id, plant);
        });
    let total = plants.iter()
        .max_by_key(|(k, _)| *k)
        .map(|(_, v)| v.energy)
        .unwrap();

    println!("Part 1: {:?}", total);
}

fn part_2() {
    let input = parse("inputs/day18pt2.txt");
    let mut instructions = parse_instructions(&input);
    let mut total = 0;

    while let Some(instruction) = instructions.pop() {
        total += try_instructions(&instruction, &input);
    }

    println!("Part 2: {:?}", total);
}

fn part_3() {
    let input = parse("inputs/day18pt3.txt");
    println!("Part 3: {:?}", 0);
}

fn try_instructions(instruction: &str, input: &[String]) -> isize {
        let instruction = get_instruction(instruction);
        let mut plants: HashMap<usize, Plant> = HashMap::new();
        input.iter()
            .filter(|line| line.contains("Plant"))
            .for_each(|descr| {
                let plant = Plant::from_description(descr, &plants, Some(&instruction));
                plants.insert(plant.id, plant);
            });
        plants.iter()
            .max_by_key(|(k, _)| *k)
            .map(|(_, v)| v.energy)
            .unwrap()
}

fn get_instruction(instr: &str) -> HashMap<usize, bool> {
    let mut instructions: HashMap<usize, bool> = HashMap::new();
    instr.split_whitespace()
        .enumerate()
        .for_each(|(i, str)| {
            let active = str.parse::<usize>().unwrap() == 1;
            instructions.insert(i + 1, active);
        });
    instructions
}

fn parse_instructions(input: &[String]) -> Vec<String> {
    input.iter()
        .filter(|line| !line.contains("Plant"))
        .flat_map(|line| line.split("\r\n"))
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

fn parse(filename: &str) -> Vec<String> {
    read_input(filename).split("\r\n\r\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}