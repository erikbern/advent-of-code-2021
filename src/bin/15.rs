use std::fs::File;
use std::io::{self, BufRead};
use std::env;


fn s2row(s: &String) -> Vec<i32> {
  s.chars().map(|s| s as i32 - '0' as i32).collect()
}

struct Coord {
  x: i32,
  y: i32,
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let grid0: Vec<Vec<i32>> = reader.lines().map(|line| s2row(&line.unwrap())).collect();
  let h0: i32 = grid0.len() as i32;
  let w0: i32 = grid0[0].len() as i32;
  println!("{:?}", grid0);

  // Blow it up by 5x
  let h = h0 * 5;
  let w = w0 * 5;
  let mut grid = Vec::<Vec<i32>>::new();
  for y in 0..h {
    grid.push(vec![999999999; w as usize]);
    for x in 0..w {
      let orig_cost: i32 = grid0[(y % h0) as usize][(x % h0) as usize];
      let extra: i32 = x / w0 + y / h0;
      grid[y as usize][x as usize] = (orig_cost + extra - 1) % 9 + 1;
    }
  }
  println!("{:?}", grid);

  // Now, do a simplified version of Dijkstra
  let mut q = Vec::<Vec<Coord>>::new();
  q.push(Vec::<Coord>::new());
  q[0].push(Coord{ x: 0, y: 0});

  let mut best = Vec::<Vec<usize>>::new();
  for _y in 0..(h as usize) {
    best.push(vec![999999999; w as usize]);
  }
  println!("{:?}", best);

  let mut cost: usize = 0;
  while cost < q.len() {
    while !q[cost].is_empty() {
      let c: Coord = q[cost].pop().unwrap();
      println!("{}: {},{}", cost, c.x, c.y);
      let mut check = |c2: Coord| {
	if c2.x >= 0 && c2.y >= 0 && c2.x < w && c2.y < h {
          let new_cost: usize = cost + grid[c2.y as usize][c2.x as usize] as usize;
          if new_cost < best[c2.y as usize][c2.x as usize] {
            best[c2.y as usize][c2.x as usize] = new_cost;
            while q.len() <= new_cost {
              q.push(Vec::<Coord>::new());
            }
            q[new_cost].push(c2);
	  }
        }
      };
      check(Coord{ x: c.x + 1, y: c.y     });
      check(Coord{ x: c.x,     y: c.y + 1 });
      check(Coord{ x: c.x - 1, y: c.y     });
      check(Coord{ x: c.x,     y: c.y - 1 });
    }
    cost += 1;
  }
  println!("{}", best[(h - 1) as usize][(w - 1) as usize]);
}
