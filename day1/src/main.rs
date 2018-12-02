use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut freqs: HashSet<i64> = HashSet::new();
    let mut freq: i64 = 0;
    let mut dup: Option<i64> = None;

    // Get our data from stdin
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input)?;

    for i in input.split_whitespace() {
        let mag = i.parse::<i64>().unwrap();
        freq += mag;
        if freqs.contains(&freq) && dup == None {
            dup = Some(freq);
        }
        freqs.insert(freq);
    }

    println!("Part 1: {}", freq);

    while dup == None {
        for i in input.split_whitespace() {
            let mag = i.parse::<i64>().unwrap();
            freq += mag;
            if freqs.contains(&freq) && dup == None {
                dup = Some(freq);
            }
            freqs.insert(freq);
        }
    }


    match dup {
        Some(i) => println!("Part 2: {}", i),
        None => println!("Part 2: None"),
    }

    Ok(())
}
