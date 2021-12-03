use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let mut x: i32 = 0;
  let mut y: i32 = 0;
  let mut y2: i32 = 0;
  let mut aim: i32 = 0;
  for line in reader.lines() {
    let line = line.unwrap();
    let items = line.split(" ").collect::<Vec<&str>>();
    let dir = items[0];
    let mag = items[1].parse::<i32>().unwrap();
    match dir {
      "forward" => { x += mag; y2 += aim * mag; },
      "down" => { y += mag; aim += mag; },
      "up" => { y -= mag; aim -= mag; },
      _ => println!("Weird direction {}", dir),
    }
  }
  println!("x = {}, y = {}, x * y = {}", x, y, x * y);
  println!("x = {}, y2 = {}, x * y2 = {}", x, y2, x * y2);
}
