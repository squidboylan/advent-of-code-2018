use std::io::{self, Read};

const CLOSE_DISTANCE: i64 = 10000;

#[derive(Debug)]
struct Coord {
    loc: (isize, isize),
    area: i64,
}

impl Coord {
    pub fn new(loc: (isize, isize)) -> Self{
        Coord {
            loc,
            area: 0,
        }
    }
}

/// Coordinates are stored as tuples of isizes, the y value being first and the second
/// being x
fn main() -> io::Result<()> {
    // Get our data from stdin
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input)?;

    let mut coords: Vec<Coord> = Vec::new();

    for i in input.split("\n") {
        if i.len() > 0 {
            coords.push(Coord::new(get_coords(i)));
        }
    }

    // Get the maximum x and y coords needed
    let mut max_y = 0;
    let mut max_x = 0;
    for i in &coords {
        let (y, x) = i.loc;
        if y > max_y {
            max_y = y;
        }
        if x > max_x {
            max_x = x;
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let mut closest_point: usize = 0;
            let mut closest_point_distance = distance((y, x), coords[0].loc);
            let mut closest_is_dup = false;
            for (i, coord) in coords.iter().enumerate().skip(1) {
                let distance = distance((y, x), coord.loc);
                if distance < closest_point_distance {
                    closest_point = i;
                    closest_point_distance = distance;
                    closest_is_dup = false;
                } else if distance == closest_point_distance {
                    closest_is_dup = true;
                }
            }
            if !closest_is_dup {
                // If the area is infinite set it to -1 otherwise, increment by 1 if it's not
                // infinite already
                if x == 0 || x == max_x || y == 0 || y == max_y {
                    coords[closest_point].area = -1;
                } else if coords[closest_point].area >= 0 {
                    coords[closest_point].area += 1;
                }
            }
        }
    }

    let max_area = coords.iter().max_by_key(|v| v.area).unwrap().area;

    println!("Part 1: {}", max_area);

    let mut close_area = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            let mut total = 0;
            for coord in coords.iter() {
                total += distance((y, x), coord.loc);
                if total >= CLOSE_DISTANCE {
                    break;
                }
            }
            if total < CLOSE_DISTANCE {
                close_area += 1;
            }
        }
    }

    println!("Part 2: {}", close_area);

    Ok(())
}

fn get_coords(input: &str) -> (isize, isize) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let mut left_str = split_input[0].to_string();
    let right_str = split_input[1].to_string();
    left_str.pop();
    let left = left_str.parse::<isize>().unwrap();
    let right = right_str.parse::<isize>().unwrap();
    (left, right)
}

fn distance(coord1: (isize, isize), coord2: (isize, isize)) -> i64 {
    let (y1, x1) = coord1;
    let (y2, x2) = coord2;

    ((y2 - y1).abs() + (x2 - x1).abs()) as i64
}
