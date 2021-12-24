use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashSet;

fn simulate(enhancement: &Vec<char>, h: &HashSet::<(i32, i32)>, is_inverted: bool) -> (HashSet::<(i32, i32)>, bool) {
  let mut to_inspect = HashSet::<(i32, i32)>::new();
  for (i, j) in h.iter() {
    for di in [-1, 0, 1] {
      for dj in [-1, 0, 1] {
        to_inspect.insert((i + di, j + dj));
      }
    }
  }
  let mut h_new = HashSet::<(i32, i32)>::new();
  let new_is_inverted = if !is_inverted { enhancement[0] == '#' } else { enhancement[511] == '#' };
  for (i, j) in to_inspect {
    let mut bin: usize = 0;
    for di in [-1, 0, 1] {
      for dj in [-1, 0, 1] {
        let is_1: bool = is_inverted ^ h.contains(&(i + di, j + dj));
	bin = 2 * bin + (is_1 as usize);
      }
    }
    let new_ch: char = enhancement[bin];
    if (new_ch == '#') ^ new_is_inverted {
      h_new.insert((i, j));
    }
  }
  (h_new, new_is_inverted)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
  let i = lines.iter().position(|line| line == "").unwrap();
  let enhancement: Vec<char> = lines[0..i].join("").chars().collect();
  let mut h = HashSet::<(i32, i32)>::new();
  for (i, row) in lines[(i+1)..].iter().enumerate() {
    for (j, ch) in row.chars().enumerate() {
      if ch == '#' {
        h.insert((i as i32, j as i32));
      }
    }
  }
  let mut is_inverted = false;
  for step in 0..50 {
    let (h_new, is_inverted_new) = simulate(&enhancement, &h, is_inverted);
    h = h_new;
    is_inverted = is_inverted_new;
    println!("{} -> {} ({})", step, h.len(), is_inverted);
  }
}