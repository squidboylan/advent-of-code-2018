use std::io::{self, Read};

#[derive(Default, Clone, Copy, Debug)]
struct Entry {
    id: usize,
    overlapped: bool,
}

fn main() -> io::Result<()> {
    let mut overlapping = 0;
    let mut matrix: Vec<Vec<Entry>> = Vec::with_capacity(1000);
    let mut id_overlapped: Vec<bool> = Vec::with_capacity(2048);

    for i in 0..1000 {
        matrix.push(Vec::with_capacity(1000));
        for _j in 0..1000 {
            matrix[i].push(Entry::default());
        }
    }


    // Get our data from stdin
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input)?;

    for i in input.split("\n") {
        if i == "" {
            break;
        }
        // Process current line
        let mut temp: Vec<String> = i.split_whitespace().map(|x| x.to_string()).collect();
        id_overlapped.push(false);
        let id = process_id(&mut temp[0]);

        let (y, x) = process_coords(&mut temp[2]);
        let (h, w) = process_dims(&mut temp[3]);
        for a in x..x+w {
            for b in y..y+h {
                if matrix[b][a].id == 0 {
                    matrix[b][a].id = id;
                } else if matrix[b][a].id != 0 {
                    if matrix[b][a].overlapped == false {
                        overlapping += 1;
                        id_overlapped[matrix[b][a].id - 1] = true;
                        id_overlapped[id - 1] = true;
                        matrix[b][a].overlapped = true;
                    } else {
                        id_overlapped[id - 1] = true;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", overlapping);

    for (index, val) in id_overlapped.iter().enumerate() {
        if *val == false {
            println!("Part 2: {}", index + 1);
            break;
        }
    }

    Ok(())
}

fn process_id(input: &mut String) -> usize {
    input.remove(0);
    input.parse::<usize>().unwrap()
}

fn process_coords(input: &mut String) -> (usize, usize) {
    input.pop();
    let split_input: Vec<&str> = input.split(",").collect();
    let x = split_input[1].parse::<usize>().unwrap();
    let y = split_input[0].parse::<usize>().unwrap();

    (y, x)
}

fn process_dims(input: &mut String) -> (usize, usize) {
    let split_input: Vec<&str> = input.split("x").collect();
    let w = split_input[1].parse::<usize>().unwrap();
    let h = split_input[0].parse::<usize>().unwrap();

    (h, w)
}
