use std::collections::{HashMap, HashSet, VecDeque};

use utils::read_lines;

struct Grid {
    cells: Vec<usize>,
    volcano: usize,
    start: usize,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_file(filename: &str) -> Grid {
        let lines = read_lines(filename);
        let width = lines[0].len();
        let height = lines.len();

        let mut volcano = 0;
        let mut start = 0;
        let cells : Vec<usize> = lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                let mut row_vec = vec![];
                let chars = line.chars();
                for (col, c) in chars.enumerate() {
                    if c == '@' {
                        volcano = row * width + col;
                        row_vec.push(0);
                    }
                    else if c == 'S' {
                        start = row * width + col;
                        row_vec.push(0);
                    }
                    else {
                        row_vec.push(c.to_digit(10).unwrap() as usize);
                    }
                }
                row_vec
            })
            .collect();

        Grid { cells, volcano, start, width, height }
    }

    fn get_moves(&self, point: usize) -> Vec<usize> {
        let moves: [isize;4] = [-(self.width as isize), 1, self.width as isize, - 1];
        moves.into_iter()
            .map(|m| point as isize + m)
            .filter(|p| self.is_valid_point(*p))
            .map(|p| p as usize)
            .collect()

    }

    fn is_valid_point(&self, point: isize) -> bool {
        point >= 0 && point < self.cells.len() as isize
    }

    fn get_r_squared_from_volcano(&self, point: usize) -> usize {
        let p_row = (point / self.width) as isize;
        let p_col = (point % self.width) as isize;

        let v_row = (self.volcano / self.width) as isize;
        let v_col = (self.volcano % self.width) as isize;

        ((p_row - v_row).pow(2) + (p_col - v_col).pow(2)) as usize

    }

}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let grid = Grid::from_file("inputs/day17pt1.txt");
    let destruction = get_destruction_cells_bfs(&grid, 10);
    let sum = destruction.into_iter()
        .filter(|(k, _)| *k <= 10)
        .map(|(_, v)| v)
        .sum::<usize>();
    println!("Part 1: {:?}", sum);
}

fn part_2() {
    let grid = Grid::from_file("inputs/day17pt2.txt");
    let max_radius = grid.get_r_squared_from_volcano(0);

    let destruction = get_destruction_cells_bfs(&grid, max_radius);
    let max_score = destruction.iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, v)| k * v)
        .unwrap();
    println!("Part 2: {:?}", max_score);
}

fn part_3() {
    let grid = Grid::from_file("inputs/day17pt3test.txt");
    let max_radius = grid.get_r_squared_from_volcano(0);

    let destruction = get_destruction_cells_bfs(&grid, max_radius);
    println!("Part 3: {:?}", 0);
}

fn get_destruction_cells_bfs(grid: &Grid, radius: usize) -> HashMap<usize, usize> {
    let r_squared = radius * radius;

    let mut visited : HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(grid.volcano);

    let mut destruction: HashMap<usize, usize> = HashMap::new();

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let r_squared_curr = grid.get_r_squared_from_volcano(current);
        if r_squared_curr <= r_squared {
            // Calculate the radius, ceil is needed to ensure that it fits within an integer step
            let r = (r_squared_curr as f64).sqrt().ceil() as usize;
            destruction.entry(r).and_modify(|v| *v += grid.cells[current]).or_insert(grid.cells[current]);

        }
        grid.get_moves(current)
            .iter()
            .for_each(|&num| queue.push_back(num));

    }
    destruction
}
