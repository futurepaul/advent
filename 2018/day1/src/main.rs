use std::collections::HashSet;
use std::fs;

fn main() {
  let mut frequencies = HashSet::new();
  let mut frequency: i32 = 0;
  let input_file = fs::read_to_string("input.txt").unwrap();
  let mut first_time_through = true;
  'outer: loop {
    for line in input_file.lines() {
      let delta: i32 = match line.trim().parse() {
        Ok(parsed) => parsed,
        Err(_) => {
          println!("Didn't get a number, just gonna keep going");
          continue;
        }
      };
      frequency += delta;

      //twice will be false if it's already in the set
      let twice = frequencies.insert(frequency);
      if !twice {
        println!("the thing we got twice is: {}", frequency);
        break 'outer;
      }
    }
    if first_time_through {
      println!("frequency: {}", frequency);
    }
    first_time_through = false;
  }
}
