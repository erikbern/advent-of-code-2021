use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashSet;
use std::cmp;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);

  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
  let empty_line_index  = lines.iter().position(|line| line == "").unwrap();

  let mut folds = Vec::<(char, i32)>::new();
  for line in lines[(empty_line_index+1) .. lines.len()].iter() {
    let mut items = line.split("=");
    let axis: char = items.next().unwrap().chars().last().unwrap();
    let value: i32 = items.next().unwrap().parse::<i32>().unwrap();
    folds.push((axis, value));
  }

  let mut dots_first = HashSet::<(i32, i32)>::new();
  let mut dots_final = HashSet::<(i32, i32)>::new();
  let mut x_max: i32 = 0;
  let mut y_max: i32 = 0;
  for line in lines[0 .. empty_line_index].iter() {
    let coords: Vec<i32> = line.split(",").map(|item| item.parse::<i32>().unwrap()).collect();
    let mut x: i32 = coords[0];
    let mut y: i32 = coords[1];
    let mut first = true;
    for (axis, value) in folds.iter() {
      if *axis == 'x' && x > *value {
        x = 2 * *value - x
      }
      if *axis == 'y' && y > *value {
        y = 2 * *value - y
      }
      if first {
        dots_first.insert((x, y));
      }
      first = false;
    }
    dots_final.insert((x, y));
    x_max = cmp::max(x_max, x);
    y_max = cmp::max(y_max, y);
  }
  println!("{}", dots_first.len());
  println!("{} {}", x_max, y_max);
  for y in 0..(y_max+1) {
    let line: String = (0..(x_max+1)).map(|x| if dots_final.contains(&(x, y)) { '#' } else {'.'}).collect();
    println!("{}", line);
  }
}
