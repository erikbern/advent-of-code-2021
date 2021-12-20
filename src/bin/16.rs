use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::cmp::{min, max};

fn read_bits(a: &[u8], n: usize) -> (usize, &[u8]) {
  let mut ret: usize = 0;
  for i in 0..n {
    ret = (ret << 1) | (a[i] as usize);
  }
  (ret, &a[n..])
}

fn read_literal(a: &[u8]) -> (usize, &[u8]) {
  let mut l: usize = 0;
  let mut a_p = &a[..];
  loop {
    let (q, a_new) = read_bits(a_p, 5);
    a_p = a_new;
    l = (l << 4) | (q & 0xf);
    if (q >> 4) & 1 == 0 {
      break;
    }
  }
  (l, a_p)
}

fn read_operator(a: &[u8], type_id: usize) -> (usize, usize, &[u8]) {
  let (lti, a) = read_bits(a, 1);
  let mut version_sum: usize = 0;
  let mut a_p: &[u8];
  let mut values = Vec::<usize>::new();
  if lti == 0 {
    let (tl, a) = read_bits(a, 15);
    let mut a_subpackets = &a[..tl];
    while a_subpackets.len() > 0 {
      let (v_sum, value, a_new) = read_packet(a_subpackets);
      a_subpackets = a_new;
      version_sum += v_sum;
      values.push(value);
    }
    a_p = &a[tl..];
  } else {
    let (n_subpackets, a_new) = read_bits(a, 11);
    a_p = a_new;
    for _i in 0..n_subpackets {
      let (v_sum, value, a_new) = read_packet(a_p);
      a_p = a_new;
      version_sum += v_sum;
      values.push(value);
    }
  }
  let value = match type_id {
    0 => values.iter().fold(0, |a, b| a + b),
    1 => values.iter().fold(1, |a, b| a * b),
    2 => values.iter().fold(usize::MAX, |a, b| min(a, *b)),
    3 => values.iter().fold(0, |a, b| max(a, *b)),
    5 => (values[0] > values[1]) as usize,
    6 => (values[0] < values[1]) as usize,
    7 => (values[0] == values[1]) as usize,
    _ => 999999999,
  };
  println!("    values = {:?} -> {}", values, value);
  (version_sum, value, a_p)
}

fn read_packet(a: &[u8]) -> (usize, usize,  &[u8]) {
  let (v, a) = read_bits(a, 3);
  let (t, a) = read_bits(a, 3);
  if t == 4 {
    let (value, a) = read_literal(a);
    (v, value, a)
  } else {
    let (version_sum, value, a) = read_operator(a, t);
    (v + version_sum, value, a)
  }
}

fn handle(line: &String) {
  let mut a = Vec::<u8>::new();
  println!("{}", line);
  for ch in line.chars() {
    let m: u8 = u8::from_str_radix(&ch.to_string(), 16).unwrap();
    for i in 0..4 {
      a.push((m >> (3-i)) & 1);
    }
  }
  println!("    {:?}", a);
  let (version_sum, value, _) = read_packet(&a[..]);
  println!("    version sum = {} value = {}", version_sum, value);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  for line in reader.lines() {
    handle(&line.unwrap());
  }
}