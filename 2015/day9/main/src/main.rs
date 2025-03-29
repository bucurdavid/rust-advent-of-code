use std::collections::HashMap;
use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("../input.txt")?;
    let mut cities: Vec<String> = Vec::new();
    let mut city_indices: HashMap<String, usize> = HashMap::new();
    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() != 2 {
            eprintln!("Warning: Skipping invalid line format: {}", line);
            continue;
        }

        let cities_part = parts[0];
        let distance: u32 = match parts[1].parse() {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Warning: Couldn't parse distance in line: {}", line);
                continue;
            }
        };

        let city_parts: Vec<&str> = cities_part.split(" to ").collect();
        if city_parts.len() != 2 {
            eprintln!("Warning: Couldn't parse cities in line: {}", line);
            continue;
        }

        let city1 = city_parts[0].to_string();
        let city2 = city_parts[1].to_string();

        // Add cities to the list if they're not already there
        if !city_indices.contains_key(&city1) {
            city_indices.insert(city1.clone(), cities.len());
            cities.push(city1.clone());
        }
        if !city_indices.contains_key(&city2) {
            city_indices.insert(city2.clone(), cities.len());
            cities.push(city2.clone());
        }

        // Store the distance between cities
        let idx1 = *city_indices.get(&city1).unwrap();
        let idx2 = *city_indices.get(&city2).unwrap();

        distances.insert((idx1, idx2), distance);
        distances.insert((idx2, idx1), distance);
    }

    let n = cities.len();
    if n == 0 {
        println!("No cities found in input. Check your input.txt file format.");
        return Ok(());
    }

    let shortest = find_shortest_path(&distances, n);

    let longest = find_longest_path(&distances, n);

    println!("Shortest route distance: {}", shortest);
    println!("Longest route distance: {}", longest);

    Ok(())
}

fn find_shortest_path(distances: &HashMap<(usize, usize), u32>, n: usize) -> u32 {
    let mut shortest_distance = u32::MAX;

    for start in 0..n {
        let mut dp = vec![vec![None; n]; 1 << n];

        let initial_mask = 1 << start;

        for end in 0..n {
            if end != start {
                let path_distance =
                    shortest_hamiltonian_path(start, initial_mask, end, distances, &mut dp, n);

                if path_distance < shortest_distance {
                    shortest_distance = path_distance;
                }
            }
        }
    }

    shortest_distance
}

fn find_longest_path(distances: &HashMap<(usize, usize), u32>, n: usize) -> u32 {
    let mut longest_distance = 0;

    for start in 0..n {
        let mut dp = vec![vec![None; n]; 1 << n];

        let initial_mask = 1 << start;

        for end in 0..n {
            if end != start {
                let path_distance =
                    longest_hamiltonian_path(start, initial_mask, end, distances, &mut dp, n);

                if path_distance > longest_distance {
                    longest_distance = path_distance;
                }
            }
        }
    }

    longest_distance
}

fn shortest_hamiltonian_path(
    current: usize,
    mask: usize,
    final_city: usize,
    distances: &HashMap<(usize, usize), u32>,
    dp: &mut Vec<Vec<Option<u32>>>,
    n: usize,
) -> u32 {
    let all_visited = (1 << n) - 1;

    if mask == all_visited {
        return 0;
    }

    if let Some(result) = dp[mask][current] {
        return result;
    }

    let mut min_dist = u32::MAX;

    for next in 0..n {
        let next_mask = 1 << next;

        if (mask & next_mask) != 0 {
            continue;
        }

        if let Some(&dist) = distances.get(&(current, next)) {
            let new_mask = mask | next_mask;

            if next == final_city && new_mask == all_visited {
                min_dist = min_dist.min(dist);
            } else {
                let sub_path =
                    shortest_hamiltonian_path(next, new_mask, final_city, distances, dp, n);

                if sub_path != u32::MAX {
                    min_dist = min_dist.min(dist + sub_path);
                }
            }
        }
    }

    dp[mask][current] = Some(min_dist);
    min_dist
}

fn longest_hamiltonian_path(
    current: usize,
    mask: usize,
    final_city: usize,
    distances: &HashMap<(usize, usize), u32>,
    dp: &mut Vec<Vec<Option<u32>>>,
    n: usize,
) -> u32 {
    let all_visited = (1 << n) - 1;

    if mask == all_visited {
        return 0;
    }

    if let Some(result) = dp[mask][current] {
        return result;
    }

    let mut max_dist = 0;

    for next in 0..n {
        let next_mask = 1 << next;

        if (mask & next_mask) != 0 {
            continue;
        }

        if let Some(&dist) = distances.get(&(current, next)) {
            let new_mask = mask | next_mask;

            if next == final_city && new_mask == all_visited {
                max_dist = max_dist.max(dist);
            } else {
                let sub_path =
                    longest_hamiltonian_path(next, new_mask, final_city, distances, dp, n);
                max_dist = max_dist.max(dist + sub_path);
            }
        }
    }

    dp[mask][current] = Some(max_dist);
    max_dist
}
