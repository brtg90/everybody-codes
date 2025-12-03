use std::collections::HashMap;

use utils::read_lines;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Dot,
    TUp,
    TDown,
    Start,
    End,
}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let grid = parse("inputs/day20pt1.txt");
    let pairs = get_number_connected_trampolines(&grid);
    println!("Part 1: {:?}", pairs);
}

fn part_2() {
    let grid = parse("inputs/day20pt2.txt");
    let connected = find_connected_trampolines(&grid);
    let start = grid.iter()
        .flatten()
        .position(|tile| *tile == Tile::Start)
        .unwrap();
    let end = grid.iter()
        .flatten()
        .position(|tile| *tile == Tile::End)
        .unwrap();
    let steps = count_steps_from_start(&connected, start, end);
    println!("Part 2: {:?}", steps);
}

fn part_3() {
    println!("Part 3: {:?}", 0);
}


fn count_steps_from_start(connected: &HashMap<usize, Vec<usize>>, start: usize, end: usize) -> usize {
    let mut counts : HashMap<usize, usize> = HashMap::new();
    counts.insert(start, 0);
    let mut steps = 0;
    let mut queue: Vec<usize> = vec![];
    queue.push(start);

    loop {
        let mut new = vec![];
        for current in &queue {
            // All trampolines in connected are connected, so unwrap is safe
            let options = connected.get(current).unwrap();
            for option in options {
                if option == &end {
                    return steps + 1;
                }
                if !counts.contains_key(option) {
                    counts.insert(*option, steps + 1);
                    new.push(*option);
                }

            }
        }
        queue = new;
        steps += 1;
    }

}

fn get_number_connected_trampolines(grid: &[Vec<Tile>]) -> usize {
    find_connected_trampolines(grid).values().flatten().collect::<Vec<&usize>>().len() / 2
}

fn find_connected_trampolines(grid: &[Vec<Tile>]) -> HashMap<usize, Vec<usize>> {
    let mut all = find_horizontally_connected_trampolines(grid);
    let vertical = find_vertically_connected_trampolines(grid);

    vertical.iter()
        .for_each(|(k, v)| {
            all.entry(*k)
                .and_modify(|hor_v| hor_v.extend(v))
                .or_insert(v.clone());
        });
    all
}

fn find_horizontally_connected_trampolines(grid: &[Vec<Tile>]) -> HashMap<usize, Vec<usize>> {
    let width = grid[0].len();
    let mut connected: HashMap<usize, Vec<usize>> = HashMap::new();

    grid.iter()
        .enumerate()
        .for_each(|(row_no, row)| {
            for i in 1..row.len() {
                if is_trampoline(&row[i]) && is_trampoline(&row[i - 1]) {
                    let current = row_no * width + i;
                    connected.entry(current)
                        .and_modify(|v| v.push(current - 1))
                        .or_insert(vec![current - 1]);
                    connected.entry(current - 1)
                        .and_modify(|v| v.push(current))
                        .or_insert(vec![current]);
                }
            }
        });
    connected
}

fn find_vertically_connected_trampolines(grid: &[Vec<Tile>]) -> HashMap<usize, Vec<usize>> {
    let width = grid[0].len();
    let mut connected: HashMap<usize, Vec<usize>> = HashMap::new();

    grid.iter()
        .enumerate()
        .skip(1)
        .for_each(|(row_no, row)| {
            for (i, col) in row.iter().enumerate() {
                if is_trampoline(col) && is_trampoline(&grid[row_no - 1][i]) {
                    let current = row_no * width + i;
                    let above = current - width;

                    let masked_curr = mask_start_end_tile(col.clone(), row_no, i);
                    let masked_above = mask_start_end_tile(grid[row_no - 1][i].clone(), row_no - 1, i);

                    if let (Tile::TDown, Tile::TUp) = (masked_curr, masked_above) {
                            connected.entry(current)
                                .and_modify(|v| v.push(above))
                                .or_insert(vec![above]);
                            connected.entry(above)
                                .and_modify(|v| v.push(current))
                                .or_insert(vec![current]);
                    };
                }
            }
        });
    connected
}

fn is_trampoline(tile: &Tile) -> bool {
    matches!(tile, Tile::Start | Tile::End | Tile::TUp | Tile::TDown)
}

fn mask_start_end_tile(tile: Tile, row: usize, col: usize) -> Tile {
    if tile == Tile::Start || tile == Tile::End {
        if (row + col) % 2 == 0 {
            Tile::TDown
        }
        else {
            Tile::TUp
        }
    }
    else {
        tile
    }
}

fn parse(filename: &str) -> Vec<Vec<Tile>> {
    read_lines(filename)
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Tile::Empty,
                    '.' => Tile::Dot,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => {
                        if (row + col) % 2 == 0 {
                            Tile::TDown
                        }
                        else {
                            Tile::TUp
                        }
                    }
                })
                .collect::<Vec<Tile>>()
        })
        .collect()
}