use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let mut score1: i64 = 0;
  let mut score2s = Vec::<i64>::new();
  for line in reader.lines().map(|line| line.unwrap()) {
    let mut incorrect = false;
    println!("{}", line);
    let mut q = Vec::<char>::new();
    for ch in line.chars() {
      if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
        q.push(ch);
      } else {
	let exp = match ch {
	  ')' => '(',
	  ']' => '[',
	  '}' => '{',
	  '>' => '<',
	  _ => '?',
	};
        let prev = q.pop().unwrap();
	if prev != exp {
	  score1 += match ch { ')' => 3, ']' => 57, '}' => 1197, '>' => 25137, _ => 0};
	  incorrect = true;
          break;
        }
      }
    }
    if !incorrect {
      let mut s: i64 = 0;
      while !q.is_empty() {
        let ch = q.pop().unwrap();
        s = 5 * s + match ch { '(' => 1, '[' => 2, '{' => 3, '<' => 4, _ => 0};
      }
      score2s.push(s);
    }
  }
  score2s.sort();
  let score2 = score2s[score2s.len() / 2];
  println!("{} {}", score1, score2);
}
