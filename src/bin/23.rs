use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use itertools::Itertools;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct XY {
  x: i32,
  y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
  xys: [[XY; 4]; 4],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
  pos: Pos,
  ener: i32,
  heur: i32,
}

fn print(pos: &Pos) {
  let mut mat = [['.'; 11]; 5];
  for i in 0..4 {
    for j in 0..4 {
      let xy = pos.xys[i][j];
      mat[xy.y as usize][xy.x as usize] = (i as u8 + 'A' as u8) as char;
    }
  }
  for r in 0..5 {
    let row: String = String::from_iter(mat[r]);
    println!("{}", row);
  }
}

fn dist1(a: &XY, b: &XY) -> i32 {
  if a.x == b.x {
    (a.y - b.y).abs()
  } else {
    a.y + (a.x - b.x).abs() + b.y
  }
}

fn heuristic(pos: &Pos) -> i32 {
  let mut total: i32 = 0;
  for i in 0..4 {
    let dists: Vec<i32> = (0..4).permutations(4).map(|perm| {
      let mut sum: i32 = 0;
      for j in 0..4 {
        let pos = pos.xys[i as usize][j];
        let goal = XY { x: 2*(i+1), y: perm[j]+1 };
        sum += dist1(&pos, &goal);
      }
      sum
    }).collect();
    total += 10_i32.pow(i as u32) * dists.iter().min().unwrap();
  }
  total
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.ener + other.heur).cmp(&(self.ener + self.heur))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn make_move(state: &State, i: usize, j: usize, x: i32, y: i32) -> State {
  let mut new_pos = state.pos.clone();
  new_pos.xys[i][j] = XY { x: x, y: y};
  let d: i32 = dist1(&state.pos.xys[i][j], &new_pos.xys[i][j]);
  new_pos.xys[i].sort();
  State {
    pos: new_pos,
    ener: state.ener + 10_i32.pow(i as u32) * d,
    heur: heuristic(&new_pos),
  }
}

fn make_moves(state: &State) -> Vec<State> {
  let pos = &state.pos;
  let mut new_states = Vec::<State>::new();
  let mut h = HashMap::<XY, usize>::new();
  for i in 0..4 {
    for j in 0..4 {
      h.insert(pos.xys[i][j], i);
    }
  }
  for i in 0..4 {
    for j in 0..4 {
      if pos.xys[i][j].y > 0 {
        // has to move from a room to the hallway
        // Make sure all positions are empty above it
        let x = pos.xys[i][j].x as i32;
        let y = pos.xys[i][j].y as i32;
        if (0..y).any(|y| h.contains_key(&(XY { x: x, y: y}))) { continue; }
        for x2 in pos.xys[i][j].x..11 {  // move up and to the right
          let new_xy =  XY { x: x2 as i32, y: 0 };
          if h.contains_key(&new_xy) { break; }
          if x2 == 2 || x2 == 4 || x2 == 6 || x2 == 8 { continue; }
          new_states.push(make_move(state, i, j, x2, 0));
        }
        for x2 in (0..(pos.xys[i][j].x+1)).rev() {
          let new_xy =  XY { x: x2 as i32, y: 0 };
          if h.contains_key(&new_xy) { break; }
          if x2 == pos.xys[i][j].x { continue; }
          if x2 == 2 || x2 == 4 || x2 == 6 || x2 == 8 { continue; }
            new_states.push(make_move(state, i, j, x2, 0));
        }
      } else {
        // has to move from the hallway into a room
	let x = pos.xys[i][j].x as i32;
        let x2 = 2*(i+1) as i32;

        // Make sure the hallway is clear
	if x2 < x {
          if (x2..x).any(|x| h.contains_key(&(XY { x: x, y: 0}))) { continue; }
	} else {
          if ((x+1)..(x2+1)).any(|x| h.contains_key(&(XY { x: x, y: 0}))) { continue; }
        }

	// Find the largest (furthest down) y that's empty
	let mut invalid = false;
	let mut found_y = None;
	for y2 in (1..5).rev() {
          let xy = XY { x: x2, y: y2 };
	  if h.contains_key(&xy) {
            if h[&xy] != i { invalid = true; }
	    else if found_y.is_some() { invalid = true; }	    
          } else if found_y.is_none() {
            found_y = Some(y2) ;
          }
        }
	if found_y.is_some() && !invalid {
          new_states.push(make_move(state, i, j, x2, found_y.unwrap()));
        }
      }
    }
  }
  new_states
}

fn solve(starting_pos: &Pos) {
  // Create priority queue of states
  let mut heap = BinaryHeap::<State>::new();
  let starting_state = State{ pos: *starting_pos, ener: 0, heur: heuristic(starting_pos) };
  heap.push(starting_state);
  let mut lowest_energy = HashMap::<Pos, i32>::new();
  let mut from = HashMap::<State, State>::new();
  lowest_energy.insert(*starting_pos, 0);
  let mut best_state = starting_state.clone();

  // Do A* search
  while !heap.is_empty() {
    let state: State = heap.pop().unwrap();
    if state.ener > lowest_energy[&state.pos] { continue; }
    if state.heur < best_state.heur || (state.heur == best_state.heur && state.ener < best_state.ener) {
      println!(" found new best: {}, {}", state.heur, state.ener);
      best_state = state.clone();
    }
    for new_state in make_moves(&state).iter() {
      if lowest_energy.contains_key(&new_state.pos) {
        if new_state.ener >= lowest_energy[&new_state.pos] {
	  continue;
	}
      }
      lowest_energy.insert(new_state.pos, new_state.ener);
      from.insert(*new_state, state);
      heap.push(*new_state);
    }
  }
  println!("best state: {:?}", best_state);
  let mut solution = Vec::<State>::new();
  let mut state: &State = &best_state;
  while from.contains_key(state) {
    solution.push(*state);
    state = from.get(&mut state).unwrap();
  }
  for i in (0..solution.len()).rev() {
    println!("{} {}", solution[i].ener, solution[i].heur);
    print(&solution[i].pos);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = File::open(&args[1]).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

  // Construct initial pos
  let mut ct = [0; 4];
  let xys = [[XY { x: 0, y: 0 }; 4]; 4];
  let mut pos = Pos { xys: xys };
  for (y, row) in lines.iter().enumerate() {
    for (x, ch) in row.chars().enumerate() {
      if ch >= 'A' && ch <= 'D' {
        let i = ch as usize - 'A' as usize;
        pos.xys[i][ct[i]] = XY { x: x as i32 - 1, y: y as i32 - 1 };
        ct[i] += 1;
      }
    }
  }

  // Day 1: add extra ones, just simple
  for i in 0..4 {
    for j in 2..4 {
      pos.xys[i][j] = XY { x: 2*(i+1) as i32, y: (j+1) as i32 };
    }
  }
  solve(&pos);

  // Day 2: unfold it
  for i in 0..4 {
    for j in 0..2 {
      if pos.xys[i][j].y == 2 { pos.xys[i][j].y = 4; }
    }
  }

  pos.xys[0][2] = XY { x: 8, y: 2 };
  pos.xys[0][3] = XY { x: 6, y: 3 };

  pos.xys[1][2] = XY { x: 6, y: 2 };
  pos.xys[1][3] = XY { x: 4, y: 3 };

  pos.xys[2][2] = XY { x: 4, y: 2 };
  pos.xys[2][3] = XY { x: 8, y: 3 };

  pos.xys[3][2] = XY { x: 2, y: 2 };
  pos.xys[3][3] = XY { x: 2, y: 3 };

  for i in 0..4 {
    pos.xys[i].sort();
  }
  solve(&pos);
}
