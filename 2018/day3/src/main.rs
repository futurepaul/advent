use std::io::{self, Read};
type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug)]
struct Rect {
  id: i32,
  tl_x: i32,
  tl_y: i32,
  br_x: i32,
  br_y: i32,
}

fn main() -> Result<()> {
  let mut input = String::new();

  io::stdin().read_to_string(&mut input)?;

  part1(&input);
  part2(&input);

  Ok(())
}

fn part1(input: &String) {
  let mut fabric = [[0; 1100]; 1100];
  let rects = parse_rects(input);
  for rect in rects {
    for x in rect.tl_x..rect.br_x {
      for y in rect.tl_y..rect.br_y {
        fabric[x as usize][y as usize] += 1;
      }
    }
  }

  let mut overlapping: i32 = 0;

  for row in fabric.iter() {
    for column in row.iter() {
      if *column > 1 {
        overlapping += 1;
      }
    }
  }

  println!("overlapping: {}", overlapping);
}

fn part2(input: &String) {
  let mut fabric = [[0; 1100]; 1100];
  let rects = parse_rects(input);
  for rect in &rects {
    for x in rect.tl_x..rect.br_x {
      for y in rect.tl_y..rect.br_y {
        fabric[x as usize][y as usize] += 1;
      }
    }
  }

  let mut id: i32 = 0;

  'outer: for rect in &rects {
    for x in rect.tl_x..rect.br_x {
      for y in rect.tl_y..rect.br_y {
        if fabric[x as usize][y as usize] != 1 {
          continue 'outer;
        }
      }
    }
    id = rect.id;
  }
  println!("id: {}", id);
}

fn parse_rects(rects: &String) -> Vec<Rect> {
  let rects = rects.lines();
  let mut rectangles: Vec<Rect> = Vec::new();

  for rect in rects {
    let split_rect: Vec<&str> = rect.split_whitespace().collect();

    let id = split_rect[0].trim_start_matches("#").parse().unwrap();

    let split_coords = split_rect[2]
      .trim_end_matches(":")
      .split(",")
      .collect::<Vec<&str>>();

    let tl_x: i32 = split_coords[0].parse().unwrap();
    let tl_y: i32 = split_coords[1].parse().unwrap();

    let split_dimensions = split_rect[3].split("x").collect::<Vec<&str>>();

    let width: i32 = split_dimensions[0].parse().unwrap();
    let height: i32 = split_dimensions[1].parse().unwrap();

    rectangles.push(Rect {
      id: id,
      tl_x: tl_x,
      tl_y: tl_y,
      br_x: tl_x + width,
      br_y: tl_y + height,
    });
  }

  rectangles
}
