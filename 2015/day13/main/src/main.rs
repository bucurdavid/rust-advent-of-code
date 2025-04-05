use std::{collections::HashMap, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("../input.txt")?;

    let mut happiness_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let person = parts[0].to_string();
        let neighbor = parts.last().unwrap().trim_end_matches('.').to_string();
        let sign = parts[2];
        let value = parts[3].parse::<i32>().unwrap();
        let happiness = if sign == "gain" { value } else { -value };

        happiness_map
            .entry(person)
            .or_insert_with(HashMap::new)
            .insert(neighbor, happiness);
    }

    let people: Vec<String> = happiness_map.keys().cloned().collect();

    let max_happiness = find_max_happiness(&people, &happiness_map);
    println!("Part 1 - Maximum possible happiness: {}", max_happiness);

    let mut updated_happiness_map = happiness_map.clone();
    let me = "Me".to_string();
    let mut my_relations = HashMap::new();

    for person in &people {
        my_relations.insert(person.clone(), 0);
        updated_happiness_map
            .get_mut(person)
            .unwrap()
            .insert(me.clone(), 0);
    }

    updated_happiness_map.insert(me.clone(), my_relations);

    let updated_people: Vec<String> = updated_happiness_map.keys().cloned().collect();
    let max_happiness_with_me = find_max_happiness(&updated_people, &updated_happiness_map);

    println!(
        "Part 2 - Maximum possible happiness with you included: {}",
        max_happiness_with_me
    );
    Ok(())
}

fn find_max_happiness(
    people: &[String],
    happiness_map: &HashMap<String, HashMap<String, i32>>,
) -> i32 {
    let mut max_happiness = std::i32::MIN;

    let mut arrangement = people.to_vec();

    fn generate_permutations(
        arr: &mut [String],
        start: usize,
        end: usize,
        happiness_map: &HashMap<String, HashMap<String, i32>>,
        max_happiness: &mut i32,
    ) {
        if start == end {
            let mut total = 0;
            let len = arr.len();

            for i in 0..len {
                let person = &arr[i];
                let left = &arr[(i + len - 1) % len];
                let right = &arr[(i + 1) % len];

                if let Some(person_map) = happiness_map.get(person) {
                    if let Some(happiness) = person_map.get(left) {
                        total += happiness;
                    }
                    if let Some(happiness) = person_map.get(right) {
                        total += happiness;
                    }
                }
            }

            *max_happiness = (*max_happiness).max(total);
        } else {
            for i in start..=end {
                arr.swap(start, i);
                generate_permutations(arr, start + 1, end, happiness_map, max_happiness);
                arr.swap(start, i); // backtrack
            }
        }
    }

    generate_permutations(
        &mut arrangement,
        1,
        people.len() - 1,
        happiness_map,
        &mut max_happiness,
    );

    max_happiness
}
