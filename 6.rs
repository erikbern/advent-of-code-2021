use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let line: String = reader.lines().next().unwrap().unwrap();

  let mut ct: [i64; 9] = [0; 9];

  for age in line.split(',').map(|s| s.parse::<usize>().unwrap()) {
    ct[age] += 1;
  }

  for i in 0..256 {
    let mut ct_next: [i64; 9] = [0; 9];
    for j in 0..9 {
      if j == 0 {
        ct_next[6] += ct[j];
        ct_next[8] += ct[j];
      } else {
        ct_next[j-1] += ct[j];
      }
    }
    ct = ct_next;

    if i == 79 || i == 255 {
     println!("{}", ct.iter().sum::<i64>());
    }
  }
}
