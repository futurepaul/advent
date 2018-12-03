use std::io::{self, Read};
use std::collections::HashMap;
type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {

  let mut input = String::new();



  io::stdin().read_to_string(&mut input)?;

  part1(&input);
  part2(&input);

  Ok(())
}

fn part1(input: &str) {
  // for the entire file
  let mut doubles = 0;
  let mut triples = 0;

  for line in input.lines() {
    // for this line only
    let mut local_doubles = 0;
    let mut local_triples = 0;

    let mut line_chars: Vec<char> = line.chars().collect();
    line_chars.sort();

    // println!("{:?}", line_chars);

    // notes for paul's eventual return:
    // not entirely sure why we need a &char here
    let mut first_seen_index: HashMap<&char, usize> = HashMap::new();

    // note: enumerate is (i, item) vs. (item, i)
    for (i, ch) in line_chars.iter().enumerate() {
      // println!("i: {}, char: {}", ch, i)

      if !first_seen_index.contains_key(&ch) {
        first_seen_index.insert(ch, i);
        continue
      }

      let first_index = first_seen_index.get(&ch).unwrap();
      let instances = i - first_index + 1;

      //first time: insert and continue
      //second time: increment "doubles" and continue
      //third time: increment "triples", decrement "doubles" and continue
      //fourth time: decrement triples
      //fifth + : continue
      match instances {
        2 => local_doubles += 1,
        3 => {
            local_doubles -= 1;
            local_triples += 1;
        },
        4 => local_triples -= 1,
        // note: default match case is '_ =>'
        _ => continue
      }
      
    }

    // if we have local double add to doubles count
    if local_doubles >= 1 {
        doubles += 1;
    }

    // if we have local triple add to triples count
    if local_triples >= 1 {
        triples += 1;
    }
  }

  println!("checksum: {}", doubles * triples);
}

fn part2(input: &str) {

  // vec
  // abc () <--
  // abd (adc) <--
  // cep (adb, adc) <--

  // abcz
  // addf
  // addg
  // cep
  // zbcz

  // 
}