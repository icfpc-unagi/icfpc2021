pub mod util;
pub mod paths;

#[cfg(test)]
mod test_contains_s;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

use std::ops::*;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use util::*;

pub type Point = P<i64>;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Input {
	pub hole: Vec<Point>,
	pub figure: Figure,
	pub epsilon: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Figure {
	pub edges: Vec<(usize, usize)>,
	pub vertices: Vec<Point>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct Output {
	pub vertices: Vec<Point>
}

pub fn read_input() -> Input {
	let mut input: Input = serde_json::from_reader(std::io::stdin()).unwrap();
	input.figure.edges.sort();
	input.figure.edges.dedup();
	let mut area = 0;
	for i in 0..input.hole.len() {
		area += input.hole[i].det(input.hole[(i + 1) % input.hole.len()]);
	}
	if area > 0 { // 時計回りにする
		input.hole.reverse();
	}
	input
}


pub fn read_input_from_file(f: &std::path::PathBuf) -> Input {
	let mut input: Input = serde_json::from_reader(std::fs::File::open(f).unwrap()).unwrap();
	input.figure.edges.sort();
	input.figure.edges.dedup();
	let mut area = 0;
	for i in 0..input.hole.len() {
		area += input.hole[i].det(input.hole[(i + 1) % input.hole.len()]);
	}
	if area > 0 { // 時計回りにする
		input.hole.reverse();
	}
	input
}

pub fn write_output(out: &Output) {
	println!("{}", serde_json::to_string(out).unwrap());
}

pub fn read_output_from_file(f: &std::path::PathBuf) -> Output {
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

impl<T> P<T> where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> {
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
		match (a.1 * b.1).cmp(&T::default()) {
			Ordering::Greater => (a.0 * b.1).cmp(&(a.1 * b.0)),
			Ordering::Less => (a.1 * b.0).cmp(&(a.0 * b.1)),
			_ => panic!("division by zero")
		}
	}; where T: Copy + Default + Ord + Mul<Output = T>);

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
fn sig<T>(x: T) -> i32 where T: Default + Ord {
	match x.cmp(&T::default()) {
		Ordering::Greater => 1,
		Ordering::Less => -1,
		_ => 0
	}
}

/// 直線に関する演算. 分数を用いて計算誤差なしで行う.
impl<T> P<T> where T: Copy + Default + From<u8> + Ord + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> {
	/// Square distance between segment and point. (D^4,D^2).
	pub fn dist2_sp((p1, p2): (P<T>, P<T>), q: P<T>) -> R<T> {
		if (p2 - p1).dot(q - p1) <= T::default() { (q - p1).abs2().into() }
		else if (p1 - p2).dot(q - p2) <= T::default() { (q - p2).abs2().into() }
		else { P::dist2_lp((p1, p2), q) }
	}
	/// Square distance between line and point. (D^4,D^2).
	pub fn dist2_lp((p1, p2): (P<T>, P<T>), q: P<T>) -> R<T> {
		let det = (p2 - p1).det(q - p1);
		R(det * det, (p2 - p1).abs2())
	}
	/// D^2.
	pub fn crs_sp((p1, p2): (P<T>, P<T>), q: P<T>) -> bool {
		P::crs_lp((p1, p2), q) && (q - p1).dot(q - p2) <= T::default()
	}
	/// D^2.
	pub fn crs_lp((p1, p2): (P<T>, P<T>), q: P<T>) -> bool {
		(p2 - p1).det(q - p1) == T::default()
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
		if d == T::default() { return None }
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
			if a.1 <= T::default() && b.1 > T::default() && a.det(b) > T::default() {
				ret = -ret;
			}
			if a.det(b) == T::default() && a.dot(b) <= T::default() {
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
				if r.is_none() && (q1 - ps[i]).dot(q2 - ps[i]) <= T::default() {
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
						if pr == T::default() && p.dot(r) < T::default() && pq > T::default() {
							return false;
						}
						if pr > T::default() && pq > T::default() && qr > T::default() {
							return false;
						}
						if pr < T::default() && (pq > T::default() || qr > T::default()) {
							return false;
						}
					} else if r == q1.into() {
						if (ps[(i + 1) % n] - ps[i]).det(q2 - ps[i]) > T::default() {
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
