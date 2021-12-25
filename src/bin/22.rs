use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use regex::Regex;
use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};

fn remap(uniq: &HashSet::<i32>) -> (Vec<i32>, HashMap<i32, u32>) {
  let mut v: Vec<i32> = uniq.iter().map(|val| *val).collect();
  v.sort();
  let mut map = HashMap::<i32, u32>::new();
  for (i, x) in v.iter().enumerate() {
    map.insert(*x, i as u32);
  }
  (v, map)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = File::open(&args[1]).expect("Can't open file");
  let reader = io::BufReader::new(file);

  let re = Regex::new(r"(on|off) x=(\-?\d+)\.\.(\-?\d+),y=(\-?\d+)\.\.(\-?\d+),z=(\-?\d+)\.\.(\-?\d+)").unwrap();
  let mut cuboids = Vec::<(i32, i32, i32, i32, i32, i32, bool)>::new();
  for line in reader.lines() {
    let line = line.unwrap();
    let caps = re.captures(&line).unwrap();
    let state: bool = &caps[1] == "on";
    let coords: Vec<i32> = (2..8).map(|i| caps[i].parse::<i32>().unwrap()).collect();
    cuboids.push((coords[0], coords[1]+1, coords[2], coords[3]+1, coords[4], coords[5]+1, state));
  }

  // Day 1: Compute initialization region first
  let mut initialization = [[[false; 101]; 101]; 101];
  for (x_min, x_max, y_min, y_max, z_min, z_max, state) in cuboids.iter() {
    let cap = |v| min(max(v+50, 0), 101);
    for x in cap(x_min)..cap(x_max) {
      for y in cap(y_min)..cap(y_max) {
        for z in cap(z_min)..cap(z_max) {
          initialization[x as usize][y as usize][z as usize] = *state;
        }
      }
    }
  }
  let mut n_on: usize = 0;
  for x in 0..101 {
    for y in 0..101 {
      for z in 0..101 {
        n_on += initialization[x][y][z] as usize;
      }
    }
  }
  println!("{}", n_on);

  // Day 2: Remap all x, y, z to a compressed coordinate system
  let mut uniq_xs = HashSet::<i32>::new();
  let mut uniq_ys = HashSet::<i32>::new();
  let mut uniq_zs = HashSet::<i32>::new();
  for (x_min, x_max, y_min, y_max, z_min, z_max, _state) in cuboids.iter() {
    uniq_xs.insert(*x_min);
    uniq_xs.insert(*x_max);
    uniq_ys.insert(*y_min);
    uniq_ys.insert(*y_max);
    uniq_zs.insert(*z_min);
    uniq_zs.insert(*z_max);
  }
  let (xs, x_map) = remap(&uniq_xs);
  let (ys, y_map) = remap(&uniq_ys);
  let (zs, z_map) = remap(&uniq_zs);
  println!("analyzing universe of size {} * {} * {}", xs.len(), ys.len(), zs.len());
  let mut universe = HashSet::<(u32, u32, u32)>::new();
  let mut i = 0;
  for (x_min, x_max, y_min, y_max, z_min, z_max, state) in cuboids.iter() {
    let (x_min, x_max) = (x_map[x_min], x_map[x_max]);
    let (y_min, y_max) = (y_map[y_min], y_map[y_max]);
    let (z_min, z_max) = (z_map[z_min], z_map[z_max]);
    println!("cuboid {}: {} * {} * {} (universe size {})", i, (x_max - x_min), (y_max - y_min), (z_max - z_min), universe.len());
    i += 1;
    for x in x_min..x_max {
      for y in y_min..y_max {
        for z in z_min..z_max {
	  if *state {
	    universe.insert((x, y, z));
	  } else {
            universe.remove(&(x, y, z));
          }
        }
      }
    }
  }
  println!("counting universe of size {}", universe.len());
  let mut volume_on: u64 = 0;
  for (x, y, z) in universe {
    let x_size: u64 = (xs[(x+1) as usize] - xs[x as usize]) as u64;
    let y_size: u64 = (ys[(y+1) as usize] - ys[y as usize]) as u64;
    let z_size: u64 = (zs[(z+1) as usize] - zs[z as usize]) as u64;
    volume_on += x_size * y_size * z_size;
  }
  println!("{}", volume_on);
}
