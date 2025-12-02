use std::collections::HashMap;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let input = parse("inputs/day19pt1.txt");
    let flaps = bfs(&input);
    println!("Part 1: {:?}", flaps);
}

fn part_2() {
    let input = parse("inputs/day19pt2.txt");
    let flaps = bfs(&input);
    println!("Part 2: {:?}", flaps);
}

fn part_3() {
    let input = parse("inputs/day19pt3.txt");
    let flaps = bfs(&input);
    println!("Part 3: {:?}", flaps);
}

fn bfs(triplets: &HashMap<isize, Vec<[isize; 3]>>) -> isize {
    let mut x_points = triplets.keys().cloned().collect::<Vec<_>>();
    x_points.sort();
    x_points.insert(0, 0);

    let mut current: HashMap<isize, isize> = HashMap::new();
    current.insert(0, 0);

    for (i, x) in x_points.iter().enumerate() {
        if i == x_points.len() - 1 {
            break;
        }
        let mut new: HashMap<isize, isize> = HashMap::new();

        for (&curr_y, &curr_flaps) in &current {
            for opening in &triplets[&x_points[i + 1]] {
                let dx = opening[0] - x;
                // Each step in x-direction is either a flap or not (i.e. difference of two if one
                // is swapped for the other)
                let y_max = dx + curr_y;
                let y_min = -dx + curr_y;
                let parity = y_max % 2;
                for y_new in opening[1]..opening[1] + opening[2] {
                    if y_new >= y_min && y_new <= y_max && y_new % 2 == parity {
                        let new_flaps = ((y_new - curr_y) + dx) / 2;
                        new.entry(y_new)
                            .and_modify(|v| *v = (*v).min(curr_flaps + new_flaps))
                            .or_insert(curr_flaps + new_flaps);
                    }
                }
            }
        }
        current = new;

    }
    *current.values().min().unwrap()
}

fn parse(filename: &str) -> HashMap<isize, Vec<[isize; 3]>> {
    let mut obstacles = HashMap::new();
    read_lines(filename)
        .iter()
        .for_each(|line| {
            let array: [isize;3] = line.split(",")
                .map(|x| x.parse::<isize>()
                    .unwrap())
                .collect::<Vec<isize>>()
                .try_into()
                .unwrap();
            obstacles.entry(array[0])
                .and_modify(|v: &mut Vec<[isize;3]>| v.push(array))
                .or_insert(vec![array]);
        });
    obstacles
}