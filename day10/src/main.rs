use std::collections::{HashMap, HashSet};

use rayon::prelude::*;
use memoize::memoize;

use utils::read_lines;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct ChessBoard {
    sheep: Vec<(isize, isize)>,
    dragon: (isize, isize),
    safe: Vec<(isize, isize)>,
    width: isize,
    height: isize,
}

impl ChessBoard {
    fn from_text(filename: &str) -> ChessBoard {
        let lines = read_lines(filename);
        let height = lines.len() as isize;
        let width = lines[0].len() as isize;

        let mut sheep: Vec<(isize, isize)> = Vec::new();
        let mut dragon: (isize, isize) = (0, 0);
        let mut safe: Vec<(isize, isize)> = Vec::new();

        lines.iter()
            .enumerate()
            .for_each(|(row, column)| {
                column.chars().enumerate().for_each(|(col, c)| {
                    if c == 'S' {
                        sheep.push((row as isize, col as isize));
                    }
                    else if c == 'D' {
                        dragon = (row as isize, col as isize);
                    }
                    else if c == '#' {
                        safe.push((row as isize, col as isize));
                    }
                });
            });

        ChessBoard {
            sheep,
            dragon,
            safe,
            width,
            height,
        }
    }

    fn move_all_sheep(&mut self) {
        self.sheep = self.sheep.iter_mut()
            .map(|(row, col)| (*row + 1, *col))
            .filter(|pos| is_valid_move(pos, self.width, self.height))
            .collect();
    }

    fn move_one_sheep(&mut self, old_pos: (isize, isize)) {
        self.sheep = self.sheep.iter_mut()
            .map(|pos| if pos == &old_pos {(pos.0 + 1, pos.1)} else {*pos})
            .collect();
    }

    fn get_possible_sheep_moves(&self) -> Vec<((isize, isize),(isize, isize))> {
        let potential_moves: Vec<((isize, isize),(isize, isize))> = self.sheep.iter()
            .map(|(row, col)| ((*row, *col), (*row + 1, *col)))
            .filter(|(_, pos)| pos != &self.dragon || self.safe.contains(pos))
            .collect();
        potential_moves
    }

    fn sheep_escapes(&self, sheep_pos: &(isize, isize)) -> bool {
        sheep_pos.0 >= self.height
    }

    fn remove_sheep(&mut self, visited: &Vec<(isize, isize)>) -> usize {
        let before = self.sheep.len();
        self.sheep = self.sheep.iter()
            .filter(|&&pos| !visited.contains(&pos))
            .cloned()
            .collect();

        before - self.sheep.len()
    }

    fn remove_sheep_current_dragon(&mut self) {
        self.sheep = self.sheep.iter()
            .filter(|&&pos| self.dragon != pos)
            .cloned()
            .collect();
    }

    fn move_sheep_get_boards(&self) -> Vec<ChessBoard> {
        // Return a copy of the boards after each possible sheep move on the current board
        // Filter out the boards where the sheep escapes, since this is a failure of the dragon
        let sheep_moves = self.get_possible_sheep_moves();

        if sheep_moves.is_empty() {
            return vec![self.clone()];
        }

        sheep_moves.into_iter()
            .filter(|(_, new)| !self.sheep_escapes(&new))
            .map(|(old, _)| {
                let mut board = self.clone();
                board.move_one_sheep(old);
                board
            })
            .collect::<Vec<ChessBoard>>()
    }

    // fn move_dragon_get_boards(&self) -> Vec<ChessBoard> {
    //     // Return a copy of the boards after each possible dragon move on the current board
    //     let row = self.dragon.0;
    //     let col = self.dragon.1;
    //
    //     let mut new_points = Vec::from([(row + 2, col + 1), (row + 2, col - 1), (row + 1, col + 2), (row + 1, col - 2),
    //         (row - 1, col + 2), (row - 1, col - 2), (row - 2, col + 1), (row - 2, col - 1)]);
    //     new_points = new_points.into_iter()
    //         .filter(|&point| is_valid_move(&point, self.width, self.height))
    //         .collect::<Vec<(isize, isize)>>();
    //
    //     new_points.into_iter()
    //         .map(|new| {
    //             let mut board = self.clone();
    //             board.dragon = new;
    //             board
    //         })
    //         .collect::<Vec<ChessBoard>>()
    // }

