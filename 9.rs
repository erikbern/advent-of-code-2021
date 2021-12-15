use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn s2row(s: &String) -> Vec<i32> {
  s.chars().map(|s| s as i32 - '0' as i32).collect()
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let grid: Vec<Vec<i32>> = reader.lines().map(|line| s2row(&line.unwrap())).collect();
  let w: i32 = grid[0].len() as i32;
  let h: i32 = grid.len() as i32;
  let dirs = [(1, 0), (0, 1), (-1i32, 0), (0, -1i32)];
  let mut risk_sum = 0;
  for y in 0..h {
    for x in 0..w {
      if dirs.iter()
        .map(|(dx, dy)| (x + *dx as i32, y + *dy as i32))
	.filter(|(x2, y2)| 0 <= *x2 && *x2 < w && 0 <= *y2 && *y2 < h)
	.all(|(x2, y2)| grid[y as usize][x as usize] < grid[y2 as usize][x2 as usize]) {
	 risk_sum += grid[y as usize][x as usize] + 1;
      }
    }
  }
  println!("{}", risk_sum);
}