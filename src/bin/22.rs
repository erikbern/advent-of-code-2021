use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use regex::Regex;
use std::cmp::{min, max};

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = File::open(&args[1]).expect("Can't open file");
  let reader = io::BufReader::new(file);

  let mut universe = [[[false; 101]; 101]; 101];
  let re = Regex::new(r"(on|off) x=(\-?\d+)\.\.(\-?\d+),y=(\-?\d+)\.\.(\-?\d+),z=(\-?\d+)\.\.(\-?\d+)").unwrap();
  for line in reader.lines() {
    let line = line.unwrap();
    let caps = re.captures(&line).unwrap();
    let state: bool = &caps[1] == "on";
    let coords: Vec<i32> = (2..8).map(|i| caps[i].parse::<i32>().unwrap()).collect();
    let cap = |v| min(max(v+50, 0), 101);
    let parse = |lo, hi| (cap(lo) as usize, cap(hi+1) as usize);
    let (x_min, x_max) = parse(coords[0], coords[1]);
    let (y_min, y_max) = parse(coords[2], coords[3]);
    let (z_min, z_max) = parse(coords[4], coords[5]);
    println!("{} {}..{} {}..{} {}..{}", state, x_min, x_max, y_min, y_max, z_min, z_max);
    for x in x_min..x_max {
      for y in y_min..y_max {
        for z in z_min..z_max {
          universe[x][y][z] = state;
        }
      }
    }
  }
  let mut n_on: usize = 0;
  for x in 0..101 {
    for y in 0..101 {
      for z in 0..101 {
        n_on += universe[x][y][z] as usize;
      }
    }
  }
  println!("{}", n_on);
}
