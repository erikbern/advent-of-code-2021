use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn process(items: &Vec<String>, bit: i32) {
  for i in 0..items[0].len()-1 {
    println!("{}", i);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
  let mut ct = vec![0; lines[0].len()];
  for line in &lines {
      for i in line.chars().enumerate().filter(|&(_, c)| c == '1').map(|(i, _)| i) {
        ct[i] += 1;
     }
  }
  ct.reverse();
  let mut gamma: i32 = 0;
  let mut epsilon: i32 = 0;
  for (i, c) in ct.iter().enumerate() {
    if 2 * c > lines.len() {
      gamma |= 1 << i;
    } else if 2 * c < lines.len() {
      epsilon |= 1 << i;
    } else if 2 * c == lines.len() {
      println!("Tie in position {}", i);
    }
  }
  println!("gamma = {} epsilon = {} gamma * epsilon = {}", gamma, epsilon, gamma * epsilon);

  let oxygen_gen = process(&lines, 0);
  let co2_gen = process(&lines, 1);
}
