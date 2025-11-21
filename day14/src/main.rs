use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use utils::read_lines;

struct SymbolGrid {
    active: HashSet<usize>,
    inactive: HashSet<usize>,
    width: usize,
    height: usize,
    diagonals: HashMap<usize, Vec<usize>>,
}

impl SymbolGrid {
    fn new_inactive(width: usize, height: usize) -> SymbolGrid {
        let active = HashSet::new();
        let inactive = (0..width * height).collect::<HashSet<_>>();
        let diagonals = SymbolGrid::get_diagonals(width, height);

        SymbolGrid {active, inactive, width, height, diagonals}
    }

    fn from_text(filename: &str) -> SymbolGrid {
        let lines = read_lines(filename);
        let width = lines[0].len();
        let height = lines.len();
        let mut active = HashSet::new();
        let mut inactive = HashSet::new();

        lines.iter()
            .enumerate()
            .for_each(|(row, line)| {
                line.chars()
                    .enumerate()
                    .for_each(|(col, c)| {
                        if c == '#' {
                            active.insert(row * width + col);
                        }
                        else {
                            inactive.insert(row * width + col);
                        }
                    });
            });

        let diagonals = SymbolGrid::get_diagonals(width, height);
        SymbolGrid { active, inactive, width, height, diagonals }
    }

    fn add_round(&mut self) -> usize {
        let mut new_active = HashSet::new();
        let mut new_inactive = HashSet::new();

        let stays_active = self.active.par_iter()
            .filter(|idx| {
                let diagonals = &self.diagonals[idx];
                let active_diagonals = diagonals.iter()
                    .filter(|diag| self.active.contains(diag))
                    .count();

                active_diagonals % 2 != 0
            })
            .cloned()
            .collect::<HashSet<usize>>();

        let becomes_inactive = self.active.difference(&stays_active)
            .cloned()
            .collect::<HashSet<usize>>();

        let becomes_active = self.inactive.par_iter()
            .filter(|idx| {
                let diagonals = &self.diagonals[idx];
                let active_diagonals = diagonals.iter()
                    .filter(|diag| self.active.contains(diag))
                    .count();

                active_diagonals % 2 == 0
            })
            .cloned()
            .collect::<HashSet<usize>>();

        let stays_inactive = self.inactive.difference(&becomes_active)
            .cloned()
            .collect::<HashSet<usize>>();

        new_active.extend(stays_active);
        new_active.extend(becomes_active);
        new_inactive.extend(stays_inactive);
        new_inactive.extend(becomes_inactive);


        self.active = new_active;
        self.inactive = new_inactive;

        self.active.len()
    }

    fn get_frequency_and_pattern_repeats(&mut self, active_pattern: HashSet<usize>, inactive_pattern: HashSet<usize>) -> (usize, Vec<(usize, usize)>) {
        // Loops over rounds until it finds the pattern where all tiles are active (after the first round).
        // Returns the frequency and the indices of the rounds where pattern shows up.
        let mut pattern_indices = vec![];
        let mut end_index = 0;
        let mut index = 0;

        loop {
            self.add_round();
            index += 1;

            if self.active.len() == 1156 && index > 1 {
                end_index = index - 1;
            }

            if self.active.is_superset(&active_pattern) && self.inactive.is_superset(&inactive_pattern) {
                pattern_indices.push((index, self.active.len()));
            }

            if end_index > 0 && !pattern_indices.is_empty() {
                break;
            }
        }

        (end_index, pattern_indices)
    }

    fn get_diagonals(width: usize, height: usize) -> HashMap<usize, Vec<usize>> {
        let mut diagonals:  HashMap<usize, Vec<usize>> = HashMap::new();

        (0..height).for_each(|row| {
            let row = row as isize;
            (0..width).for_each(|col| {
                let col = col as isize;
                let neighbors = [(row + 1, col + 1), (row - 1, col + 1),
                                         (row + 1, col - 1), (row - 1, col - 1)];
                let neighbors: Vec<usize> = neighbors.iter()
                    .filter(|point| SymbolGrid::is_valid_point(point, width as isize, height as isize))
                    .map(|(r, c)| *r as usize * width + *c as usize)
                    .collect();
                let index = row as usize * width + col as usize;
                diagonals.insert(index, neighbors.clone());
            })
        });
        diagonals
    }

    fn is_valid_point(point: &(isize, isize), width: isize, height: isize) -> bool {
        let row = point.0;
        let col = point.1;

        row >= 0 && row < height && col >= 0 && col < width
    }
}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let mut grid = SymbolGrid::from_text("inputs/day14pt1.txt");
    let mut actives = 0;
    for _ in 0..10 {
        actives += grid.add_round();
    }
    println!("Part 1: {:?}", actives);
}

fn part_2() {
    let mut grid = SymbolGrid::from_text("inputs/day14pt2.txt");
    let mut actives = 0;
    for _ in 0..2025 {
        actives += grid.add_round();
    }
    println!("Part 2: {:?}", actives);
}

fn part_3() {
    let mut grid = SymbolGrid::new_inactive(34, 34);
    let center_grid = SymbolGrid::from_text("inputs/day14pt3.txt");

    let active_pattern: HashSet<usize> = center_grid.active.iter()
        .map(|&idx| map_to_larger_grid(idx, &grid, &center_grid))
        .collect();

    let inactive_pattern : HashSet<usize> = center_grid.inactive.iter()
        .map(|&idx| map_to_larger_grid(idx, &grid, &center_grid))
        .collect();

    let (freq, repeat_indices) = grid.get_frequency_and_pattern_repeats(active_pattern, inactive_pattern);

    let full_rounds = 1000000000 / freq;
    let partial = 1000000000 % freq;

    let mut total = full_rounds * repeat_indices.iter()
        .map(|(_, active)| active)
        .sum::<usize>();

    total += repeat_indices.iter()
        .filter(|(idx, _)| idx <= &partial)
        .map(|(_, active)| active)
        .sum::<usize>();

    println!("Part 3: {:?}", total);

}

fn map_to_larger_grid(idx: usize, grid: &SymbolGrid, center_grid: &SymbolGrid) -> usize {
    let row_offset = (grid.height - center_grid.height) / 2;
    let col_offset = (grid.width - center_grid.width) / 2;
    let original_row = idx / center_grid.width;
    let original_col = idx % center_grid.width;
    (original_row + row_offset) * grid.width + original_col + col_offset
}
