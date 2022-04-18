use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    value: u32,
    column: usize,
    row: usize,
}


pub fn calc_number_flashes() {
    let mut start_array = convert_to_digit_array();
    let mut number_flashed_points = 0;

    let mut all_flash = false;
    let mut iteration = 1;
    while !all_flash {
        let mut incremented_array = increment_jellies(&mut start_array);

        let mut flash_memory: Vec<Vec<bool>> = create_flash_memory();

        //find all >9s and radiate out from them flashing everything next door
        let mut flashed_points = find_new_flash_points(&incremented_array, &mut flash_memory);
        number_flashed_points += flashed_points.len();

        while !flashed_points.is_empty() {
            //stage 2 update all points around the flashed points
            let flashed_array = incremement_neighbour_induced_flashes(&incremented_array, &flashed_points);
            let neighbour_induced_flashes = find_new_flash_points(&flashed_array, &mut flash_memory);
            number_flashed_points += neighbour_induced_flashes.len();
            flashed_points = neighbour_induced_flashes;
            incremented_array = flashed_array;
        }

        start_array = reset_flashied_jellies(&incremented_array);

        all_flash = all_jellies_flash(&start_array);
        if all_flash {
            println!("All jellies have flashed on {}", iteration)
        }
        iteration += 1;
    }

    println!("{}", number_flashed_points);
}

fn reset_flashied_jellies(incremented_array: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    incremented_array.into_iter()
        .map(|line| line.into_iter().map(|point| {
            if point.value > 9 {
                Point {
                    value: 0,
                    column: point.column,
                    row: point.row,
                }
            } else {
                Point {
                    value: point.value,
                    column: point.column,
                    row: point.row,
                }
            }
        }).collect())
        .collect()
}

fn incremement_neighbour_induced_flashes(incremented_array: &Vec<Vec<Point>>, flashed_points: &Vec<Point>) -> Vec<Vec<Point>> {
    let flashed_array: Vec<Vec<Point>> = incremented_array.into_iter()
        .map(|line| line.into_iter().map(|point| {
            let number_neighbouring_flashes: u32 = calc_amount_to_increment_point(
                point.row, point.column, &flashed_points);
            return Point {
                value: point.value + number_neighbouring_flashes,
                column: point.column,
                row: point.row,
            };
        }).collect())
        .collect();
    flashed_array
}

fn increment_jellies(start_array: &mut Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    let mut incremented_array: Vec<Vec<Point>> = start_array.into_iter()
        .map(|line| line.into_iter().map(|point| Point {
            value: point.value + 1,
            column: point.column,
            row: point.row,
        }).collect())
        .collect();
    incremented_array
}

fn all_jellies_flash(jellies: &Vec<Vec<Point>>) -> bool {
    for row in jellies {
        for col in row {
            if col.value > 0 {
                return false;
            }
        }
    }
    return true;
}

fn find_new_flash_points(incremented_array: &Vec<Vec<Point>>,
                         flash_memory: &mut Vec<Vec<bool>>) -> Vec<Point> {
    let mut flashed_points: Vec<Point> = Vec::new();
    let mut row_index = 0;
    for row in incremented_array {
        for point in row {
            if point.value > 9 && !flash_memory[point.row][point.column] {
                flash_memory[point.row][point.column] = true;

                flashed_points.push(Point {
                    value: point.value,
                    column: point.column,
                    row: point.row,
                });
            }
        }
        row_index += 1;
    }
    flashed_points
}

fn calc_amount_to_increment_point(row: usize, col: usize, flashed_points: &Vec<Point>) -> u32 {
    let mut amount_to_increment = 0;
    for flashed_point in flashed_points {
        if (col + 1) >= flashed_point.column && col <= flashed_point.column + 1
            && (row + 1) >= flashed_point.row && row <= flashed_point.row + 1 {

            amount_to_increment += 1;
        }
    }
    return amount_to_increment;
}

fn create_flash_memory() -> Vec<Vec<bool>> {
    let mut flashed_memory: Vec<Vec<bool>> = Vec::with_capacity(10);

    for _ in 0..10 {
        let mut flashed_memory_row = Vec::with_capacity(10);
        for _ in 0..10 {
            flashed_memory_row.push(false);
        }
        flashed_memory.push(flashed_memory_row);
    }
    return flashed_memory;
}

fn convert_to_digit_array() -> Vec<Vec<Point>> {
    let file = File::open("resources/d11-input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut digit_array: Vec<Vec<Point>> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut digit_line: Vec<Point> = Vec::new();
        let mut col_index = 0;
        for character in line.chars() {
            let value = character.to_digit(10).unwrap();
            digit_line.push(Point {
                value,
                column: col_index,
                row: index,
            });
            col_index += 1;
        }
        digit_array.push(digit_line);
    }

    return digit_array;
}