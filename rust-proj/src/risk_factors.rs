use std::cmp::min;
use std::collections::{HashMap, HashSet};
use crate::split_input_file_into_lines;

pub fn calculate_risk_factor() {
    let lines = split_input_file_into_lines("resources/risk-factors.txt".to_string());
    //transform to in memory vector list
    let risk_factors: Vec<Vec<usize>> = lines.iter()
        .map(|x| x.chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect()
        )
        .collect();
    let board_size = risk_factors.len();

    //exhaustively move through the whole map (if only move Right or down don't need to worry about repeating)
    //keep track of min score so far and stop if exceeding it?

    // calc_minimum_risk(&risk_factors, board_size);

    /*
    **** PART 2 ****
     */
    println!("Board size {}", board_size);
    let mut super_board: Vec<Vec<usize>> = Vec::new();
    for down in 0..board_size * 5 {
        let mut row : Vec<usize> = Vec::new();
        for right in 0..board_size * 5 {
            let offset_right = right % board_size;
            let offset_down = down % board_size;
            let original_value = risk_factors[offset_down][offset_right];

            let number_to_add_right = right / board_size;
            let number_to_add_down = down / board_size;

            let mut new_value = original_value + number_to_add_down + number_to_add_right;
            if new_value > 9 {
                new_value = new_value % 9;
            }
            row.push(new_value);
        }
        super_board.push(row);
    }

    for row in &super_board {
        for value in row {
            print!("{}", value);
        }
        println!();
    }

    calc_minimum_risk(&super_board, board_size * 5);
}

fn calc_minimum_risk(risk_factors: &Vec<Vec<usize>>, board_size: usize) {
    let mut board_with_minimum_risk_paths: Vec<Vec<u32>> = vec![vec![0; board_size]; board_size];
    let mut seen_locations: HashSet<Pair> = HashSet::new();

    //descending for loop
    for x in (0..board_size).rev() {
        for y in (0..board_size).rev() {
            let local_min = derive_risk_factor_for_position(x, y, board_size, 0,
                                            &risk_factors, &mut board_with_minimum_risk_paths,
                                            &mut seen_locations);
            println!("For x: {}, y: {}, Local Min is: {}", x, y, local_min);
        }
    }

    //answer is 2935 (guessing 9 too high ... why ...)
    let min_risk = board_with_minimum_risk_paths[0][0] - risk_factors[0][0] as u32;
    println!("Min risk is {}", min_risk);
}

/*
Calculate all possible solutions and choose the minimum?
 */
fn simple_solve() {
    //nah
}

/*
Almost works but not quite: think the issue is that we sometimes need to recalculate stored values and this never does
 */
fn derive_risk_factor_for_position(x: usize, y: usize, row_size: usize, risk_factor: u32,
                                   risk_factors: &Vec<Vec<usize>>, board_with_minimum_risk_paths: &mut Vec<Vec<u32>>,
                                   seen_locations: &mut HashSet<Pair>) -> u32 {
    seen_locations.insert(Pair{x, y});
    let new_risk_factor = risk_factor + risk_factors[x][y] as u32;

    if x == row_size - 1 && y == row_size - 1 {
        //have reached end
        return new_risk_factor;
    }

    if board_with_minimum_risk_paths[x][y] > 0 {
        return risk_factor + board_with_minimum_risk_paths[x][y];
    }

    //find the minimum risk from this position
    let valid_moves = find_valid_next_moves(x, y, row_size);
    let mut min_score = u32::MAX;
    for pair in valid_moves {
        if seen_locations.contains(&pair) {
            let min_score_for_move = derive_risk_factor_for_position(pair.x, pair.y, row_size,
                                                                     new_risk_factor, risk_factors, board_with_minimum_risk_paths,
                                                                     seen_locations);
            min_score = min(min_score, min_score_for_move);
        }
    }

    //memoise result
    board_with_minimum_risk_paths[x][y] = min_score;

    return min_score;
}

/*
Can for each position work out which is the min path to the destination then use that as guide

Start at the bottom right and move outwards calculating risks
 */

fn find_valid_next_moves(x: usize, y: usize, row_size: usize) -> Vec<Pair> {
    let mut next_moves:  Vec<Pair>  = Vec::new();
    if x > 0 {
        next_moves.push(Pair {
            x: x -1, y
        });
    }
    if x < row_size - 1 {
        next_moves.push(Pair {
            x: x + 1, y
        });
    }
    if y > 0 {
        next_moves.push(Pair {
            x, y: y - 1
        });
    }
    if y < row_size - 1 {
        next_moves.push(Pair {
            x, y: y + 1
        });
    }
    return next_moves;
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Pair {
    x: usize,
    y: usize
}