mod syntax;
mod day_nine;
use std::alloc::System;
use std::cmp::min;
use std::collections::HashSet;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::os::raw::c_char;

fn main() {
    syntax::braces_syntax();
}