mod syntax;
mod day_nine;
mod jumbo_jellies;
mod paths;
mod folds;
mod polymers;
mod risk_factors;

use std::alloc::System;
use std::cmp::min;
use std::collections::HashSet;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::os::raw::c_char;

fn main() {
    // folds::fold_points();
    // polymers::polymerise();
    risk_factors::calculate_risk_factor();
}

pub fn split_input_file_into_lines(file_location: String) -> Vec<String> {
    let file = File::open(file_location).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        lines.push(line.unwrap());
    }

    return lines;
}