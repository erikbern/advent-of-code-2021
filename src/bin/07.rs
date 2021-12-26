use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let line: String = reader.lines().next().unwrap().unwrap();
  let mut xs: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

  // We're going to use the fact that the median minimizes the total absolute distance
  xs.sort();
  let median: i32 = xs[xs.len() / 2];
  let total_dist_to_median: i32 = xs.iter().map(|x| (x - median).abs()).sum();

  // For part 2, let's just brute force (it should be very close to the mean since it minimizes the total square)
  let xmin: i32 = *xs.iter().min().unwrap();
  let xmax: i32 = *xs.iter().max().unwrap();
  let sum_n = |n: i32| n * (n + 1) / 2;  // sum of 1 .. n
  let total_dist = |x: i32| xs.iter().map(|x2: &i32| sum_n((x2 - x).abs()) ).sum();
  let total_dist_to_other: i32 = (xmin .. xmax + 1).map(total_dist).min().unwrap();

  println!("{} {}", total_dist_to_median, total_dist_to_other);
}