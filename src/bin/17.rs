extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use regex::Regex;
use std::cmp::max;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let input = reader.lines().next().unwrap().unwrap();
  let re = Regex::new(r"target area: x=(\-?\d+)..(\-?\d+), y=(\-?\d+)..(\-?\d+)").unwrap();
  let caps = re.captures(&input).unwrap();
  let caps: Vec<i32> = (1..5).map(|i| caps[i].parse::<i32>().unwrap()).collect();
  let (xl, xh, yl, yh) = (caps[0], caps[1], caps[2], caps[3]);
  println!("{} {} {} {}", xl, xh, yl, yh);
  let mut y_max_all: i32 = 0;
  let mut hit_count: usize = 0;
  for dx in 0..(xh+1) {
    for dy in -1000..1000 {
       let mut dx = dx;
       let mut dy = dy;
       let mut x: i32 = 0;
       let mut y: i32 = 0;
       let mut y_max: i32 = 0;
       loop {
         if x > xh || y < yl {
           break
         }
 	 y_max = max(y_max, y);
	 if x >= xl && x <= xh && y >= yl && y <= yh {
	   y_max_all = max(y_max_all, y_max);
	   hit_count += 1;
	   break;
         }
	 x += dx;
	 y += dy;
	 dx = max(dx - 1, 0);
         dy -= 1;
       }
    }
  }
  println!("{} {}", y_max_all, hit_count);
}
