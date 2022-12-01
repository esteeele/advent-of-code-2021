use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;
use crate::split_input_file_into_lines;

pub fn polymerise() {
    let lines = split_input_file_into_lines("resources/polymers.txt".to_string());
    let initial_polymer = lines.get(0).unwrap();
    println!("{}", initial_polymer);

    let mut rule_map: HashMap<String, String> = HashMap::new();
    for line in &lines {
        if line.contains("->") {
            //is polymer rule
            let mut split_line = line.split("->");
            let pair = split_line.next().unwrap().replace(" ", "");
            let insert_char = split_line.next().unwrap().replace(" ", "");
            rule_map.insert(pair.to_string(), insert_char.to_string());
        }
    }
    let polymer = build_polymer(1, 0, initial_polymer, &rule_map);
    println!("{}", polymer);

    let polymer_chars: Vec<char> = initial_polymer.chars().collect();
    let pairs = convert_polymer_to_pairs(&polymer_chars);
    let mut initial_count = HashMap::new();
    for pair in pairs {
        println!("Initial pair: {}", pair);
        let counter = initial_count.entry(pair).or_insert(0);
        *counter += 1;
    }
    let hashed_polymer = build_hashed_polymer(10, 0, initial_count, &rule_map);

    let mut count_char_map = HashMap::new();
    count_char_map.insert(polymer_chars[0], 1);
    count_char_map.insert(polymer_chars[polymer_chars.len()-1], 1);
    for (key, value) in &hashed_polymer {
        println!("{}, {}", key, value);
        let split_key: Vec<char> = key.chars().collect();
        let char_1_counter = count_char_map.entry(split_key[0]).or_insert(0);
        *char_1_counter += value;

        let char_2_counter = count_char_map.entry(split_key[1]).or_insert(0);
        *char_2_counter += value;
    }

    let mut max_char_count: i64 = 0;
    let mut min_char_count: i64 = i64::MAX;
    for (key, value) in &count_char_map {
        max_char_count = max(max_char_count, *value);
        min_char_count = min(min_char_count, *value);
        println!("{}, {}", key, value);
    }
    //half because all pairs contain a shared character e.g. ABC = AB, BC
    println!("Max: {}, Min: {}, Diff: {}", max_char_count / 2, min_char_count / 2,
             (max_char_count - min_char_count) / 2);
}

fn build_polymer(limit: i32, iteration: i32, polymer: &str, rule_map: &HashMap<String, String>) -> String {
    if iteration == limit {
        return polymer.to_string();
    }
    let polymer_chars: Vec<char> = polymer.chars().collect();
    let polymer_pairs = convert_polymer_to_pairs(&polymer_chars);
    let mut new_polymer = String::new();
    new_polymer.push(polymer_chars[0]);
    for pair in &polymer_pairs {
        let pair_chars: Vec<char> = pair.chars().collect();
        let insert_char = rule_map.get(pair);
        let mut triplet = String::new();
        triplet.push_str(insert_char.unwrap());
        triplet.push(pair_chars[1]);
        new_polymer.push_str(&triplet);
    }
    return build_polymer(limit, iteration + 1, &new_polymer, rule_map);
}

fn convert_polymer_to_pairs(polymer_chars: &Vec<char>) -> Vec<String> {
    let mut modified_polymer: Vec<String> = Vec::new();
    for head in 1..polymer_chars.len() {
        let tail = head - 1;
        let mut pair = String::new();
        pair.push(polymer_chars[tail]);
        pair.push(polymer_chars[head]);

        modified_polymer.push(pair);
    }
    modified_polymer
}

fn build_hashed_polymer(limit: i32, iteration: i32, mut hashed_pairs: HashMap<String, i64>, rule_map: &HashMap<String, String>) -> HashMap<String, i64> {
    if iteration == limit {
        return hashed_pairs;
    }
    let mut new_hashed_pairs = HashMap::new();
    for (key, value) in hashed_pairs {
        let key_chars : Vec<char> = key.chars().collect();
        let insert_char = rule_map.get(&key);
        let mut first_pair = String::new();
        first_pair.push(key_chars[0]);
        first_pair.push_str(insert_char.unwrap());

        let mut second_pair = String::new();
        second_pair.push_str(insert_char.unwrap());
        second_pair.push(key_chars[1]);

        let counter_first_pair = new_hashed_pairs.entry(first_pair.to_string()).or_insert(0);
        *counter_first_pair += value;

        let counter_second_pair = new_hashed_pairs.entry(second_pair.to_string()).or_insert(0);
        *counter_second_pair += value;
    }
    return build_hashed_polymer(limit, iteration + 1, new_hashed_pairs, rule_map);
}

