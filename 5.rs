use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn parse_coord(s: &str) -> (i32, i32) {
  let items: Vec<i32> = s.split(",").map(|item| item.parse::<i32>().unwrap()).collect();
  (items[0], items[1])
}

fn draw(grid: &mut HashMap::<(i32, i32), i32>, x1: i32, y1: i32, x2: i32, y2: i32) {
  // println!("{} {} -> {} {}", x1, y1, x2, y2);
  let steps = std::cmp::max((x2 - x1).abs(), (y2 - y1).abs());
  let dx = (x2 - x1) / steps;
  let dy = (y2 - y1) / steps;
  for i in 0..steps+1 {
    let coord = (x1 + dx * i, y1 + dy * i);
    let c: i32 = *grid.get(&coord).unwrap_or(&0);
    grid.insert(coord, c + 1);
  }      
}

fn count_intersections(grid: &HashMap::<(i32, i32), i32>) -> usize {
  grid.into_iter().map(|(&_k, &v)| v).filter(|&v| v > 1).count()
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);

  let mut grid1 = HashMap::<(i32, i32), i32>::new();
  let mut grid2 = HashMap::<(i32, i32), i32>::new();

  for line in reader.lines() {
    let line = line.unwrap();
    let mut split = line.split(" -> ");
    let (src, dst) = (split.next().unwrap(), split.next().unwrap());
    let (x1, y1) = parse_coord(src);
    let (x2, y2) = parse_coord(dst);
    if x1 == x2 || y1 == y2 {
      draw(&mut grid1, x1, y1, x2, y2);
    }
    draw(&mut grid2, x1, y1, x2, y2);
  }
  println!("{} {}", count_intersections(&grid1), count_intersections(&grid2));
}