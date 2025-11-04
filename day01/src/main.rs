use utils::read_lines;


fn main() {
    let lines = read_lines("inputs/day01pt3.txt");
    let name = parse_instructions(&lines, true);
    let parent1 = parse_instructions(&lines, false);
    let parent2 = swap_names(&lines);

    println!("Final name: {:?}", name);
    println!("First parent name: {:?}", parent1);
    println!("Second parent name: {:?}", parent2);
}

fn parse_instructions(lines: &[String], clamp: bool) -> String {
    let instructions = &lines[1];
    let names = lines[0].split(",").collect::<Vec<&str>>();
    let max_length = names.len() as isize - 1;

    let mut final_pos = 0;
    instructions.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|x| {
            final_pos = calculate_position(x, final_pos, max_length, clamp)
        });

    if !clamp {
        if final_pos < 0 {
            final_pos += max_length;
        }
        final_pos %= max_length + 1;
    }
    names[final_pos as usize].to_owned()
}

fn calculate_position(instruction: &str, mut final_pos: isize, max_length: isize, clamp: bool) -> isize {
    let mut sign = 1;
    let direction = instruction.chars().nth(0);
    if direction == Some('L') {
        sign = -1;
    }
    else if direction != Some('R') {
        panic!("Unexpected direction: {:?}", direction);
    }

    let value = instruction[1..].parse::<isize>().unwrap();
    if clamp {
        final_pos = (final_pos + value * sign).clamp(0, max_length);
    }
    else {
        final_pos += value * sign;
    }
    final_pos
}

fn swap_names(lines: &[String]) -> String {
    let instructions = &lines[1];
    let mut names = lines[0].split(",").collect::<Vec<&str>>();
    // Here max_length is defined as +1 compared to the one above, since we do not have to subtract
    // one anywhere
    let max_length = names.len() as isize;

    instructions.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|x| {
            let mut sign = 1;
            let direction = x.chars().nth(0);
            if direction == Some('L') {
                sign = -1;
            }
            else if direction != Some('R') {
                panic!("Unexpected direction: {:?}", direction);
            }

            let value = x[1..].parse::<isize>().unwrap();
            let step = value % max_length;

            let pos: isize = if sign == - 1 {
                (max_length - step) % max_length
            }
            else {
                step
            };

            names.swap(pos as usize, 0);
        });

    names[0].to_owned()
}