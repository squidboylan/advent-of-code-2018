use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut letter_freqs: HashMap<char, u64> = HashMap::new();
    let mut twos: u64 = 0;
    let mut threes: u64 = 0;

    // Get our data from stdin
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input)?;

    for i in input.split_whitespace() {
        letter_freqs.clear();
        for c in i.chars() {
            letter_freqs.entry(c).and_modify(|x| {*x += 1}).or_insert(1);
        }

        let mut two_done = false;
        let mut three_done = false;
        for (_c, val) in letter_freqs.iter() {
            if *val == 2 && !two_done {
                twos += 1;
                two_done = true;
            } else if *val == 3 && !three_done {
                threes += 1;
                three_done = true;
            }
        }
    }

    let strings: Vec<&str> = input.split_whitespace().collect();

    let mut p2_output = String::new();
    'outer: for i in &strings {
        for j in &strings {
            // If our strings are exactly the same skip them
            if j == i {
                continue;
            }
            // Calculate the difference between the strings
            let mut s1: Vec<char> = j.chars().collect();
            let mut s2: Vec<char> = i.chars().collect();
            let mut diff_chars = 0;
            let mut char_index = 0;
            for k in 0..s1.len() {
                if s1[k] != s2[k] {
                    diff_chars += 1;
                    char_index = k;
                    if diff_chars > 1 {
                        break;
                    }
                }
            }
            // if the difference between the strings is 1, remove the different char
            // then add that new string as our output string and break the outer loop
            if diff_chars == 1 {
                s1.remove(char_index);
                for k in s1 {
                    p2_output.push(k);
                }
                break 'outer;
            }
        }
    }

    println!("Part 1: {}", twos * threes);

    println!("Part 2: {}", p2_output);

    Ok(())
}
