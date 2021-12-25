use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashMap;
use std::cmp::max;


fn simulate(spaces: &Vec<usize>) {
  let mut spaces: Vec<usize> = vec![spaces[0], spaces[1]];
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
    // println!("scores: {:?} spaces: {:?}", scores, spaces);
    if scores[player] >= 1000 {
      break;
    }
    player = (player + 1) % 2;
  };
  println!("{} {} -> {}", n_die_rolls, scores[1-player], n_die_rolls * scores[1-player]);
}

fn compute(n: &mut HashMap<(i32, i32, i32, i32), (u64, u64)>, pos_1: i32, pos_2: i32, sc_1: i32, sc_2: i32) -> (u64, u64) {
  if sc_1 >= 21 {
    (1, 0)
  } else if sc_2 >= 21 {
    (0, 1)
  } else if n.contains_key(&(pos_1, pos_2, sc_1, sc_2)) {
    *n.get(&(pos_1, pos_2, sc_1, sc_2)).unwrap()
  } else {
    let mut wins_1_sum: u64 = 0;
    let mut wins_2_sum: u64 = 0;
    for die_roll_1 in 1..4 {
      for die_roll_2 in 1..4 {
        for die_roll_3 in 1..4 {
          let new_pos_1 = (pos_1 + die_roll_1 + die_roll_2 + die_roll_3 - 1) % 10 + 1;
          let new_sc_1 = sc_1 + new_pos_1;
          let (wins_2_term, wins_1_term) = compute(n, pos_2, new_pos_1, sc_2, new_sc_1);
          wins_1_sum += wins_1_term;
          wins_2_sum += wins_2_term;
	}
      }
    }
    n.insert((pos_1, pos_2, sc_1, sc_2), (wins_1_sum, wins_2_sum));
    // println!("{} {} {} {} -> {} {}", pos_1, pos_2, sc_1, sc_2, wins_1_sum, wins_2_sum);
    (wins_1_sum, wins_2_sum)
  }
}

fn dirac(spaces: &Vec<usize>) {
  let mut n = HashMap::<(i32, i32, i32, i32), (u64, u64)>::new();
  let (wins_1, wins_2) = compute(&mut n, spaces[0] as i32, spaces[1] as i32, 0, 0);
  println!("{} {} -> {}", wins_1, wins_2, max(wins_1, wins_2));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
  let spaces: Vec<usize> = lines.iter().map(|line| line.split_whitespace().last().unwrap().parse::<usize>().unwrap()).collect();
  simulate(&spaces);
  dirac(&spaces);
}
  