pub mod util;
pub mod paths;

#[cfg(test)]
mod test_contains_s;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
pub mod tree_decomposition;

use std::{io, ops::*};
use std::cmp::Ordering;
use num::{Signed, Zero};
use serde::{Serialize, Deserialize};
use util::*;
use std::fmt::Debug;

pub type Point = P<i64>;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Input {
	pub hole: Vec<Point>,
	pub figure: Figure,
	pub epsilon: i64,
	#[serde(default)]
	pub bonuses: Vec<Bonus>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Figure {
	pub edges: Vec<(usize, usize)>,
	pub vertices: Vec<Point>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Bonus {
	pub bonus: BonusType,
	pub problem: u32,
	pub position: Point,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub enum BonusType {
	#[serde(rename = "GLOBALIST")]
	Globalist,
	#[serde(rename = "BREAK_A_LEG")]
	BreakALeg,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Output {
	pub vertices: Vec<Point>
}

pub fn read_input() -> Input {
	read_input_from_reader(std::io::stdin()).unwrap()
}


pub fn read_input_from_file(f: impl AsRef<std::path::Path>) -> Input {
	read_input_from_reader(std::fs::File::open(f).unwrap()).unwrap()
}

pub fn read_input_from_reader<R: io::Read>(r: R) -> io::Result<Input> {
	let mut input: Input = serde_json::from_reader(r)?;
	for i in 0..input.figure.edges.len() {
		if input.figure.edges[i].0 > input.figure.edges[i].1 {
			let t = input.figure.edges[i].0;
			input.figure.edges[i].0 = input.figure.edges[i].1;
			input.figure.edges[i].1 = t;
		}
	}
	input.figure.edges.sort();
	input.figure.edges.dedup();
	let mut area = 0;
	for i in 0..input.hole.len() {
		area += input.hole[i].det(input.hole[(i + 1) % input.hole.len()]);
	}
	if area > 0 { // 時計回りにする
		input.hole.reverse();
	}
	Ok(input)
}

pub fn write_output(out: &Output) {
	println!("{}", serde_json::to_string(out).unwrap());
}

pub fn read_output_from_file(f: impl AsRef<std::path::Path>) -> Output {
	serde_json::from_reader(std::fs::File::open(f).unwrap()).unwrap()
}

pub fn compute_score(input: &Input, out: &Output) -> i64 {
	if out.vertices.len() != input.figure.vertices.len() {
		return 1000000000;
	}
	let mut score = 0;
	for &p in &input.hole {
		let mut min = i64::max_value();
		for &q in &out.vertices {
			min.setmin((p - q).abs2());
		}
		score += min;
	}
	for &p in &out.vertices {
		if P::contains_p(&input.hole, p) < 0 {
			eprintln!("outside point");
			return 1000000000;
		}
	}
	for &(i, j) in &input.figure.edges {
		if !P::contains_s(&input.hole, (out.vertices[i], out.vertices[j])) {
			eprintln!("cross edge");
			return 1000000000;
		}
		let before = (input.figure.vertices[i] - input.figure.vertices[j]).abs2();
		let after = (out.vertices[i] - out.vertices[j]).abs2();
		if (after * 1000000 - before * 1000000).abs() > input.epsilon * before {
			eprintln!("illegal length");
			return 1000000000;
		}
	}
	score
}

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct P<T>(pub T, pub T);

impl<T> Add for P<T> where T: Copy + Add<Output = T> {
	type Output = P<T>;
	#[inline]
	fn add(self, a: P<T>) -> P<T> {
		P(self.0 + a.0, self.1 + a.1)
	}
}

impl<T> AddAssign for P<T> where T: Copy + AddAssign {
	#[inline]
	fn add_assign(&mut self, a: P<T>) {
		self.0 += a.0;
		self.1 += a.1;
	}
}

impl<T> Sub for P<T> where T: Copy + Sub<Output = T> {
	type Output = P<T>;
	#[inline]
	fn sub(self, a: P<T>) -> P<T> {
		P(self.0 - a.0, self.1 - a.1)
	}
}

impl<T> SubAssign for P<T> where T: Copy + SubAssign {
	#[inline]
	fn sub_assign(&mut self, a: P<T>) {
		self.0 -= a.0;
		self.1 -= a.1;
	}
}

impl<T> Mul<T> for P<T> where T: Copy + Mul<Output = T> {
	type Output = P<T>;
	#[inline]
	fn mul(self, a: T) -> P<T> {
		P(self.0 * a, self.1 * a)
	}
}

impl<T> MulAssign<T> for P<T> where T: Copy + MulAssign {
	#[inline]
	fn mul_assign(&mut self, a: T) {
		self.0 *= a;
		self.1 *= a;
	}
}

impl<T> P<T> where T: Copy + Signed {
	#[inline]
	pub fn dot(self, a: P<T>) -> T {
		(self.0 * a.0) + (self.1 * a.1)
	}
	#[inline]
	pub fn det(self, a: P<T>) -> T {
		(self.0 * a.1) - (self.1 * a.0)
	}
	#[inline]
	pub fn abs2(self) -> T {
		self.dot(self)
	}
	#[inline]
	pub fn rot(self) -> P<T> {
		P(-self.1, self.0)
	}
}


macro_rules! impl_cmp {
	($name:ident $(<$($t:ident),*>)*; |$x:ident, $y:ident| $e:expr; $($w:tt)*) => {
		impl $(<$($t),*>)* Ord for $name $(<$($t),*>)* $($w)* {
			#[inline]
			fn cmp(&self, $y: &Self) -> ::std::cmp::Ordering {
				let $x = &self;
				$e
			}
		}
		impl $(<$($t),*>)* PartialOrd for $name $(<$($t),*>)* $($w)* {
			#[inline]
			fn partial_cmp(&self, a: &Self) -> Option<::std::cmp::Ordering> {
				Some(self.cmp(a))
			}
		}
		impl $(<$($t),*>)* PartialEq for $name $(<$($t),*>)* $($w)* {
			#[inline]
			fn eq(&self, a: &Self) -> bool {
				self.cmp(a) == ::std::cmp::Ordering::Equal
			}
		}
		impl $(<$($t),*>)* Eq for $name $(<$($t),*>)* $($w)* {}
	}
}

/// R(n,d) := n / d.
#[derive(Clone, Copy, Debug)]
pub struct R<T>(pub T, pub T);
impl_cmp!(R<T>; |a, b| {
		match (a.1 * b.1).cmp(&T::zero()) {
			Ordering::Greater => (a.0 * b.1).cmp(&(a.1 * b.0)),
			Ordering::Less => (a.1 * b.0).cmp(&(a.0 * b.1)),
			_ => panic!("division by zero")
		}
	}; where T: Copy + Zero + Ord + Mul<Output = T>);

impl<T> From<T> for R<T> where T: From<u8> {
	#[inline]
	fn from(x: T) -> R<T> { R(x, T::from(1)) }
}

impl<T> From<P<T>> for P<R<T>> where R<T>: From<T> {
	#[inline]
	fn from(p: P<T>) -> P<R<T>> { P(p.0.into(), p.1.into()) }
}

impl<T> R<T> where T: Div<Output = T> {
	#[inline]
	pub fn val(self) -> T { self.0 / self.1 }
}

impl<T> From<P<R<T>>> for P<T> where T: Div<Output = T> {
	#[inline]
	fn from(x: P<R<T>>) -> P<T> {
		P(x.0.val(), x.1.val())
	}
}

#[inline]
fn sig<T>(x: T) -> i32 where T: Zero + Ord {
	match x.cmp(&T::zero()) {
		Ordering::Greater => 1,
		Ordering::Less => -1,
		_ => 0
	}
}

/// 直線に関する演算. 分数を用いて計算誤差なしで行う.
impl<T> P<T> where T: Copy + From<u8> + Ord + Signed {
	/// Square distance between segment and point. (D^4,D^2).
	pub fn dist2_sp((p1, p2): (P<T>, P<T>), q: P<T>) -> R<T> {
		if (p2 - p1).dot(q - p1) <= T::zero() { (q - p1).abs2().into() }
		else if (p1 - p2).dot(q - p2) <= T::zero() { (q - p2).abs2().into() }
		else { P::dist2_lp((p1, p2), q) }
	}
	/// Square distance between line and point. (D^4,D^2).
	pub fn dist2_lp((p1, p2): (P<T>, P<T>), q: P<T>) -> R<T> {
		let det = (p2 - p1).det(q - p1);
		R(det * det, (p2 - p1).abs2())
	}
	/// D^2.
	pub fn crs_sp((p1, p2): (P<T>, P<T>), q: P<T>) -> bool {
		P::crs_lp((p1, p2), q) && (q - p1).dot(q - p2) <= T::zero()
	}
	/// D^2.
	pub fn crs_lp((p1, p2): (P<T>, P<T>), q: P<T>) -> bool {
		(p2 - p1).det(q - p1) == T::zero()
	}
	/// D^2.
	pub fn crs_ss((p1, p2): (P<T>, P<T>), (q1, q2): (P<T>, P<T>)) -> bool {
		let sort = |a, b| { if a < b { (a, b) } else { (b, a) }};
		let (lp0, up0) = sort(p1.0, p2.0);
		let (lq0, uq0) = sort(q1.0, q2.0);
		let (lp1, up1) = sort(p1.1, p2.1);
		let (lq1, uq1) = sort(q1.1, q2.1);
		if up0 < lq0 || uq0 < lp0 || up1 < lq1 || uq1 < lp1 { return false }
		return sig((p2 - p1).det(q1 - p1)) * sig((p2 - p1).det(q2 - p1)) <= 0
			&& sig((q2 - q1).det(p1 - q1)) * sig((q2 - q1).det(p2 - q1)) <= 0
	}
	/// (D^3,D^2).
	pub fn proj((p1, p2): (P<T>, P<T>), q: P<T>) -> P<R<T>> {
		let d = (p2 - p1).abs2();
		let r = p1 * d + (p2 - p1) * (p2 - p1).dot(q - p1);
		P(R(r.0, d), R(r.1, d))
	}
	/// (D^3,D^2).
	pub fn pi_ll((p1, p2): (P<T>, P<T>), (q1, q2): (P<T>, P<T>)) -> Option<P<R<T>>> {
		let d = (q2 - q1).det(p2 - p1);
		if d == T::zero() { return None }
		let r = p1 * d + (p2 - p1) * (q2 - q1).det(q1 - p1);
		Some(P(R(r.0, d), R(r.1, d)))
	}
	/// 点の多角形に対する内外判定
	/// 内部のときは1、辺上のときは0、外部のときは-1を返す
	pub fn contains_p(ps: &Vec<P<T>>, q: P<T>) -> i32 {
		let n = ps.len();
		let mut ret = -1;
		for i in 0..n {
			let mut a = ps[i] - q;
			let mut b = ps[(i + 1) % n] - q;
			if a.1 > b.1 {
				std::mem::swap(&mut a, &mut b);
			}
			if a.1 <= T::zero() && b.1 > T::zero() && a.det(b) > T::zero() {
				ret = -ret;
			}
			if a.det(b) == T::zero() && a.dot(b) <= T::zero() {
				return 0;
			}
		}
		ret
	}
	// 多角形(境界を含む)に線分が完全に含まれているか
	// psは時計回り
	// O(n)
	pub fn contains_s(ps: &Vec<P<T>>, (q1, q2): (P<T>, P<T>)) -> bool {
		if P::contains_p(ps, q1) < 0 || P::contains_p(ps, q2) < 0 {
			return false;
		}
		let n = ps.len();
		for i in 0..n {
			if P::crs_ss((q1, q2), (ps[i], ps[(i + 1) % n])) {
				let mut r = P::pi_ll((q1, q2), (ps[i], ps[(i + 1) % n]));
				if r.is_none() && (q1 - ps[i]).dot(q2 - ps[i]) <= T::zero() {
					r = Some(ps[i].into());
				}
				if let Some(r) = r {
					if r == q2.into() || r == ps[(i + 1) % n].into() {
						continue;
					}
					if r == ps[i].into() {
						let p = ps[(i + 1) % n] - ps[i];
						let q = q2 - ps[i];
						let r = ps[(i + n - 1) % n] - ps[i];
						let pr = p.det(r);
						let pq = p.det(q);
						let qr = q.det(r);
						if pr == T::zero() && p.dot(r) < T::zero() && pq > T::zero() {
							return false;
						}
						if pr > T::zero() && pq > T::zero() && qr > T::zero() {
							return false;
						}
						if pr < T::zero() && (pq > T::zero() || qr > T::zero()) {
							return false;
						}
					} else if r == q1.into() {
						if (ps[(i + 1) % n] - ps[i]).det(q2 - ps[i]) > T::zero() {
							return false;
						}
					} else {
						return false;
					}
				}
			}
		}
		true
	}
}

// epsilon 以内の伸び縮み判別
fn stretch_within<T: num::traits::Signed + From<i32> + Copy>(d2: T, base_d2: T, epsilon: T) -> Ordering {
	let diff = d2 - base_d2;
	if !(diff.abs() * 1000000.into() - epsilon * base_d2).is_positive() {
		Ordering::Equal
	} else if diff.is_negative() {
		Ordering::Less
	} else {
		Ordering::Greater
	}
}

//
// shortest path
//

fn compute_adjmat(hole: &Vec<Point>) -> Vec<Vec<bool>> {
	let n_vs = hole.len();
	let mut adjmat = vec![vec![false; n_vs]; n_vs];

	for u in 0..n_vs {
		for v in (u + 1)..n_vs {
			let b = P::contains_s(&hole, (hole[u], hole[v]));
			adjmat[u][v] = b;
			adjmat[v][u] = b;
		}
	}

	adjmat
}

fn shortest_path_rec(i: usize, j: usize, via: &Vec<Vec<usize>>, path: &mut Vec<usize>) {
	if via[i][j] == !0 {
		path.push(i);
	} else {
		let k = via[i][j];
		shortest_path_rec(i, k, via, path);
		shortest_path_rec(k, j, via, path);
	}
}

/// p0からp1にhole内部だけを通って行く最短経路を計算する。
///
/// p0, p1はhole内部あるいは境界上であること。
/// 距離と、通過する端点の列（holeの頂点番号）を返す。
pub fn shortest_path(hole: &Vec<Point>, p0: Point, p1: Point) -> (f64, Vec<usize>) {
	// 直接いけるならもういいよ
	if P::contains_s(&hole, (p0, p1)) {
		return (((p1 - p0).abs2() as f64).sqrt(), vec![]);
	}

	// TODO: adjmat作るのが重かったらここキャッシュできる
	let mut adjmat = compute_adjmat(hole);

	let n_hole_vs = hole.len();
	for u in 0..n_hole_vs {
		adjmat[u].push(P::contains_s(&hole, (hole[u], p0)));
		adjmat[u].push(P::contains_s(&hole, (hole[u], p1)));
	}

	let mut row0: Vec<bool> = (0..n_hole_vs).map(|i| adjmat[i][n_hole_vs]).collect();
	let mut row1: Vec<bool> = (0..n_hole_vs).map(|i| adjmat[i][n_hole_vs + 1]).collect();
	let n_vs = n_hole_vs + 2;
	row0.resize(n_vs, false);
	row1.resize(n_vs, false);
	adjmat.push(row0);
	adjmat.push(row1);

	let mut ps = hole.clone();
	ps.push(p0);
	ps.push(p1);

	let mut dst = vec![vec![1e30; n_vs]; n_vs];
	for u in 0..n_vs {
		for v in (u + 1)..n_vs {
			if adjmat[u][v] {
				let d = ((ps[u] - ps[v]).abs2() as f64).sqrt();
				dst[u][v] = d;
				dst[v][u] = d;
			}
		}
	}

	let mut via = vec![vec![!0; n_vs]; n_vs];
	for k in 0..n_vs {
		for i in 0..n_vs {
			for j in 0..n_vs {
				if dst[i][k] + dst[k][j] < dst[i][j] {
					dst[i][j] = dst[i][k] + dst[k][j];
					via[i][j] = k;
				}
			}
		}
	}

	let mut path = vec![];
	let s = n_hole_vs;
	let t = n_hole_vs + 1;
	shortest_path_rec(s, t, &via, &mut path);
	assert_eq!(path.remove(0), s);

	(dst[s][t], path)
}

#[cfg(test)]
mod shortest_path_tests {
	use super::*;

	fn generate_tsubo() -> Vec<Point> {
		vec![P(0, 0), P(2, 4), P(0, 8), P(8, 8), P(6, 4), P(8, 0)]
	}

	#[test]
	fn tsubo1() {
		let (d, p) = shortest_path(&generate_tsubo(), P(2, 2), P(2, 6));
		assert_eq!(p, Vec::<usize>::new());
	}

	#[test]
	fn tsubo2() {
		let (d, p) = shortest_path(&generate_tsubo(), P(1, 2), P(1, 6));
		assert_eq!(p, vec![1]);
	}

	fn generate_boko() -> Vec<Point> {
		vec![
			P(0, 0),
			P(0, 8),
			P(2, 8),
			P(2, 4),
			P(6, 4),
			P(6, 8),
			P(8, 8),
			P(8, 0),
		]
	}

	#[test]
	fn boko1() {
		let (d, p) = shortest_path(&generate_boko(), P(1, 2), P(7, 2));
		assert_eq!(p, Vec::<usize>::new());
	}

	#[test]
	fn boko2() {
		let (d, p) = shortest_path(&generate_boko(), P(1, 6), P(7, 6));
		assert_eq!(p, vec![3, 4]);
	}

	#[test]
	fn boko3() {
		let (d, p) = shortest_path(&generate_boko(), P(7, 6), P(1, 6));
		assert_eq!(p, vec![4, 3]);
	}
}
