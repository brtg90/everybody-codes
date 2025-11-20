use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let clock = parse("inputs/day13pt1.txt");
    let number = clock[2025 % clock.len()];
    println!("Part 1: {:?}", number);
}

fn part_2() {
    let clock = parse_ranges("inputs/day13pt2.txt");
    let number = clock[20252025 % clock.len()];
    println!("Part 2: {:?}", number);
}

fn part_3() {
    let clock = parse_ranges("inputs/day13pt3.txt");
    let number = clock[202520252025 % clock.len()];
    println!("Part 3: {:?}", number);
}

fn parse(filename: &str) -> Vec<usize> {
    let numbers: Vec<usize> = read_lines(filename)
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut total = vec![1];

    let mut first: Vec<usize> = numbers.iter()
        .step_by(2)
        .copied()
        .collect();

    let mut second: Vec<usize> = numbers.into_iter()
        .skip(1)
        .step_by(2)
        .collect();

    second.reverse();
    total.append(&mut first);
    total.append(&mut second);
    total
}

fn parse_ranges(filename: &str) -> Vec<usize> {
    let mut first: Vec<usize> = Vec::new();
    let mut second: Vec<usize> = Vec::new();

    read_lines(filename)
        .iter()
        .enumerate()
        .for_each(|(i, line)| {
            let extremes = line.split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut vec = (extremes[0]..extremes[1] + 1).collect::<Vec<usize>>();
            if i % 2 == 0 {
                first.append(&mut vec);
            }
            else {
                second.append(&mut vec);
            }
        });

    let mut total = vec![1];


    second.reverse();
    total.append(&mut first);
    total.append(&mut second);
    total
}