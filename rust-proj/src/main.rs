use std::alloc::System;
use std::cmp::min;
use std::collections::HashSet;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::os::raw::c_char;

fn main() {
    work_out_advent_of_code_lava_day_9();
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    value: u32,
    column: usize,
    row: usize,
}

fn work_out_advent_of_code_lava_day_9() {
    let file = File::open("bar.txt").unwrap();
    let reader = BufReader::new(file);

    let values = convert_input_to_point_array(reader);

    let mut total_risk = 0;
    let mut basins: Vec<usize> = Vec::new();
    for (row_index, row_value) in values.iter().enumerate() {
        for (column_index, column_point) in row_value.iter().enumerate() {
            // println!("{}, {}, {}", *column_value, row_index, column_index);
            if is_local_min(column_point.value, &values, row_index, column_index) {
                total_risk = total_risk + column_point.value + 1;

                //here radiate out with all the points for this low point
                let basin = find_basin_size(
                    &values, Point {value: column_point.value, column: column_point.column, row: column_point.row } );
                //don't really need to store all these values only current 3 largest but lazy
                basins.push(basin.len());
                println!("Have found basin with size {}", basin.len());
            }
        }
    }
    println!("{}", total_risk);
    //reverse sort
    basins.sort_by(|a, b| b.cmp(a));
    println!("Top 3 basin size: {}", basins[0] * basins[1] * basins[2]);
}

fn convert_input_to_point_array(reader: BufReader<File>) -> Vec<Vec<Point>> {
    let mut values: Vec<Vec<Point>> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut iter = 0;
        let mut number_list: Vec<Point> = Vec::new();
        for character in line.chars() {
            let value = character.to_digit(10).unwrap();
            let point = Point {
                value,
                column: iter,
                row: index
            };
            number_list.push(point);
            iter = iter + 1;
        }
        values.push(number_list);
    }
    values
}

fn is_local_min(number: u32, values: &Vec<Vec<Point>>, row: usize, column: usize) -> bool {
    if number == 9 {
        return false;
    }
    let above_value = if row == 0 {9} else {values[row-1][column].value};
    let below_value = if row >= values.len() - 1 {9} else {values[row+1][column].value};
    let left_value = if column == 0 {9} else { values[row][column-1].value };
    let right_value = if column >= values[0].len() - 1 {9} else { values[row][column+1].value };

    let minimum_local_point = min(above_value, min(below_value, min(left_value, right_value)));
    // println!("{}, {}, {}, {}, {}", above_value, below_value, left_value, right_value, minimum_local_point);
    return number < minimum_local_point;
}

fn find_neighbouring_points(values: &Vec<Vec<Point>>, row: usize, column: usize) -> Vec<Point> {
    let mut neighbouring_points: Vec<Point>= Vec::new();
    if row != 0 {
        neighbouring_points.push(Point {
            value: values[row-1][column].value,
            column,
            row: row-1
        })
    };
    if row < values.len() - 1 {
        neighbouring_points.push(Point {
            value: values[row+1][column].value,
            column,
            row: row + 1
        })
    };
    if column != 0 {
        neighbouring_points.push(Point {
            value: values[row][column-1].value,
            column: column - 1,
            row
        })
    };
    if column < values[0].len() - 1 {
        neighbouring_points.push(Point {
            value: values[row][column+1].value,
            column: column+1,
            row
        })
    };

    return neighbouring_points;
}

fn find_basin_size(values: &Vec<Vec<Point>>, original_point: Point) -> Vec<Point> {
    let mut visited_points: HashSet<Point> = HashSet::new();
    return find_points_to_visit(values, original_point, &mut visited_points);
}

fn find_points_to_visit(values: &Vec<Vec<Point>>, original_point: Point,
                        visited_points: &mut HashSet<Point>) -> Vec<Point>  {
    let neighbouring_points = find_neighbouring_points(values, original_point.row, original_point.column);
    let mut points_to_visit: Vec<Point> = Vec::new();

    for point in neighbouring_points {
        if !visited_points.contains(&point) && point.value < 9 {
            visited_points.insert(Point{value: point.value,
                column: point.column, row: point.row
            });
            points_to_visit.push(Point{value: point.value,
                column: point.column, row: point.row
            });

            let mut downstream_points: Vec<Point> =
                find_points_to_visit(values, Point{value: point.value,
                    column: point.column,
                    row: point.row
                }, visited_points);
            points_to_visit.append(&mut downstream_points);
        }
    }

    return points_to_visit;
}
