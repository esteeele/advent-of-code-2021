//figure out alg to perform the fold 'up' / 'left' for all the points created so far

//essentially move up any points below the line the diff from where the line is

use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ptr::null;
use crate::split_input_file_into_lines;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32
}

struct FoldInstruction {
    coord: String,
    value: i32
}

pub fn fold_points() {
    let lines = split_input_file_into_lines("resources/folds.txt".to_string());
    let mut lines_coords: HashSet<Point> = HashSet::new();
    let mut fold_instructions: Vec<FoldInstruction> = Vec::new();
    for line in lines {
        if line.contains(",") {
            let mut split_lines = line.split(",");
            let x = split_lines.next().unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = split_lines.next().unwrap();
            let y = y.parse::<i32>().unwrap();
            lines_coords.insert(Point { x, y });
        } else if line.contains("=") {
            let instruction = line.to_string().replace("fold along", "");
            let mut split_instruction = instruction.split("=");
            let coord = split_instruction.next().unwrap();
            let value =  split_instruction.next().unwrap();
            fold_instructions.push(FoldInstruction {
                coord: coord.trim().to_string(),
                value: value.parse::<i32>().unwrap()
            });
        }
    }

    let first_instruction = fold_instructions.get(0).unwrap();
    let transformed_points = fold(&lines_coords, first_instruction);

    //part 1
    println!("Number of visible points {}", transformed_points.len());

    let mut coords = lines_coords;
    for instruction in &fold_instructions {
        println!("Instruction {}={}", instruction.coord, instruction.value);
        let transformed_points = fold(&coords, instruction);
        coords = transformed_points;
    }
    println!("Number of visible points after all folds {}", coords.len());

    let mut max_x = 0;
    let mut max_y = 0;
    for point in &coords {
        max_x = max(max_x, point.x);
        max_y = max(max_y, point.y);
    }
    print_coords(&coords, max_x, max_y);
}

fn fold(lines_coords: &HashSet<Point>,
        instruction: &FoldInstruction) -> HashSet<Point> {
    let mut transformed_points = HashSet::new();
    for coord in lines_coords {
        let transformed_point = transform_point(coord, instruction);
        if transformed_point.x >= 0 && transformed_point.y >= 0 {
            transformed_points.insert(transformed_point);
        }
    }
    return transformed_points;
}

fn print_coords(points: &HashSet<Point>, max_x : i32, max_y: i32) {
    let mut file = fs::OpenOptions::new().write(true).append(true)
        .open("fold-results.txt")
        .unwrap();
    for y in 0..max_y+1 {
        let mut line: String = "".to_string();
        for x in 0..max_x+1 {
            let point = Point {x, y};
            if points.contains(&point) {
                line += "#";
            } else {
                line += " ";
            }
        }
        line += "\n";
        file.write(line.as_bytes());
    }
    file.write("\n".as_bytes());
}

fn transform_point(point: &Point, instruction: &FoldInstruction) -> Point {
    if instruction.coord == "y" {
        // println!("y transform value={}, y={}", instruction.value, point.y);
        if point.y > instruction.value {
            //need to move point 'up'
            let diff = point.y - instruction.value;
            return Point {
                x : point.x,
                y: point.y - diff * 2
            }
        }
    } else {
        if point.x > instruction.value {
            let diff = point.x - instruction.value;
            return Point {
                x: point.x - diff * 2,
                y: point.y
            }
        }
    }

    return Point {x: point.x, y: point.y};
}