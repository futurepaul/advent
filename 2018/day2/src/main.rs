use std::collections::HashMap;
use std::io::{self, Read};
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
                continue;
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
                }
                4 => local_triples -= 1,
                // note: default match case is '_ =>'
                _ => continue,
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

//if string
// returns position
fn string_compare(string1: &str, string2: &str) -> Option<usize> {
    let string1_chars: Vec<char> = string1.chars().collect();
    let string2_chars: Vec<char> = string2.chars().collect();

    if string1_chars.len() != string2_chars.len() {
        return None;
    }

    let mut diff = 0;
    let mut diff_index = 0;

    for (i, ch) in string1_chars.iter().enumerate() {
        let ch2 = &string2_chars[i];
        if ch != ch2 {
            diff += 1;

            // this is just a shortcut to make it faster
            if diff > 1 {
                return None;
            }

            diff_index = i;
        }
    }

    match diff {
        1 => Some(diff_index), //one difference is okay
        _ => None,
    }
}

fn part2(input: &str) -> Result<()> {
    let mut lines: Vec<&str> = input.lines().collect();

    //I have to reverse the strings because it didn't work when the strings weren't reversed
    //your input might be different, in which case you don't need to reverse
    //TODO: implement this in a less hacky way
    let mut lines_as_strings: Vec<String> = lines
        .iter()
        .map(|x| x.to_string().chars().rev().collect())
        .collect();

    lines_as_strings.sort();

    for (line_index, line1) in lines_as_strings.iter().enumerate() {
        if line_index == 0 {
            continue;
        }
 //conmpare current line to every line
        match string_compare(line1, &lines_as_strings[line_index - 1]) {
            Some(char_index) => {
                println!(
                    "line_a: {}, line_b: {}, index: {}",
                    line1,
                    &lines_as_strings[line_index - 1],
                    char_index
                );
                let (first_half, second_half) = line1.split_at(char_index); // split the string at the found char

                let answer: String = [first_half, &second_half[1..]]
                    .join("")
                    .chars()
                    .rev()
                    .collect(); // reconcatonate them leaving out the target char
                println!("answer: {}", answer);
            }

            None => continue,
        };
    }

    Ok(())
}
