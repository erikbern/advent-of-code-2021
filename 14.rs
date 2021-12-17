use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashMap;

fn get_0 (h: &HashMap<(char, char), i64>, tup: &(char, char)) -> i64 {
  if !h.contains_key(&tup) { 0 } else { *h.get(&tup).unwrap() }
}

fn simulate(ct: &HashMap<(char, char), i64>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), i64> {
  let mut ct2 = HashMap::<(char, char), i64>::new();
  for (tup, c) in &*ct {
    if !rules.contains_key(tup) {
      let c_sum: i64 = get_0(&ct2, tup);
      ct2.insert(*tup, c_sum + c);
    } else {
      let mid: char = *rules.get(tup).unwrap();
      let (ch1, ch2) = tup;
      let tup1: (char, char) = (*ch1, mid);
      let tup2: (char, char) = (mid, *ch2);
      let c_sum1: i64 = get_0(&ct2, &tup1);
      ct2.insert(tup1, c_sum1 + c);
      let c_sum2: i64 = get_0(&ct2, &tup2);
      ct2.insert(tup2, c_sum2 + c);
    }
  }
  ct2
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
  let polymer = &lines[0];

  let mut ct = HashMap::<(char, char), i64>::new();
  let chars: Vec<char> = polymer.chars().collect();
  for i in 0..chars.len() {
    let tup = (chars[i], if i + 1 < chars.len() { chars[i+1] } else { '_' });
    let c: i64 = get_0(&ct, &tup);
    ct.insert(tup, c+1);
  }
  println!("{:?}", ct);

  let mut rules = HashMap::<(char, char), char>::new();
  for rule in lines[2..lines.len()].iter() {
    let chars: Vec<char> = rule.chars().collect();
    rules.insert((chars[0], chars[1]), chars[6]);
  }
  println!("{:?}", rules);

  for step in 0..40 {
    ct = simulate(&ct, &rules);

    if step == 9 || step == 39 {
      let mut ct_by_char = HashMap::<char, i64>::new();
      for (tup, c) in &ct {
        let (ch1, _) = tup;
        let c_sum: i64 = if ct_by_char.contains_key(&ch1) { *ct_by_char.get(&ch1).unwrap() } else { 0 };
        ct_by_char.insert(*ch1, c_sum + c);
      }
      println!("{} -> {:?}", step, ct_by_char);

      let mut cts = Vec::<i64>::new();
      for (_, c) in ct_by_char {
        cts.push(c);
      }
      println!("{} -> {:?}", step, cts);

      cts.sort();
      println!("{} -> {}", step, cts[cts.len()-1] - cts[0]);
    }
  }
}