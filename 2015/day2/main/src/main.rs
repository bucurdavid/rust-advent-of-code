use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // area of the box is 2 * l * w + 2 * w * h + 2 * h * l
    // input is in the form of l x w x h
    //
    let contents = fs::read_to_string("../input.txt")?;

    let mut total_area = 0;
    let mut total_ribbon = 0;
    for line in contents.lines() {
        let [l, w, h] = line
            .split('x')
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>()[..3]
        else {
            continue;
        };

        total_ribbon += calculate_ribbon_length(&mut [l, w, h]);
        total_area += calculate_area(l, w, h);
    }
    println!("Total area: {}", total_area);
    println!("Total ribbon: {}", total_ribbon);
    Ok(())
}

fn calculate_area(l: i32, w: i32, h: i32) -> i32 {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;
    let smallest_side = lw.min(wh).min(hl);

    2 * lw + 2 * wh + 2 * hl + smallest_side
}
fn calculate_ribbon_length(dimensions: &mut [i32; 3]) -> i32 {
    dimensions.sort();
    let smallest_perimter = 2 * dimensions[0] + 2 * dimensions[1];
    smallest_perimter + dimensions.iter().product::<i32>()
}
