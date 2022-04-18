//from each opening bracket find its closing bracket then recursively find all pairs within that

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct LineParseResult {
    line_type: String,
    score: i64
}

fn brackets_map() -> HashMap<char, char> {
    return HashMap::from([
        ('{', '}'),
        ('(', ')'),
        ('[', ']'),
        ('<', '>')
    ]);
}

fn find_inverse_bracket(bracket: char) -> char {
    match bracket {
        '(' => ')',
        ')' => '(',
        '{' => '}',
        '}' => '{',
        '[' => ']',
        ']' => '[',
        '<' => '>',
        '>' => '<',
        _ => '?'
    }
}

fn brackets_score(character: char) -> i64 {
    if character == ')' {
        return 3;
    } else if character == ']' {
        return 57;
    } else if character == '}' {
        return 1197;
    } else if character == '>' {
        return 25137
    }
    return 0;
}

pub fn braces_syntax() {
    let file = File::open("resources/d10-input.txt").unwrap();
    let reader = BufReader::new(file);

    let allowed_values: HashSet<char> = HashSet::from(
        ['{', '}', '(', ')', '[', ']', '<', '>']
    );

    let mut total_corrupted_score = 0;
    let mut total_incomplete_scores: Vec<i64> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut line_chars: Vec<char> = Vec::new();
        for char in line.chars() {
            if allowed_values.contains(&char) {
                line_chars.push(char);
            }
        }
        let line_result = process_line(line_chars);
        if line_result.line_type == "CORRUPTED" {
            total_corrupted_score += line_result.score;
        } else if line_result.line_type == "INCOMPLETE" {
            total_incomplete_scores.push(line_result.score);
        }
    }

    println!("Total score part 1 = {}", total_corrupted_score);
    //get middle item
    total_incomplete_scores.sort();
    println!("Total score part 2 = {}", total_incomplete_scores.get(total_incomplete_scores.len() / 2).unwrap());
}

fn process_line(line : Vec<char>) -> LineParseResult {
    let mut brackets_stack: Vec<char> = Vec::new();
    for char in line {
        if brackets_map().contains_key(&char) {
            brackets_stack.push(char);
        } else {
            let last_stacked = brackets_stack.pop().unwrap();
            let inverse_bracket = find_inverse_bracket(last_stacked);
            if char != inverse_bracket {
                return LineParseResult {
                    line_type: "CORRUPTED".to_string(),
                    score: brackets_score(char)
                };
            }
        }
    }

    if brackets_stack.is_empty() {
        return LineParseResult {
            line_type: "PERFECT".to_string(),
            score: 0
        };
    }

    let mut closing_score: i64 = 0;
    brackets_stack.reverse();
    for character_remaining in brackets_stack {
        let inverse_bracket = find_inverse_bracket(character_remaining);
        let bracket_score = match inverse_bracket {
            ')' => 1,
            ']'  => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        };
        closing_score = (5 * closing_score) + bracket_score;
    }

    return LineParseResult {
        line_type: "INCOMPLETE".to_string(),
        score: closing_score
    };
}