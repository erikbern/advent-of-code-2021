use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn process(items: Vec<String>, flip: bool, pos: usize) -> String {
  if items.len() == 1 {
    println!("returning");
    items[0].clone()
  } else {
    let ct1 = items.iter().filter(|item| item.as_bytes()[pos] == b'1').count();
    let ge1 = 2 * ct1 >= items.len();
    let bit = if flip ^ ge1 { b'0'} else { b'1' };
    let items_filtered : Vec<String> = items.into_iter().filter(|item| item.as_bytes()[pos] == bit).collect::<Vec<_>>();
    assert!(items_filtered.len() >= 1);
    process(items_filtered, flip, pos+1)
  }
}

fn bin2int(s: String) -> i32 {
  let mut num: i32 = 0;
  let c = s.as_bytes();
  for i in 0..c.len() {
    num |= ((c[c.len() - i - 1] == b'1') as i32) << i;
  }
  num
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

  let oxygen_gen = process(lines.clone(), true, 0);
  let co2_gen = process(lines.clone(), false, 0);
  let oxygen_gen_int = bin2int(oxygen_gen);
  let co2_gen_int = bin2int(co2_gen);
  println!("oxygen = {}, co2 = {}, product = {}", oxygen_gen_int, co2_gen_int, oxygen_gen_int * co2_gen_int);
}
