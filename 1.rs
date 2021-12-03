use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();
   let filename = &args[1];
   println!("{}", filename);
   let file = File::open(filename).expect("Can't open file");
   let reader = io::BufReader::new(file);
   let inputs: Vec<i32> = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
   let mut ct1: i32 = 0;
   for i in 1..inputs.len() {
     if inputs[i] > inputs[i-1] {
       ct1 += 1;
     }
  }
  let mut ct3: i32 = 0;
  for i in 3..inputs.len() {
     if inputs[i] > inputs[i-3] {  // inputs[i-1] and inputs[i-2] cancel out
       ct3 += 1;
     }
  }
  println!("{} {}", ct1, ct3);
}