    fn print_board(&self) {
        (0..self.height)
            .for_each(|row| {
                let column = (0..self.width)
                    .map(|col| {
                        let pos = (row, col);
                        if pos == self.dragon {
                            'D'
                        } else if self.sheep.contains(&pos) {
                            'S'
                        } else if self.safe.contains(&pos) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<Vec<char>>();
                println!("{}", column.iter().collect::<String>());
            });
        println!("\n\n");
    }
}

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let board = ChessBoard::from_text("inputs/day10pt1.txt");
    let visited = move_dragon_max_times(board.dragon, 0, 4, (board.width, board.height));
    let killed_sheep = get_killed_sheep(&visited, &board).len();
    println!("Part 1: {:?}", killed_sheep);
}

fn part_2() {
    let mut board = ChessBoard::from_text("inputs/day10pt2.txt");
    let mut total_sheep = 0;

    let mut dragon_pos: Vec<(isize, isize)> = vec![board.dragon];

    for _ in 0..20 {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        dragon_pos.iter()
            .map(|&pos| move_dragon_max_times(pos, 0, 1, (board.width, board.height)))
            .for_each(|set| visited.extend(&set));
        // First check which sheep are at the new dragon positions
        let killed_sheep = get_killed_sheep(&visited, &board);
        total_sheep += board.remove_sheep(&killed_sheep);

        // Check which sheep walk into dragon positions
       board.move_all_sheep();
        let killed_sheep = get_killed_sheep(&visited, &board);
        total_sheep += board.remove_sheep(&killed_sheep);

        dragon_pos = visited.into_iter().collect();
    }
    println!("Part 2: {:?}", total_sheep);
}

fn part_3() {
    let board = ChessBoard::from_text("inputs/day10pt3.txt");
    let mut memo = HashMap::new();
    let unique_sequences = dfs_memo(board, &mut memo);
    println!("Part 3: {}", unique_sequences);
}

// fn part_3() {
//     let mut board = ChessBoard::from_text("inputs/day10pt3.txt");
//
//     // let mut total_sequences = 0;
//
//     // let mut dragon_pos_stack: Vec<(isize, isize)> = vec![board.dragon];
//     // while !dragon_pos_stack.is_empty() {
//     //     let dragon_pos = dragon_pos_stack.pop().unwrap();
//     //
//     //     let sheep_moves = board.get_possible_sheep_moves(&dragon_pos);
//     //     for move_combination in sheep_moves {
//     //         let old = move_combination.0;
//     //         let sheep_pos = move_combination.1;
//     //
//     //         if board.sheep_escapes(&sheep_pos) {
//     //             continue;
//     //         }
//     //         let mut new_board = board.clone();
//     //         new_board.sheep = new_board.sheep.iter_mut()
//     //             .map(|pos| if *pos == old {
//     //                 sheep_pos.clone()
//     //             }
//     //             else {
//     //                 pos.clone()
//     //             })
//     //             .collect();
//     //
//     //
//     //     }
//     //
//     //     let mut visited: HashSet<(isize, isize)> = HashSet::new();
//
//
//     // }
//     let unique_sequences = dfs(board);
//     println!("Part 3: {:?}", unique_sequences);
// }

fn round_of_chess(board: &mut ChessBoard) -> isize {
    let mut unique_sequences = 0;

    let dragon = board.dragon;
    let sheep_moves = board.get_possible_sheep_moves();

    if board.sheep.is_empty() {
        return 1;
    }

    if sheep_moves.is_empty() {
        let dragon_moves = move_dragon_max_times(dragon, 0, 1, (board.width, board.height));
        // println!("No sheep move possible, dragon moves are: {:?}", dragon_moves);
        // for dragon_move in dragon_moves {
        //     let mut board_clone = board.clone();
        //     board_clone.dragon = dragon_move;
        //     // board_clone.print_board();
        //     // println!("Dragon move: {:?}", dragon_move);
        //     board_clone.remove_sheep(&vec![dragon_move]);
        //     unique_sequences += round_of_chess(&mut board_clone);
        // }
        let sum: isize = dragon_moves.par_iter()
            .map(|dragon_move| {
                let mut board_clone = board.clone();
                board_clone.dragon = dragon_move.to_owned();
                // board_clone.print_board();
                // println!("Dragon move: {:?}", dragon_move);
                board_clone.remove_sheep(&vec![dragon_move.to_owned()]);
                round_of_chess(&mut board_clone)
            })
            .sum();
        unique_sequences += sum;

    } else {
        unique_sequences += sheep_moves.par_iter()
            .map(|move_combination| {
            let mut board = board.clone();
            let old = move_combination.0;
            let sheep_pos = move_combination.1;

            if board.sheep_escapes(&sheep_pos) {
                // println!("Sheep escapes by {:?}", sheep_pos);
               return 0;
            }

            board.move_one_sheep(old);
            // println!("Sheep move: {:?}", sheep_pos);
            let dragon_moves = move_dragon_max_times(dragon, 0, 1, (board.width, board.height));
            // println!("Sheep move {:?}, dragon moves are: {:?}", move_combination, dragon_moves);
            // for dragon_move in dragon_moves {
            //     let mut board_clone = board.clone();
            //     board_clone.dragon = dragon_move;
            //     // board_clone.print_board();
            //     // println!("Dragon move: {:?}", dragon_move);
            //     board_clone.remove_sheep(&vec![dragon_move]);
            //     unique_sequences += round_of_chess(&mut board_clone);
            // }
            let sum: isize = dragon_moves.par_iter()
                .map(|dragon_move| {
                    let mut board_clone = board.clone();
                    board_clone.dragon = dragon_move.to_owned();
                    // board_clone.print_board();
                    // println!("Dragon move: {:?}", dragon_move);
                    board_clone.remove_sheep(&vec![dragon_move.to_owned()]);
                    round_of_chess(&mut board_clone)
                })
                .sum();
            sum
        })
            .sum::<isize>();
        // for move_combination in sheep_moves {
        //     let mut board = board.clone();
        //     let old = move_combination.0;
        //     let sheep_pos = move_combination.1;
        //
        //     if board.sheep_escapes(&sheep_pos) {
        //         // println!("Sheep escapes by {:?}", sheep_pos);
        //         return 0;
        //     }
        //
        //     board.move_one_sheep(old);
        //     // println!("Sheep move: {:?}", sheep_pos);
        //     let dragon_moves = move_dragon_max_times(dragon, 0, 1, (board.width, board.height));
        //     // println!("Sheep move {:?}, dragon moves are: {:?}", move_combination, dragon_moves);
        //     // for dragon_move in dragon_moves {
        //     //     let mut board_clone = board.clone();
        //     //     board_clone.dragon = dragon_move;
        //     //     // board_clone.print_board();
        //     //     // println!("Dragon move: {:?}", dragon_move);
        //     //     board_clone.remove_sheep(&vec![dragon_move]);
        //     //     unique_sequences += round_of_chess(&mut board_clone);
        //     // }
        //     let sum: isize = dragon_moves.par_iter()
        //         .map(|dragon_move| {
        //             let mut board_clone = board.clone();
        //             board_clone.dragon = dragon_move.to_owned();
        //             // board_clone.print_board();
        //             // println!("Dragon move: {:?}", dragon_move);
        //             board_clone.remove_sheep(&vec![dragon_move.to_owned()]);
        //             round_of_chess(&mut board_clone)
        //         })
        //         .sum();
        //     unique_sequences += sum;
        // }
    }


    // println!("This part should never have to be reached!");
    // println!("Unique sequences: {:?}", unique_sequences);
    unique_sequences
}

fn recurse_board(mut board: ChessBoard) -> usize {
    let mut unique_sequences = 0;

    if board.sheep.is_empty() {
        return 1;
    }
    let dragon = board.dragon;
    let sheep_moves = board.get_possible_sheep_moves();

    if sheep_moves.is_empty() {
        let dragon_moves = move_dragon_max_times(board.dragon, 0, 1, (board.width, board.height));
        for dragon_move in dragon_moves {
            let mut board_clone = board.clone();
            board_clone.dragon = dragon_move;
            board_clone.remove_sheep(&vec![dragon_move]);
            unique_sequences += recurse_board(board_clone);
        }
    }
    else {
        for sheep_combination in sheep_moves {
            let old = sheep_combination.0;
            let new = sheep_combination.1;
            if board.sheep_escapes(&new) {
                return 0;
            }

            let mut board_clone = board.clone();
            board_clone.move_one_sheep(new);

            let dragon_moves = move_dragon_max_times(dragon, 0, 1, (board.width, board.height));
            for dragon_move in dragon_moves {
                let mut board_clone_2 = board_clone.clone();
                board_clone_2.dragon = dragon_move;
                board_clone_2.remove_sheep(&vec![dragon_move]);
                unique_sequences += recurse_board(board_clone_2);
            }
        }
    }
    unique_sequences
}

// #[memoize]
// fn dfs(mut board: ChessBoard) -> usize {
//     let mut unique_sequences: HashSet<Vec<ChessBoard>> = HashSet::new();
//
//     let mut stack: Vec<(ChessBoard, Vec<ChessBoard>)> = vec![(board, vec![])];
//
//     while let Some((board, mut path)) = stack.pop() {
//         if path.contains(&board) {
//             continue;
//         }
//         // board.print_board();
//         path.push(board.clone());
//
//         if board.sheep.is_empty() {
//             // println!("All sheep killed");
//             // board.print_board();
//             unique_sequences.insert(path.clone());
//             continue;
//         }
//         let sheep_move_boards = board.move_sheep_get_boards();
//
//         if sheep_move_boards.len() == 1 && sheep_move_boards[0] == board {
//             // println!("No sheep moves found!");
//             // board.print_board();
//         }
//
//         if sheep_move_boards.is_empty() {
//             println!("Empty move board!");
//             continue;
//         }
//
//         for sheep_move_board in &sheep_move_boards {
//             // println!("Sheep moves");
//             // sheep_move_board.print_board();
//         }
//
//         // sheep_move_boards.into_iter()
//         //     .flat_map(|b| move_dragon_get_boards(b))
//         //     .for_each(|mut b| {
//         //         b.remove_sheep_current_dragon();
//         //         stack.push((b, path.clone()));
//         //     });
//
//         for sheep_board in sheep_move_boards {
//             let dragon_boards = move_dragon_get_boards(sheep_board);
//
//             // If dragon has no valid moves, this path also fails
//             if dragon_boards.is_empty() {
//                 continue;
//             }
//
//             for mut dragon_board in dragon_boards {
//                 dragon_board.remove_sheep_current_dragon();
//                 stack.push((dragon_board, path.clone()));
//             }
//         }
//         // for dragon_board in &dragon_boards {
//             // println!("Dragon moves");
//             // dragon_board.print_board();
//             // if board.sheep.is_empty() {
//             //     println!("All sheep killed");
//             // }
//         // }
//         // stack.append(&mut dragon_boards);
//     }
//     unique_sequences.len()
// }

fn dfs_memo(board: ChessBoard, memo: &mut HashMap<ChessBoard, usize>) -> usize {
    // Check memo
    if let Some(&result) = memo.get(&board) {
        return 0;
    }

    if board.sheep.is_empty() {
        return 1;
    }

    let sheep_move_boards = board.move_sheep_get_boards();

    if sheep_move_boards.is_empty() {
        return 0; // Sheep escaped
    }

    let mut total = 0;
    for sheep_board in sheep_move_boards {
        let dragon_boards = move_dragon_get_boards(sheep_board);

        for mut dragon_board in dragon_boards {
            dragon_board.remove_sheep_current_dragon();
            total += dfs_memo(dragon_board, memo);
        }
    }

    memo.insert(board, total);
    let counts = memo.values().max().unwrap();
    println!("Total: {}", counts);
    total
}

fn move_dragon_get_boards(board: ChessBoard) -> Vec<ChessBoard> {
    // Return a copy of the boards after each possible dragon move on the current board
    let row = board.dragon.0;
    let col = board.dragon.1;

    let mut new_points = Vec::from([(row + 2, col + 1), (row + 2, col - 1), (row + 1, col + 2), (row + 1, col - 2),
        (row - 1, col + 2), (row - 1, col - 2), (row - 2, col + 1), (row - 2, col - 1)]);
    new_points = new_points.into_iter()
        .filter(|&point| is_valid_move(&point, board.width, board.height))
        .collect::<Vec<(isize, isize)>>();

    new_points.into_iter()
        .map(|new| {
            let mut board_clone = board.clone();
            board_clone.dragon = new;
            board_clone
        })
        .collect::<Vec<ChessBoard>>()
}

fn move_dragon_and_kill(mut board: ChessBoard, new_pos: &(isize, isize)) -> ChessBoard {
    board.dragon = new_pos.to_owned();
    board.remove_sheep(&vec![new_pos.to_owned()]);
    board
}

fn get_killed_sheep(visited: &HashSet<(isize, isize)>, board: &ChessBoard) -> Vec<(isize, isize)> {
    board.sheep.iter()
        .filter(|&pos| visited.contains(&pos) && !board.safe.contains(pos))
        .cloned()
        .collect()
}

fn move_dragon_max_times(dragon: (isize, isize), n: isize, max: isize, dimensions: (isize, isize)) -> HashSet<(isize, isize)> {
    let width = dimensions.0;
    let height = dimensions.1;

    if n == max {
        return HashSet::new();
    }

    let row = dragon.0;
    let col = dragon.1;

    let mut new_points = HashSet::from([(row + 2, col + 1), (row + 2, col - 1), (row + 1, col + 2), (row + 1, col - 2),
                          (row - 1, col + 2), (row - 1, col - 2), (row - 2, col + 1), (row - 2, col - 1)]);
    new_points = new_points.into_iter()
        .filter(|&point| is_valid_move(&point, width, height))
        .collect::<HashSet<(isize, isize)>>();

    new_points.extend(new_points.iter()
        .flat_map(|point| move_dragon_max_times(*point, n + 1, max, dimensions))
        .collect::<HashSet<(isize, isize)>>());
    new_points

}

fn is_valid_move(end: &(isize, isize), width: isize, height: isize) -> bool {
    end.0 >= 0 && end.0 < height && end.1 >= 0 && end.1 < width
}

fn print_sheep_chess_board(board: &ChessBoard) {
    let mut print_board: Vec<Vec<char>> = vec![vec!['.'; board.width as usize]; board.height as usize];
    board.sheep.iter()
        .for_each(|(row, col)| print_board[*row as usize][*col as usize] = 'S');

    for row in print_board {
        println!("{}", row.iter().collect::<String>());
    }
    println!("\n\n")
}