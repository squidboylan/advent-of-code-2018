use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
struct Guard {
    total_sleep: u32,
    sleep_map: Vec<u32>,
}

enum Action {
    SLEEP,
    WAKE,
    WORK,
}

impl Guard {
    pub fn new() -> Self {
        let sleep_map = vec![0; 60];
        Guard {
            total_sleep: 0,
            sleep_map,
        }
    }

    pub fn update(&mut self, start_minute: usize, end_minute: usize) {
        let mut i = start_minute;
        while i < end_minute {
            self.sleep_map[i] += 1;
            self.total_sleep += 1;
            i += 1;
            if i == 60 {
                i = 0;
            }
        }

    }
}

fn main() -> io::Result<()> {
    let mut guards: HashMap<u32, Guard> = HashMap::new();

    // Get our data from stdin
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input)?;

    // Sort our input for processing
    let mut input_sorted: Vec<&str> = input.split("\n").collect();
    input_sorted.sort();

    let mut id: u32 = 0;
    let mut sleep_start_minute: usize = 0;
    for i in input_sorted {
        let i_split: Vec<&str> = i.split_whitespace().collect();
        if i_split.len() == 0 {
            continue;
        }
        let minute = get_minute(&i_split);
        let action = get_action(&i_split);

        match action {
            Action::WORK => id = get_id(&i_split),
            Action::SLEEP => sleep_start_minute = minute,
            Action::WAKE => {
                let guard = guards.entry(id).or_insert(Guard::new());
                guard.update(sleep_start_minute, minute);
            }
        };
    }

    let mut most_sleep = 0;
    let mut most_sleep_id = 0;
    let mut most_sleep_minute = 0;
    for (k, v) in guards.iter() {
        if v.total_sleep > most_sleep {
            most_sleep = v.total_sleep;
            most_sleep_id = *k;
            let mut tmp = 0;
            let mut minute = 0;
            for (e, i) in v.sleep_map.iter().enumerate() {
                if *i > tmp {
                    minute = e;
                    tmp = *i;
                }
            }
            most_sleep_minute = minute;
        }
    }

    println!("Part 1: {}", most_sleep_id as usize * most_sleep_minute);

    let mut most_sleep_id = 0;
    let mut most_sleep_minute = 0;
    for (k, v) in guards.iter() {
        let mut tmp = 0;
        let mut minute = 0;
        for (e, i) in v.sleep_map.iter().enumerate() {
            if *i > tmp {
                minute = e;
                tmp = *i;
            }
        }
        if minute > most_sleep_minute {
            most_sleep_minute = minute;
            most_sleep_id = *k;
        }
    }

    println!("Part 2: {}", most_sleep_id as usize * most_sleep_minute);

    Ok(())
}

fn get_minute(input: &[&str]) -> usize {
    let minute_str: Vec<&str> = input[1].split(":").collect();

    let mut minute_str = minute_str[1].to_string();
    minute_str.pop();
    minute_str.parse::<usize>().unwrap()
}

fn get_action(input: &[&str]) -> Action {
    if input[2] == "Guard" && input[4] == "begins" && input[5] == "shift" {
        Action::WORK
    } else if input[2] == "falls" && input[3] == "asleep" {
        Action::SLEEP
    } else if input[2] == "wakes" && input[3] == "up" {
        Action::WAKE
    } else {
        panic!("Bad input for getting action")
    }
}

fn get_id(input: &[&str]) -> u32 {
    let mut id_str = input[3].to_string();
    id_str.remove(0);
    id_str.parse::<u32>().unwrap()
}
