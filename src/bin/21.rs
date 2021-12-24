use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
  let mut spaces: Vec<usize> = lines.iter().map(|line| line.split_whitespace().last().unwrap().parse::<usize>().unwrap()).collect();
  let mut n_die_rolls = 0;
  let mut scores = vec![0, 0];
  let mut player = 0;
  loop {
    for _i in 0..3 {
      n_die_rolls += 1;
      let die_next = 1 + ((n_die_rolls - 1) % 100);
      spaces[player] = (spaces[player] + die_next - 1) % 10 + 1;
    }
    scores[player] += spaces[player];
    println!("scores: {:?} spaces: {:?}", scores, spaces);
    if scores[player] >= 1000 {
      break;
    }
    player = (player + 1) % 2;
  };
  println!("{} {} -> {}", n_die_rolls, scores[1-player], n_die_rolls * scores[1-player]);
}
  