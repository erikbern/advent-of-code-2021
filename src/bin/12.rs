use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::{HashMap, HashSet};

struct Vertex {
  edges: Vec<String>,
}

fn add (vertices: &mut HashMap::<String, Vertex>, a: &str, b: &str) {
  if !vertices.contains_key(a) {
    vertices.insert(a.to_string(), Vertex { edges: Vec::<String>::new() });
  }
  let v = vertices.get_mut(&a.to_string()).unwrap();
  v.edges.push(b.to_string());
}

fn lowercase(s: &String) -> bool {
  let ch = s.chars().next().unwrap();
  ch >= 'a' && ch <= 'z'
}

fn traverse(vertices: &HashMap::<String, Vertex>, a: &str, visited: &mut HashSet<String>, visited_twice: bool) -> i64 {
  if a == "end" {
    return 1;
  }
  let mut count: i64 = 0;
  let v = vertices.get(a).unwrap();
  for b in v.edges.iter() {
    if b == "start" {
      continue;
    }
    if !lowercase(b) {
      count += traverse(vertices, b, visited, visited_twice);
    } else if !visited.contains(b) {
      visited.insert(b.to_string());
      count += traverse(vertices, b, visited, visited_twice);
      visited.remove(&b.to_string());
    } else if !visited_twice {
      count += traverse(vertices, b, visited, true);
    }
  }
  count
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let mut vertices = HashMap::<String, Vertex>::new();
  
  for line in reader.lines() {
    let line = line.unwrap();
    let vec: Vec<&str> = line.split("-").collect();
    add(&mut vertices, vec[0], vec[1]);
    add(&mut vertices, vec[1], vec[0]);
  }

  let mut visited = HashSet::<String>::new();
  let count = traverse(&vertices, "start", &mut visited, true);
  println!("{}", count);

  let count = traverse(&vertices, "start", &mut visited, false);
  println!("{}", count);
}