use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashSet;


#[derive(Debug, PartialEq)]
struct State {
  w: i32,
  h: i32,
  e: Vec<(i32, i32)>,
  s: Vec<(i32, i32)>,
}

fn get_hash(e: &Vec<(i32, i32)>, s: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
  let mut h = HashSet::<(i32, i32)>::new();
  for tup in e.iter() { h.insert(*tup); }
  for tup in s.iter() { h.insert(*tup); }
  h
}

fn shift(v: &Vec<(i32, i32)>, blocked: &HashSet<(i32, i32)>, di: i32, dj: i32, h: i32, w: i32) -> Vec<(i32, i32)> {
  let mut u: Vec<(i32, i32)> = v.iter().map(|(i, j)| {
    let i2 = (i + di) % h;
    let j2 = (j + dj) % w;
    // println!("{} {} -> {} {} ??? {}", i, j, i2, j2, blocked.contains(&(i2, j2)));
    if blocked.contains(&(i2, j2)) { (*i, *j) } else { (i2, j2) }
  }).collect();
  u.sort();
  u
}

fn step(state: &State) -> State {
  let blocked = get_hash(&state.s, &state.e);
  let e_new = shift(&state.e, &blocked, 0, 1, state.h, state.w);
  let blocked = get_hash(&state.s, &e_new);
  let s_new = shift(&state.s, &blocked, 1, 0, state.h, state.w);  
  State { e: e_new, s: s_new, w: state.w, h: state.h }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = File::open(&args[1]).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let mut state = State { e: vec![], s: vec![], h: 0, w: 0 };
  for (i, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    for (j, ch) in line.chars().enumerate() {
      if ch == '>' { state.e.push((i as i32, j as i32)); }
      if ch == 'v' { state.s.push((i as i32, j as i32)); }
    }
    state.h = (i + 1) as i32;
    state.w = line.len() as i32;
  }
  let mut ct = 1;
  loop {
    if ct % 1000 == 0 { println!("{}", ct); }
    // println!("{}, {:?}", ct, state);
    let new_state = step(&state);
    if new_state == state { break; }
    state = new_state;
    ct += 1;
  }
  println!("{}", ct);
}