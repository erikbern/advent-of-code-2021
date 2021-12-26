use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::cmp::{min, max};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct XY {
  x: i32,
  y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
  xys: [[XY; 2]; 4],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
  pos: Pos,
  ener: i32,
  heur: i32,
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
    let goal_1 = XY { x: 2*(i+1), y: 1 };
    let goal_2 = XY { x: 2*(i+1), y: 2 };
    let pos_1: &XY = &pos.xys[i as usize][0];
    let pos_2: &XY = &pos.xys[i as usize][1];
    total += 10_i32.pow(i as u32) * min(
      dist1(pos_1, &goal_1) + dist1(pos_2, &goal_2),
      dist1(pos_1, &goal_2) + dist1(pos_2, &goal_1),
    );
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
    for j in 0..2 {
      h.insert(pos.xys[i][j], i);
    }
  }
  for i in 0..4 {
    for j in 0..2 {
      if pos.xys[i][j].y > 0 {
        // has to move from a room to the hallway
        // If it's at y=2 then make sure y=1 isn't taken (doesn't make sense to move to y=1?)
        let up_xy = XY { x: pos.xys[i][j].x, y: 1};
        if pos.xys[i][j].y == 2 && h.contains_key(&up_xy) { continue; }
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
          if (x..x2).any(|x| h.contains_key(&(XY { x: x, y: 0}))) { continue; }
	} else {
          if ((x+1)..(x2+1)).any(|x| h.contains_key(&(XY { x: x, y: 0}))) { continue; }
        }
        let down_1_xy = XY { x: x2, y: 1};
        let down_2_xy = XY { x: x2, y: 2};
        if h.contains_key(&down_1_xy) { continue };
        if !h.contains_key(&down_2_xy) {
          // If y=2 is empty then always go there
          new_states.push(make_move(state, i, j, x2, 2));
        } else {
          let i_cur: usize = *h.get(&down_2_xy).unwrap();
          if i_cur == i {
            new_states.push(make_move(state, i, j, x2, 1));
          }
        }
      }
    }
  }
  new_states
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = File::open(&args[1]).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

  // Construct initial pos
  let mut ct = [0; 4];
  let xys = [[XY { x: 0, y: 0 }; 2]; 4];
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

  // Create priority queue of states
  let mut heap = BinaryHeap::<State>::new();
  heap.push(State{ pos: pos, ener: 0, heur: heuristic(&pos) });
  let mut lowest_energy = HashMap::<Pos, i32>::new();
  lowest_energy.insert(pos, 0);
  let mut best_heur = heuristic(&pos);
  let mut best_ener = 0;
  //let mut ct = 0;

  // Do A* search
  while !heap.is_empty() {
    let state: State = heap.pop().unwrap();
    /*ct += 1;
    if ct % 100000 == 0 {
      println!("{}: {}+{}", ct, state.ener, state.heur);
    }*/
    if state.ener > lowest_energy[&state.pos] { continue; }
    if state.heur < best_heur {
      best_heur = state.heur;
      println!("{} + {}: {:?}", state.ener, state.heur, state.pos);
      best_ener = state.ener;
    } else if state.heur == best_heur && state.ener < best_ener {
      best_ener = state.ener;
    }
    for new_state in make_moves(&state).iter() {
      if lowest_energy.contains_key(&new_state.pos) {
        if new_state.ener >= lowest_energy[&new_state.pos] {
	  continue;
	}
      }
      lowest_energy.insert(new_state.pos, new_state.ener);
      heap.push(*new_state);
    }
  }
  println!("best solution: {}", best_ener);
}
