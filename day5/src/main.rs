use std::io::{self, Read};

const ASCII: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() -> io::Result<()> {
    // Get our data from stdin
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input)?;

    if input.ends_with("\n") {
        input.pop();
    }

    let mut input_chars: Vec<char> = input.chars().collect();

    let reacted_polymer = react_polymer(input_chars);

    println!("Part 1: {}", reacted_polymer.len());

    let mut shortest_reacted = std::usize::MAX;
    for i in ASCII.iter() {
        let mut input_chars: Vec<char> = input.chars().collect();

        // Remove all units of the current type i
        input_chars.retain(|x| *x != *i && *x as u8 - *i as u8 != 32 && *i as u8 - *x as u8 != 32);

        let reacted_polymer = react_polymer(input_chars);
        if reacted_polymer.len() < shortest_reacted {
            shortest_reacted = reacted_polymer.len();
        }
    }

    println!("Part 2: {}", shortest_reacted);

    Ok(())
}

fn react_polymer(mut input_chars: Vec<char>) -> Vec<char> {
    let mut deleted = true;
    while deleted {
        deleted = false;
        let mut tmp = Vec::with_capacity(input_chars.len());
        let mut i = 0;
        while i <= input_chars.len() - 1 && input_chars.len() > 0 {
            let left = input_chars[i];
            if i == input_chars.len() - 1 {
                tmp.push(left);
                break;
            }
            let right = input_chars[i + 1];
            // Check if left and right are the same char of different case
            if left as u8 - right as u8 != 32 && right as u8 - left as u8 != 32 {
                tmp.push(left);
                i += 1;
            } else {
                deleted = true;
                i += 2;
            }
        }
        input_chars = tmp;
    }
    input_chars
}
