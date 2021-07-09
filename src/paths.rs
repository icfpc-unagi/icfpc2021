use std::fmt::*;
use crate::P;

pub fn polygon<T: std::fmt::Display>(points: &[P<T>]) -> String {
  let mut s = String::new();
  for P(x, y) in points {
    write!(s, "{},{} ", x, y).unwrap();
  }
  s
}

pub fn path<T: std::fmt::Display>(edges: &[(usize, usize)], vertices: &[P<T>]) -> String {
  let mut s = String::new();
  for &(i, j) in edges {
    write!(
      s,
      "M{} {}L{} {}",
      vertices[i].0, vertices[i].1, vertices[j].0, vertices[j].1
    ).unwrap();
  }
  s
}
