use std::{collections::HashMap, fs};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ^ - north
    // v - south
    // < - west
    // > - east
    // ^v / v^ - same houses
    // <> / <> - same houses
    let directions = fs::read("../input.txt")?;

    let houses_visited = count_houses_visits(directions.clone());
    let houses_visited_2 = count_houses_visits_2_santas(directions);
    println!("Houses visited by 2 santas: {}", houses_visited_2.len());
    println!("Houses visited: {}", houses_visited.len());

    Ok(())
}

fn count_houses_visits(directions: Vec<u8>) -> HashMap<(i32, i32), i32> {
    let mut visited = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    visited.insert((x, y), 1);

    for direction in directions {
        match direction {
            b'^' => y += 1,
            b'v' => y -= 1,
            b'<' => x -= 1,
            b'>' => x += 1,
            _ => continue,
        }

        *visited.entry((x, y)).or_insert(0) += 1;
    }
    visited
}

fn count_houses_visits_2_santas(directions: Vec<u8>) -> HashMap<(i32, i32), i32> {
    let mut visited = HashMap::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;

    visited.insert((0, 0), 2);

    for (i, &direction) in directions.iter().enumerate() {
        let is_santa = i % 2 == 0;

        if is_santa {
            match direction {
                b'^' => santa_y += 1,
                b'v' => santa_y -= 1,
                b'<' => santa_x -= 1,
                b'>' => santa_x += 1,
                _ => continue,
            }
            *visited.entry((santa_x, santa_y)).or_insert(0) += 1;
        } else {
            match direction {
                b'^' => robo_y += 1,
                b'v' => robo_y -= 1,
                b'<' => robo_x -= 1,
                b'>' => robo_x += 1,
                _ => continue,
            }

            *visited.entry((robo_x, robo_y)).or_insert(0) += 1;
        }
    }

    visited
}
