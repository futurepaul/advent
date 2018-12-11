use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug)]
enum EventType {
    ShiftStart(u32),
    FallAsleep(u32),
    WakeUp(u32),
}

impl EventType {
    fn value(&self) -> u32 {
        match self {
            EventType::ShiftStart(value) => *value,
            EventType::FallAsleep(value) => *value,
            EventType::WakeUp(value) => *value,
        }
    }
}

type GuardSleepEvents = Vec<EventType>;

fn main() -> Result<()> {
    //Read the input
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;
    let mut lines: Vec<&str> = input.lines().collect();

    //Sort by time
    lines.sort();

    // for line in lines.clone() {
    //   println!("{}", line);
    // }

    let mut events: Vec<EventType> = Vec::new();

    let mut guard_log = HashMap::new();

    //Parse into events
    let mut current_guard = 0;

    for event_string in lines {
        let log_item: Vec<&str> = event_string.split(" ").collect();

        if log_item[3].starts_with("#") {
            let guard_id: u32 = log_item[3]
                .trim_start_matches("#")
                .parse()
                .expect("parse failed");

            // let event_type = EventType::ShiftStart;

            if !guard_log.contains_key(&guard_id) {
                match guard_log.insert(guard_id, Vec::new()) {
                    None => println!("The key wasn't present"),
                    Some(value) => println!("The key was already there uh oh"),
                };
            }

            current_guard = guard_id;

            continue;
        }

        //sleep events
        if log_item[3].starts_with("a") {
            let time: u32 = log_item[1]
                .trim_start_matches("00:")
                .trim_end_matches("]")
                .parse()
                .unwrap();

            if let Some(log) = guard_log.get_mut(&current_guard) {
                log.push(EventType::FallAsleep(time));
            }

            continue;
        }

        //wake events
        if log_item[3].starts_with("u") {
            let time: u32 = log_item[1]
                .trim_start_matches("00:")
                .trim_end_matches("]")
                .parse()
                .unwrap();

            if let Some(log) = guard_log.get_mut(&current_guard) {
                log.push(EventType::WakeUp(time));
            }

            continue;
        }
    }

    // println!("{:?}", guard_log);

    //Find the guard who slept most
    let mut sleeps: Vec<(u32, u32)> = Vec::new();

    for guard in guard_log.keys() {
        let sleepy_time: u32 = guard_log
            .get(&guard)
            .expect("didn't find the guard")
            .chunks(2)
            .fold(0, |acc, chunk| acc + (chunk[1].value() - chunk[0].value()));
        sleeps.push((*guard, sleepy_time));
    }

    let (winner_id, winner_slept_for) = sleeps.iter().max_by_key(|(guard, sleep)| sleep).unwrap();

    println!(
        "winner is: {}, slept for {} minutes",
        winner_id, winner_slept_for
    );

    //Find single minute the guard sleeps most often
    let mut hour: [u32; 60] = [0; 60];

    for chunk in guard_log.get(winner_id).unwrap().chunks(2) {
        for minute in chunk[0].value()..chunk[1].value() {
            hour[minute as usize] += 1;
        }
    }

    let mut minute_tuples = Vec::new();

    for (i, minute) in hour.iter().enumerate() {
        println!("minute {}: {}", i, minute);
        minute_tuples.push((i, minute));
    }

    let (most_minute, minutes) = minute_tuples
        .iter()
        .max_by_key(|(which_minute, minutes)| minutes)
        .unwrap();

    println!("the answer is: {}", ((*winner_id as usize) * *most_minute));

    Ok(())
}
