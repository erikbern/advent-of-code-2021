use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn s2row(s: &String) -> Vec<i32> {
  s.chars().map(|s| s as i32 - '0' as i32).collect()
}

fn steps(x: i32, y: i32, w: i32, h: i32) -> Vec<(i32, i32)> {
  let dirs = [(1, 0), (0, 1), (-1i32, 0), (0, -1i32)];
  dirs.iter()
   .map(|(dx, dy)| (x + *dx as i32, y + *dy as i32))
   .filter(|(x2, y2)| 0 <= *x2 && *x2 < w && 0 <= *y2 && *y2 < h)
   .collect()
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let grid: Vec<Vec<i32>> = reader.lines().map(|line| s2row(&line.unwrap())).collect();
  let w: i32 = grid[0].len() as i32;
  let h: i32 = grid.len() as i32;
  let mut risk_sum = 0;
  let mut basin: Vec<Vec<bool>> = (0..h).map(|_| vec![false; w as usize]).collect();
  let mut basin_sizes = Vec::<usize>::new();
  for y in 0..h {
    for x in 0..w {
      if steps(x, y, w, h).iter().all(|(x2, y2)| grid[y as usize][x as usize] < grid[*y2 as usize][*x2 as usize]) {
	 risk_sum += grid[y as usize][x as usize] + 1;

	 // Do flood-fill using BFS
	 let mut basin_size = 0;
	 let mut q = vec![(x, y)];
	 while !q.is_empty() {
	   let (qx, qy) = q.pop().unwrap();
	   basin_size += 1;
	   for (qx2, qy2) in steps(qx, qy, w, h) {
	     if grid[qy as usize][qx as usize] < grid[qy2 as usize][qx2 as usize]
               && grid[qy2 as usize][qx2 as usize] < 9
	       && !&basin[qy2 as usize][qx2 as usize] {
	        basin[qy2 as usize][qx2 as usize] = true;
	        q.push((qx2, qy2));
              }
	    }
	 }
	 basin_sizes.push(basin_size);
      }
    }
  }
  println!("{}", risk_sum);
  basin_sizes.sort();
  basin_sizes.reverse();
  println!("{:?} {}", basin_sizes, basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}