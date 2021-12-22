use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use regex::Regex;
use std::collections::HashMap;

struct Beacon {
  coords: [i32; 3],
  id: usize,
}

struct Scanner {
  beacons: Vec<Beacon>,
}

fn det(n: &[[i32; 3]; 3]) -> i32 {
  let mut s: i32 = 0;
  for i in 0..3 {
    let (j, k) = ((i+1)%3, (i+2)%3);
    s += n[0][i] * (n[1][j]*n[2][k] - n[1][k]*n[2][j]);
  }
  s
}

fn mult(n: &[[i32; 3]; 3], x: &[i32; 3]) -> [i32; 3] {
  let mut y = [0i32; 3];
  for i in 0..3 {
    for j in 0..3 {
      y[i] += n[i][j] * x[j];
    }
  }
  y
}

fn sub(a: &[i32; 3], b: &[i32; 3]) -> [i32; 3] {
  let mut c = [0i32; 3];
  for i in 0..3 {
    c[i] = a[i] - b[i];
  }
  c
}


struct DisjointSet {
  n_groups: usize,
  group_sizes: Vec<usize>,
  representatives: Vec<usize>,
}

impl DisjointSet {
  fn new(n_groups: usize) -> Self {
    DisjointSet {
      n_groups: n_groups,
      group_sizes: vec![1; n_groups],
      representatives: (0..n_groups).collect(),
    }
  }

  fn representative(&self, member: usize) -> usize {
    if self.representatives[member] == member {
      member
    } else {
      self.representative(self.representatives[member])
    }
  }

  fn merge(&mut self, a: usize, b: usize) {
    let repr_a = self.representative(a);
    let repr_b = self.representative(b);
    if repr_a == repr_b {
       // nothing to do
    } else if self.group_sizes[repr_a] >= self.group_sizes[repr_b] {
       self.representatives[repr_b] = repr_a;
       self.group_sizes[repr_a] += self.group_sizes[repr_b];
       self.n_groups -= 1;
    } else {
       self.representatives[repr_a] = repr_b;
       self.group_sizes[repr_b] += self.group_sizes[repr_a];
       self.n_groups -= 1;
    }
  }

  fn len(&self) -> usize {
    self.n_groups
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).expect("Can't open file");
  let reader = io::BufReader::new(file);
  let scanner_re = Regex::new(r"--- scanner (\d+) ---").unwrap();
  let mut scanner_i: usize = 0;
  let mut scanners = Vec::<Scanner>::new();
  let mut n_beacons: usize = 0;
  for line in reader.lines() {
    let line = line.unwrap();
    if scanner_re.is_match(&line) {
      scanner_i = scanner_re.captures(&line).unwrap()[1].parse::<usize>().unwrap();
      while scanners.len() <= scanner_i {
        scanners.push(Scanner{ beacons: Vec::<Beacon>::new() });
      }
    } else if line == "" {
      continue;
    } else {
      let items: Vec<&str> = line.split(",").collect();
      let coords: Vec<i32> = items.iter().map(|item| item.parse::<i32>().unwrap()).collect();
      let beacon = Beacon { coords: [coords[0], coords[1], coords[2]], id: n_beacons };
      scanners[scanner_i].beacons.push(beacon);
      n_beacons += 1;
    }
  }
  let mut m = [[[0i32; 3]; 3]; 24];

  // Generate all rotation matrices
  let mut i: usize = 0;
  for axis_1 in 0..3 {
    for axis_1_sign_bool in 0..2 {
      for axis_2_offset in 1..3 {
        for axis_2_sign_bool in 0..2 {
          for axis_3_sign_bool in 0.. 2 {
            let axis_1_sign = 2 * axis_1_sign_bool - 1;
            let axis_2_sign = 2 * axis_2_sign_bool - 1;
            let axis_3_sign = 2 * axis_3_sign_bool - 1;
            let axis_2: usize = (axis_1 + axis_2_offset) % 3;
            let axis_3: usize = 3 - axis_1 - axis_2;

            let mut n = [[0i32; 3]; 3];	  
            n[0][axis_1] = axis_1_sign;
            n[1][axis_2] = axis_2_sign;
            n[2][axis_3] = axis_3_sign;

            // Probably not necessary, but we compute the determinant of the
            // rotation matrix to make sure it doesn't have negative sign
            // There are 48 possible axis-aligned rotation matrices,
            // but we only want the 24 that preserve the determinant sign.
            if det(&n) == 1 {
              m[i] = n;
              i += 1;
            }
          }
        }
      }
    }
  }

  // Create a disjoint set used to merge beacons
  let mut disjoint_set = DisjointSet::new(n_beacons);

  // Go through every pair of scanners
  for i in 0..scanners.len() {
    for j in 0..scanners.len() {
      if i == j { continue; }

      for k in 0..24 {
        for a_i in 0..scanners[i].beacons.len() {
          for a_j in 0..scanners[j].beacons.len() {
            let offset: [i32; 3] = sub(&mult(&m[k], &scanners[i].beacons[a_i].coords), &scanners[j].beacons[a_j].coords);

            // Create a hash map of scanner i's beacons
            let mut h = HashMap::<[i32; 3], usize>::new();
            for b_i in 0..scanners[i].beacons.len() {
              let proj: [i32; 3] = sub(&mult(&m[k], &scanners[i].beacons[b_i].coords), &offset);
              h.insert(proj, scanners[i].beacons[b_i].id);
            }

            // Look up scanner j's beacon
            let mut n_matches: usize = 0;
            for b_j in 0..scanners[j].beacons.len() {
              if h.contains_key(&scanners[j].beacons[b_j].coords) {
                n_matches += 1;
              }
            }
            if n_matches >= 12 {
              for b_j in 0..scanners[j].beacons.len() {
                if h.contains_key(&scanners[j].beacons[b_j].coords) {
		  let b_id = scanners[j].beacons[b_j].id;
		  let a_id = *h.get(&scanners[j].beacons[b_j].coords).unwrap();
		  disjoint_set.merge(a_id, b_id);
                }
              }
            }
          }
        }
      }
      println!("{} {} -> {}", i, j, disjoint_set.len());
    }
  }
}
