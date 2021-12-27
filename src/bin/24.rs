use std::fs::File;
use std::io::{self, BufRead};
use std::env;

#[derive(Debug)]
struct Term {
  is_var: bool,
  var: usize,
  value: i64,
}

fn parse(s: &str) -> Term {
  match s {
    "x" => Term { is_var: true, var: 0, value: 0 },
    "y" => Term { is_var: true, var: 1, value: 0 },
    "z" => Term { is_var: true, var: 2, value: 0 },
    "w" => Term { is_var: true, var: 3, value: 0 },
    _ => Term { is_var: false, var: 0, value: s.parse::<i64>().unwrap() },
  }
}

#[derive(Debug)]
struct Instruction {
  op: String,
  terms: Vec<Term>,
}

fn cap(lo: i64, hi: i64, val: i64) -> i64 {
  if val < lo { lo }
  else if val > hi { hi }
  else { val }
}

fn simulate(inputs: &Vec<u8>, instructions: &Vec<Instruction>) -> (i64, i64) {
   let mut lo = [0i64; 4];
  let mut hi = [0i64; 4];
  let mut input_count = 0;
  for ins in instructions {
    if ins.op == "inp" {
      if inputs[input_count] == 0 {
        lo[ins.terms[0].var] = 1;
        hi[ins.terms[0].var] = 9;
      } else {
        lo[ins.terms[0].var] = inputs[input_count] as i64;
        hi[ins.terms[0].var] = inputs[input_count] as i64;
      }
      input_count += 1;
    } else {
      let var = ins.terms[0].var;
      let lo_term = if ins.terms[1].is_var { lo[ins.terms[1].var] } else { ins.terms[1].value };
      let hi_term = if ins.terms[1].is_var { hi[ins.terms[1].var] } else { ins.terms[1].value };
      if ins.op == "add" {
        lo[var] += lo_term;
        hi[var] += hi_term;
      } else if ins.op == "mul" {
        let products = vec![lo[var]*lo_term, hi[var]*lo_term, lo[var]*hi_term, hi[var]*hi_term];
        lo[var] = *products.iter().min().unwrap();
        hi[var] = *products.iter().max().unwrap();
      } else if ins.op == "div" {
        // all divisions are by constants
        lo[var] /= ins.terms[1].value;
        hi[var] /= ins.terms[1].value;
      } else if ins.op == "mod" {
        // all mods are by constants
        if lo[var] == hi[var] {
          lo[var] = lo[var] % ins.terms[1].value;
          hi[var] = hi[var] % ins.terms[1].value;
        } else {
          lo[var] = cap(-ins.terms[1].value+1, 0, lo[var]);
          hi[var] = cap(0, ins.terms[1].value-1, hi[var]);
        }
      } else if ins.op == "eql" {
        if lo[var] == hi[var] && hi[var] == lo_term && lo_term == hi_term {
          lo[var] = 1;
          hi[var] = 1;
        } else if lo[var] > hi_term || hi[var] < lo_term {
          lo[var] = 0;
          hi[var] = 0;
        } else {
          lo[var] = 0;
          hi[var] = 1;
        }
      }
    }
  }
  (lo[2], hi[2])
}

fn mag(inputs: &Vec<u8>) -> u64 {
  let mut tot: u64 = 0;
  for i in 0..14 {
    tot = 10 * tot + (inputs[i] as u64);
  }
  tot
}

fn solve(inputs: &Vec<u8>, i: usize, instructions: &Vec<Instruction>, highest: bool) -> bool{
  let (lo, hi) = simulate(inputs, instructions);
  if lo > 0 || hi < 0 {
    return false;
  }
  // println!("{:?} -> ({}, {})", inputs, lo, hi);
  if i == 14 {
    println!("solution: {:?}", mag(inputs));
    return true;
  }
  let mut inputs_new = inputs.clone();
  let mut digits: Vec<usize> = (1..10).collect();
  if highest { digits.reverse(); }
  for digit in digits {
    inputs_new[i] = digit as u8;
    if solve(&inputs_new, i+1, instructions, highest) {
      return true;
    }
  }
  false
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = File::open(&args[1]).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let mut instructions = Vec::<Instruction>::new();
  for line in reader.lines() {
    let line: String = line.unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();
    instructions.push(Instruction {
      op: words[0].to_string(),
      terms: words[1..].iter().map(|word| parse(word)).collect()
    });
  }
  let inputs = vec![0u8; 14];
  solve(&inputs, 0, &instructions, true);
  solve(&inputs, 0, &instructions, false);
}
