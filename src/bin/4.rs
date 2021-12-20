use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let mut lines = reader.lines();
  let numbers: Vec<i32> = lines.next().unwrap().unwrap().split(',').map(|s| s.parse().unwrap()).collect();

  // For each number, track when it was called (todo: dupes)
  let mut ts = HashMap::<i32, i32>::new();
  for i in 0..numbers.len() {
    ts.insert(numbers[i], i as i32);
  }

  let mut t_min_all: i32 = 999999;
  let mut min_answer: i32 = 0;
  let mut t_max_all: i32 = 0;
  let mut max_answer: i32 = 0;

  loop {
    // Read matrix and build up a matrix that stores the time t when the number was called
    let empty_line = lines.next();
    if empty_line.is_none() {
      break;
    }
    let mut matrix = [[0i32; 5]; 5];
    let mut t_matrix = [[0i32; 5]; 5];
    for i in 0..5 {
      let line = lines.next().unwrap().unwrap();
      let row: Vec<i32>  = line.split_whitespace().map(|s| s.trim().parse().unwrap()).collect();
      for j in 0..5 {
        matrix[i][j] = row[j];
        t_matrix[i][j] = if ts.contains_key(&row[j]) { *ts.get(&row[j]).unwrap() } else { 999999 };
      }
    }

    // Scan rows to find the first
    let mut t_min = 999999;
    for i in 0..5 {
      let mut t_max = 0;
      for j in 0..5 {
        t_max = std::cmp::max(t_max, t_matrix[i][j]);
      }
      t_min = std::cmp::min(t_min, t_max);
    }

    // Scan cols to find the first
    for j in 0..5 {
      let mut t_max = 0;
      for i in 0..5 {
        t_max = std::cmp::max(t_max, t_matrix[i][j]);
      }
      t_min = std::cmp::min(t_min, t_max);
    }

    // Find called number
    let called_number: i32 = numbers[t_min as usize];

    // Add up unmarked
    let mut sum_unmarked: i32 = 0;
    for i in 0..5 {
      for j in 0..5 {
        sum_unmarked += if t_matrix[i][j] <= t_min { 0 } else { matrix[i][j] }
      }
    }
    println!("{} {} {}", t_min, called_number, sum_unmarked);

    // Check against all other boards (so far)
    if t_min < t_min_all {
      t_min_all = t_min;
      min_answer = called_number * sum_unmarked;
    }
    if t_min > t_max_all {
      t_max_all = t_min;
      max_answer = called_number * sum_unmarked;
    }

  }
  println!("first: {} {}", t_min_all, min_answer);
  println!("last:  {} {}", t_max_all, max_answer);
}
