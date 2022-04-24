use std::collections::{HashMap, HashSet};
use crate::split_input_file_into_lines;

struct cave {
    name: String,
    connections: HashSet<String>,
    is_big: bool
}

pub fn count_paths() -> i32 {
    let mut caves: HashMap<String, cave> = HashMap::new();
    let lines = split_input_file_into_lines("resources/routes.txt".to_string());

    for line in lines {
        let mut split_line = line.split("-");

        let entry_cave = split_line.next().unwrap().to_string();
        let exit_cave = split_line.next().unwrap().to_string();

        if is_big_cave(entry_cave.to_string()) && is_big_cave(exit_cave.to_string()) {
            println!("{}", "Unprepared for this input, would infinitely loop");
            return -1;
        }

        //need to add forwards and backwards connection
        add_new_caves(&mut caves, entry_cave.to_string(), exit_cave.to_string());
        add_new_caves(&mut caves, exit_cave.to_string(), entry_cave.to_string());
    }

    /*
    Rules
     - Can only visit small cave once
     - Cannot revisit start and end
     - Each path must be unique
     - Assume there's only 1 start and end
     */


    let cave_names: Vec<String> = caves.keys().into_iter()
        .filter(|cave_name| !is_big_cave(cave_name.to_string())
            && cave_name.to_string() != "end" && cave_name.to_string() != "start")
        .map(|s| s.to_string())
        .collect();

    let start = caves.get("start").unwrap();
    let mut all_routes: HashSet<Vec<String>> = HashSet::new();
    let mut total_count = 0;

    //should memoise this to avoid recomputing these same paths ... wonder how it would run on worse hardware
    for special_small_cave in cave_names {
        let mut routes_for_special_cave: HashSet<Vec<String>> = explore_cave(start, &caves,
                                                            HashSet::new(), Vec::new(),
                                                            special_small_cave, 0);
        for special_route in routes_for_special_cave {
            all_routes.insert(special_route);
        }
    }

    println!("Size all routes {}", all_routes.len());
    return 0;
}

fn add_new_caves(caves: &mut HashMap<String, cave>, first_cave: String, second_cave: String) {
    if caves.contains_key(&first_cave) {
        let existing_cave = caves.get(&first_cave).unwrap();

        let new_connections = copy_existing_connections(second_cave.to_string(), existing_cave);

        caves.insert(first_cave.to_string(), cave {
            name: first_cave.to_string(),
            connections: new_connections,
            is_big: is_big_cave(first_cave.to_string())
        });
    } else {
        caves.insert(first_cave.to_string(), cave {
            name: first_cave.to_string(),
            connections: HashSet::from([second_cave.to_string()]),
            is_big: is_big_cave(first_cave.to_string())
        });
    }
}

fn explore_cave(cave: &cave, caves_map: &HashMap<String, cave>,
                mut visited_small_caves: HashSet<String>, mut route: Vec<String>,
                small_cave: String, times_used_small_cave: i32) -> HashSet<Vec<String>>  {
    route.push(cave.name.to_string());
    let mut used_small_cave = times_used_small_cave;
    if cave.name.to_string() == small_cave {
        used_small_cave += 1;
    }

    //have we reached the end?
    if cave.name.eq_ignore_ascii_case("end") {
        let mut existing_route: Vec<String> = route.iter().map(|s| s.to_string()).collect();
        let mut result: HashSet<Vec<String>> = HashSet::new();
        result.insert(existing_route);
        return result;
    }

    if !cave.is_big {
        if cave.name.to_string() == small_cave {
            if used_small_cave > 1 {
                visited_small_caves.insert(cave.name.to_string());
            }
        } else {
            visited_small_caves.insert(cave.name.to_string());
        }
    }

    let mut all_sub_routes: HashSet<Vec<String>> = HashSet::new();
    //append on the sub routes to whatever the parent route was
    for connection in &cave.connections {
        if !visited_small_caves.contains(connection)
            && !connection.eq_ignore_ascii_case("start") {

            //cave name will be there
            let new_sub_routes = explore_cave(caves_map.get(connection).unwrap(), &caves_map,
                                visited_small_caves.iter().map(|s| s.to_string()).collect(),
                                              route.iter().map(|s| s.to_string()).collect(),
                                                    small_cave.to_string(), used_small_cave);

            //for each new route attach to the base route
            for new_route in &new_sub_routes {
                let mut new_route :Vec<String> = new_route.iter().map(|s| s.to_string()).collect();
                all_sub_routes.insert(new_route);
            }
        }
    }

    return all_sub_routes;
}

fn copy_existing_connections(exit_cave: String, existing_cave: &cave) -> HashSet<String> {
    let mut new_connections = HashSet::new();
    for connection in &existing_cave.connections {
        new_connections.insert(connection.to_string());
    };
    new_connections.insert(exit_cave.to_string());
    new_connections
}

fn is_big_cave(cave_name: String) -> bool {
    return cave_name.chars().all(|c| c.is_uppercase());
}