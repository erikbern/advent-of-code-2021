use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use regex::Regex;
use std::collections::HashSet;

struct Beacon {
  coords: [i32; 3],
  id: usize,
}

struct Scanner {
  beacons: Vec<Beacon>,
  offset: [i32; 3], // In scanner 0's coordinate system
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
        scanners.push(Scanner{ beacons: Vec::<Beacon>::new(), offset: [0i32; 3] });
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

  let mut h = HashSet::<[i32; 3]>::new();
  
  // Add scanner 0's beacons to the set of coordinates we know
  for a_i in 0..scanners[0].beacons.len() {
    h.insert(scanners[0].beacons[a_i].coords);
  }

  let mut remaining = HashSet::<usize>::from_iter(1..scanners.len());
  while !remaining.is_empty() {
    println!("remaining: {}", remaining.len());
    'search: for i in remaining.iter() {
      println!("... checking {}", i);
      let i: usize = *i;
      for k in 0..24 {
        for anchor in h.iter() {
	  for a_j in 0..scanners[i].beacons.len() {
	    let offset: [i32; 3] = sub(&mult(&m[k], &scanners[i].beacons[a_j].coords), &anchor);
            let mut n_matches: usize = 0;
            for a_i in 0..scanners[i].beacons.len() {
              let proj: [i32; 3] = sub(&mult(&m[k], &scanners[i].beacons[a_i].coords), &offset);
              if h.contains(&proj) {
                n_matches += 1;
              }
	    }
	    if n_matches >= 12 {
	      scanners[i].offset = offset;
              for a_i in 0..scanners[i].beacons.len() {
                let proj: [i32; 3] = sub(&mult(&m[k], &scanners[i].beacons[a_i].coords), &offset);
                h.insert(proj);
              }
	      remaining.remove(&i);
	      break 'search 
            }
	  }
        }
      }
    }
  }
  println!("number of beacons: {}", h.len());
}
