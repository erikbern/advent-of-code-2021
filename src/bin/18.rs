use std::fs::File;
use std::io::{self, BufRead};
use std::env;

#[derive(Clone, Copy, PartialEq)]
enum TokenKind {
  OPEN,
  CLOSE,
  VALUE,
}

#[derive(Clone, Copy)]
struct Token {
  kind: TokenKind,
  value: i32,
}

/*
fn print(a: &Vec<Token>) {
  let mut output = Vec::<char>::new();
  for token in a.iter() {
    if token.kind == TokenKind::OPEN { output.push('['); }
    else if token.kind == TokenKind::CLOSE { output.push(']'); }
    else if token.kind == TokenKind::VALUE {
      if token.value >= 10 {
        output.push(('0' as u8 + ((token.value / 10) as u8)) as char);
      }
      output.push(('0' as u8 + ((token.value % 10) as u8)) as char);
    }
  }
  let s: String = output.into_iter().collect();
  println!("{}", s);
}
*/

fn reduce_one(a: &Vec<Token>) -> (Vec<Token>, bool) {
  let mut depth: usize = 0;
  let mut explode_close: usize = 0;
  let mut split: usize = 0;
  for (i, token) in a.iter().enumerate() {
    if token.kind == TokenKind::CLOSE && depth > 4 && explode_close == 0 { explode_close = i; }
    if token.kind == TokenKind::VALUE && token.value >= 10 && split == 0 {split = i; }
    if token.kind == TokenKind::OPEN { depth += 1; }
    else if token.kind == TokenKind::CLOSE { depth -= 1; }
  }
  if explode_close != 0 {
    // println!("exploding at {}", explode_close);
    let explode_open = explode_close - 3;
    let mut a_new = Vec::<Token>::new();
    a_new.extend(&a[0..explode_open]);
    a_new.push(Token { kind: TokenKind::VALUE, value: 0 });
    a_new.extend(&a[(explode_close+1)..]);
    
    // Find the first value-token to the left and right and add it
    for i in (0..explode_open).rev() {
      if a_new[i].kind == TokenKind::VALUE {
        a_new[i].value += a[explode_open + 1].value;
	break;
      }
    }
    for i in (explode_open+1)..a_new.len() {
      if a_new[i].kind == TokenKind::VALUE {
        a_new[i].value += a[explode_close - 1].value;
	break;
      }
    }
    (a_new, true)
  } else if split != 0 {
    // println!("splitting at {} ({})", split, a[split].value);
    let mut a_new = Vec::<Token>::new();
    a_new.extend(&a[0..split]);
    a_new.push(Token { kind: TokenKind::OPEN, value: 0 });
    a_new.push(Token { kind: TokenKind::VALUE, value: (a[split].value/2) });
    a_new.push(Token { kind: TokenKind::VALUE, value: ((a[split].value+1)/2) });
    a_new.push(Token { kind: TokenKind::CLOSE, value: 0 });
    a_new.extend(&a[(split+1)..]);
    (a_new, true)
  } else {
    (a.clone(), false)
  }
}

fn add(a: &Vec<Token>, b: &Vec<Token>) -> Vec<Token> {
  let mut combined = Vec::<Token>::new();
  combined.push(Token { kind: TokenKind::OPEN, value: 0});
  combined.extend(a.iter());
  combined.extend(b.iter());
  combined.push(Token { kind: TokenKind::CLOSE, value: 0});
  loop {
    // print(&combined);
    let (new_array, reduced) = reduce_one(&combined);
    if !reduced { break; }
    combined = new_array;
  }
  // println!("");
  combined
}

fn magnitude(a: &Vec<Token>) -> u64 {
  let mut mult: u64 = 1;
  let mut ret: u64 = 0;
  for token in a {
    if token.kind == TokenKind::OPEN { mult *= 3; }
    else if token.kind == TokenKind::CLOSE { mult /= 2; }
    else { ret += mult * (token.value as u64); mult = mult / 3 * 2; }
  }
  ret
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let mut input = Vec::<Vec<Token>>::new();
  for line in reader.lines() {
    let mut tokens = Vec::<Token>::new();
    for ch in line.unwrap().chars() {
      if ch == '[' {
        tokens.push(Token { kind: TokenKind::OPEN, value: 0 });
      } else if ch == ']' {
        tokens.push(Token { kind: TokenKind::CLOSE, value: 0 });
      } else if ch >= '0' && ch <= '9' {
        tokens.push(Token { kind: TokenKind::VALUE, value: ch as i32 - '0' as i32});
      }
    }
    input.push(tokens);
  }
  let mut sum = input[0].clone();
  for i in 1..input.len() {
    sum = add(&sum, &input[i]);
  }
  println!("{}", magnitude(&sum));

  let mut mag_max: u64 = 0;
  for term1 in input.iter() {
    for term2 in input.iter() {
      let sum = add(&term1, &term2);
      let mag = magnitude(&sum);
      if mag > mag_max { mag_max = mag; }
    }
  }
  println!("{}", mag_max);
}
