use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day19pt1.txt");
    println!("{:?}", input);
    let flaps = count_flaps_for_triplets(&input, 0, 0);
    println!("Part 1: {:?}", flaps);
}

fn part_2() {
    println!("Part 2: {:?}", 0);
}

fn part_3() {
    println!("Part 3: {:?}", 0);
}

fn count_flaps_for_triplets(triplets: &Vec<Vec<isize>>, x: isize, y: isize) -> isize {
    let mut flaps = 0;

    let options = take_obstacle(&triplets[0], x, y);
    if options.is_none() {
        return 0;
    }

    for option in options.unwrap() {
        if triplets.len() > 1 {
            flaps += option[2] + count_flaps_for_triplets(&triplets[1..].to_vec(), option[0], option[1]);

        }
        else {
            return flaps;
        }
    }
    flaps
}

fn take_obstacle(obstacle: &[isize], x: isize, y: isize) -> Option<Vec<Vec<isize>>> {
    let dx = obstacle[0] - x;
    let (dy_max, dy_min) = get_max_min(obstacle[1] - y, obstacle[1] - y + obstacle[2]);
    println!("dx {dx}, dy_max {dy_max}, dy_min {dy_min}");

    let possible: Vec<bool> = (dy_min..=dy_max)
        .map(|dy| (dy / dx).abs() <= 1)
        .collect();

    if !possible.contains(&true) {
        // Point out of reach
        return None;
    }

    let possible_end_points = possible.iter()
        .enumerate()
        .filter(|(_, possible)| **possible)
        .map(|(i, _)| vec![obstacle[0], dy_min + i as isize, std::cmp::max(0, dy_min + i as isize - y)])
        .collect();

    Some(possible_end_points)
}

fn get_max_min(a: isize, b: isize) -> (isize, isize) {
    (std::cmp::max(a, b), std::cmp::min(a, b))
}

fn parse(filename: &str) -> Vec<Vec<isize>> {
    read_lines(filename)
        .iter()
        .map(|line| line.split(",")
            .map(|x| x.parse::<isize>()
                .unwrap())
            .collect::<Vec<isize>>())
        .collect()
}