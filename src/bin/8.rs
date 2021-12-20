use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;


fn generate_permutations(perm: Vec<char>, pool: HashSet<char>, output: &mut Vec<String>) {
  if pool.len() == 0 {
    let p: String = perm.into_iter().collect();
    output.push(p);
  } else {
    for item in &pool {
      let mut perm2 = perm.clone();
      let mut pool2 = pool.clone();
      pool2.remove(item);
      perm2.push(*item);
      generate_permutations(perm2, pool2, output);
    }
  }
}

fn get_map(perm: &str) -> HashMap<char, char> {
  let abc: &str = "abcdefg";
  let mut map = HashMap::<char, char>::new();
  for (p, q) in perm.chars().zip(abc.chars()) {
    map.insert(p, q);
  }
  map
}

fn remap(word: &str, perm: &HashMap<char, char>) -> String {
  word.chars().map(|c| perm[&c]).collect()
}

fn sort_string(word: &str) -> String {
  let mut chars: Vec<char> = word.chars().collect();
  chars.sort_by(|a, b| a.cmp(b));
  String::from_iter(chars)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);

  // 0..9 segments
  let mut seg2i = HashMap::<String, usize>::new();
  for (i, seg) in ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"].iter().enumerate() {
    seg2i.insert(seg.to_string(), i);
  }

  // Generate all 7! permutations and store in a vector
  let perm = Vec::<char>::new();
  let pool = HashSet::<char>::from_iter("abcdefg".chars());
  let mut perms = Vec::<String>::new();
  generate_permutations(perm, pool, &mut perms);
  let maps: Vec<HashMap<char, char>> = perms.iter().map(|perm| get_map(perm)).collect();

  // Read input and process
  let mut count_1: usize = 0;
  let mut code_sum: usize = 0;
  for line in reader.lines() {
    let line = line.unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(words.len(), 10 + 1 + 4);
    count_1 += words[11..15].iter().filter(|word| word.len() == 7 || word.len() == 4 || word.len() == 3 || word.len() == 2).count();

    // Try every permutation of a..g and see if it recreates
    for map in &maps {
      // println!("{:?}", map);
      let digits: Vec<String> = words[0..10].iter().map(|word| remap(word, &map)).map(|word| sort_string(&word)).collect();
      if digits.iter().all(|word| seg2i.contains_key(word)) {
        println!("{:?}", digits);
        let mut code: usize = 0;
        for word in words[11..15].iter() {
          let word = remap(word, &map);
	  let word = sort_string(&word);
	  let digit: usize = seg2i[&sort_string(&word)];
	  code = code * 10 + digit;
	}
	code_sum += code;
	continue;
      }
    }
  }
  println!("{} {}", count_1, code_sum);
}
