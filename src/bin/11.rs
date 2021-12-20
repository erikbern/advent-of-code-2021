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
  let mut grid: Vec<Vec<i32>> = reader.lines().map(|line| s2row(&line.unwrap())).collect();
  let w: i32 = grid[0].len() as i32;
  let h: i32 = grid.len() as i32;
  let mut total_n_flashes: i32 = 0;
  let mut step: i32 = 0;
  loop {
    if step % 100 == 0 {
      println!("{}...", step);
      println!("{:?}", grid);
    }
    let mut grid2: Vec<Vec<i32>> = grid.iter().map(|row| row.clone()).collect();
    let mut q = Vec::<(i32, i32)>::new();
    for y in 0..h {
      for x in 0..w {
        grid2[y as usize][x as usize] += 1;
        if grid2[y as usize][x as usize] == 10 {
          q.push((x, y));
        }
      }
    }
    let mut n_flashes: i32 = 0;
    while !q.is_empty() {
      let (x, y) = q.pop().unwrap();
      n_flashes += 1;
      for dx in -1..2 {
        for dy in -1..2 {
          let x2 = x + dx;
          let y2 = y + dy;
          if (x2 != x || y2 != y) && x2 >= 0 && y2 >= 0 && x2 < w && y2 < h {
            grid2[y2 as usize][x2 as usize] += 1;
            if grid2[y2 as usize][x2 as usize] == 10 {
              q.push((x2, y2));
            }
          }
        }
      }
    }
    for y in 0..h {
      for x in 0..w {
        if grid2[y as usize][x as usize] > 9 {
          grid2[y as usize][x as usize] = 0;
        }
      }
    }
    grid = grid2;
    step += 1;
    if step < 100 {
      total_n_flashes += n_flashes;
    }
    if n_flashes == w * h {
      break;
    }
  }
  println!("{} {}", total_n_flashes, step);
}