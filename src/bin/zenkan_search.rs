use icfpc2021::{*, util::*};
use rand::Rng;

pub fn get_time() -> f64 {
	static mut STIME: f64 = -1.0;
	let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
	unsafe {
		if STIME < 0.0 {
			STIME = ms;
		}
		ms - STIME
	}
}

struct Data {
	input: Input,
	dist: Vec<Vec<f64>>,
	inside: Vec<Vec<bool>>,
	is_hole: Vec<Vec<bool>>,
	g: Vec<Vec<usize>>,
	cand: Vec<Vec<Vec<Point>>>,
}

fn can_place(data: &Data, out: &Vec<Point>, u: usize, p: Point) -> bool {
	if p.0 < 0 || p.0 >= data.inside.len() as i64 || p.1 < 0 || p.1 >= data.inside[0].len() as i64 || !data.inside[p.0 as usize][p.1 as usize] {
		return false;
	}
	for &v in &data.g[u] {
		if out[v].0 < 0 {
			continue;
		}
		if !P::contains_s(&data.input.hole, (p, out[v])) {
			return false;
		}
		let before = (data.input.figure.vertices[v] - data.input.figure.vertices[u]).abs2();
		let after = (out[v] - p).abs2();
		if (after * 1000000 - before * 1000000).abs() > data.input.epsilon * before {
			return false;
		}
	}
	let mul_ub = (1.0 + data.input.epsilon as f64 * 1e-6).sqrt();
	for v in 0..out.len() {
		if out[v].0 < 0 {
			continue;
		}
		let dist = ((p - out[v]).abs2() as f64).sqrt();
		if data.dist[u][v] as f64 * mul_ub < dist - 1e-4 {
			return false;
		}
	}
	true
}

const W: usize = 10;

fn rec(state: &State, s: usize, prev: usize, data: &Data, prev_score: i64, next: &mut Vec<(i64, State)>) {
	let p = data.input.hole[s];
	if state.orz(&data) {
		return;
	}
	let score = state.score(data);
	if prev_score < score && next.iter().all(|&(score2, ref state2)| score != score2 || state.out != state2.out) {
		let mut p = next.len();
		while p > 0 && next[p - 1].0 < score {
			p -= 1;
		}
		if p < W {
			next.insert(p, (score, state.clone()));
			if next.len() == W {
				next.pop();
			}
		}
	}
	if state.out.contains(&p) {
		return;
	}
	if prev == !0 {
		for i in 0..state.out.len() {
			if state.out[i].0 < 0 && can_place(data, &state.out, i, p) {
				let mut state = state.clone();
				if state.set(data, i, p) {
					rec(&state, (s + 1) % data.input.hole.len(), i, data, score, next);
				}
			}
		}
	} else {
		for &i in &data.g[prev] {
			if state.out[i].0 < 0 && can_place(data, &state.out, i, p) {
				let mut state = state.clone();
				if state.set(data, i, p) {
					rec(&state, (s + 1) % data.input.hole.len(), i, data, score, next);
				}
			}
		}
	}
}


fn get_mins(input: &Input, out: &Vec<Point>) -> Vec<i64> {
	input.hole.iter().map(|&p| {
		let mut min = i64::max_value();
		for &q in out {
			if q.0 >= 0 {
				min.setmin((p - q).abs2());
			}
		}
		min
	}).collect()
}

pub fn get_new_graph(input: &Input, pre: &Vec<P<i64>>, dont_move: &Vec<bool>) -> Vec<P<i64>> {
	let mut now = pre.clone();
	for _ in 0..100000 {
		let mut next_now = now.clone();
		for e in &input.figure.edges {
			let a = e.0;
			let b = e.1;
			let d1 = (now[b] - now[a]).abs2();
			let d2 = (input.figure.vertices[b] - input.figure.vertices[a]).abs2();
			if d1 > d2 {
				if !dont_move[a] {
					next_now[a].0 += (now[b] - now[a]).0 / 20;
					next_now[a].1 += (now[b] - now[a]).1 / 20;
				}
				if !dont_move[b] {
					next_now[b].0 += (now[a] - now[b]).0 / 20;
					next_now[b].1 += (now[a] - now[b]).1 / 20;
				}
			} else {
				if !dont_move[a] {
					next_now[a].0 -= (now[b] - now[a]).0 / 40;
					next_now[a].1 -= (now[b] - now[a]).1 / 40;
				}
				if !dont_move[b] {
					next_now[b].0 -= (now[a] - now[b]).0 / 40;
					next_now[b].1 -= (now[a] - now[b]).1 / 40;
				}
			}
		}
		now = next_now.clone();
	}
	return now;
}

#[derive(Clone, Debug)]
struct State {
	out: Vec<Point>,
	cand: Vec<Option<Vec<Point>>>,
}


impl State {
	fn set(&mut self, data: &Data, u: usize, p: Point) -> bool {
		self.out[u] = p;
		let mut us = vec![];
		for &v in &data.g[u] {
			if self.out[v].0 < 0 {
				let list: Vec<_> = self.cand[v].clone().unwrap_or(data.cand[v][u].iter().map(|&d| self.out[u] + d).collect());
				let list = list.into_iter().filter(|&p| can_place(&data, &self.out, v, p)).collect::<Vec<_>>();
				if list.len() == 0 {
					return false;
				}
				if self.cand[v].as_ref() != Some(&list) {
					us.push(v);
					self.cand[v] = Some(list);
				}
			}
		}
		self.fixing(data, us)
	}
	fn propagete(&mut self, data: &Data, u: usize) -> bool {
		let ps = self.cand[u].clone().unwrap();
		let mut us = vec![];
		for &v in &data.g[u] {
			if self.out[v].0 < 0 {
				let mut list = vec![];
				for &p in &ps {
					self.out[u] = p;
					let tmp = self.cand[v].clone().unwrap_or(data.cand[v][u].iter().map(|&d| p + d).collect());
					list.extend(tmp.into_iter().filter(|&p| can_place(&data, &self.out, v, p)).collect::<Vec<_>>());
				}
				if list.len() == 0 {
					return false;
				}
				if self.cand[v].as_ref() != Some(&list) {
					us.push(v);
					self.cand[v] = Some(list);
				}
				self.out[u] = P(-1, -1);
			}
		}
		self.fixing(data, us)
	}
	fn fixing(&mut self, data: &Data, us: Vec<usize>) -> bool {
		for u in us {
			if self.out[u].0 < 0 && self.cand[u].is_some() {
				if data.g[u].iter().all(|&v| self.out[v].0 >= 0) {
					let cand = self.cand[u].as_ref().unwrap();
					let mins = get_mins(&data.input, &self.out);
					let mut best = -1;
					let mut p = cand[0];
					for &q in cand {
						let mut score = 0;
						for i in 0..mins.len() {
							if mins[i] > (q - data.input.hole[i]).abs2() {
								score += mins[i] - (q - data.input.hole[i]).abs2();
							}
						}
						if best.setmax(score) {
							p = q;
						}
					}
					if !self.set(data, u, p) {
						return false;
					}
				} else if self.cand[u].as_ref().unwrap().len() == 1 {
					let p = self.cand[u].as_ref().unwrap()[0];
					if !self.set(data, u, p) {
						return false;
					}
				} else if self.cand[u].as_ref().unwrap().len() <= 10 {
					if !self.propagete(data, u) {
						return false;
					}
				}
			}
		}
		true
	}
	fn score(&self, data: &Data) -> i64 {
		let mut score = 0;
		for i in 0..self.out.len() {
			if self.out[i].0 >= 0 && data.is_hole[self.out[i].0 as usize][self.out[i].1 as usize] {
				for &j in &data.g[i] {
					if self.out[j].0 >= 0 && data.is_hole[self.out[j].0 as usize][self.out[j].1 as usize] {
						score += 1;
					}
				}
			}
		}
		score
	}
	fn orz(&self, data: &Data) -> bool {
		for i in 0..data.input.hole.len() {
			if !self.out.contains(&data.input.hole[i]) {
				let mut ok = false;
				for j in 0..self.out.len() {
					if self.out[j].0 < 0 {
						if let Some(ref c) = self.cand[j] {
							if c.contains(&data.input.hole[i]) {
								ok = true;
								break;
							}
						} else if can_place(data, &self.out, j, data.input.hole[i]){
							ok = true;
							break;
						}
					}
				}
				if !ok {
					return true;
				}
			}
		}
		false
	}
}

fn output(out: &Vec<Point>, data: &Data) {
	// let mut out = out.clone();
	// for i in 0..out.len() {
	// 	if out[i].0 < 0 {
	// 		out[i] += def + P(rand::thread_rng().gen_range(-10..=10), rand::thread_rng().gen_range(-10..=10));
	// 	}
	// }
	let mut out = out.clone();
	let mut dont_move = vec![false; out.len()];
	for i in 0..out.len() {
		if out[i].0 >= 0 {
			dont_move[i] = true;
		} else {
			out[i] = P(rand::thread_rng().gen_range(0..100), rand::thread_rng().gen_range(0..100));
		}
	}
	let out = get_new_graph(&data.input, &out, &dont_move);
	write_output(&Output { vertices: out });
}

fn main() {
	let input = read_input();
	let n = input.figure.vertices.len();
	eprintln!("n = {}, m = {}, #hole = {}", n, input.figure.edges.len(), input.hole.len());
	let mut g = vec![vec![]; n];
	let mut has_edge = mat![false; n; n];
	let mut dist = mat![1e20; n; n];
	for &(i, j) in &input.figure.edges {
		g[i].push(j);
		g[j].push(i);
		has_edge[i][j] = true;
		has_edge[j][i] = true;
		dist[i][j] = ((input.figure.vertices[i] - input.figure.vertices[j]).abs2() as f64).sqrt();
		dist[j][i] = dist[i][j];
	}
	for k in 0..n {
		for i in 0..n {
			for j in 0..n {
				let tmp = dist[i][k] + dist[k][j];
				dist[i][j].setmin(tmp);
			}
		}
	}
	let min_x = input.hole.iter().map(|p| p.0).min().unwrap();
	let max_x = input.hole.iter().map(|p| p.0).max().unwrap();
	let min_y = input.hole.iter().map(|p| p.1).min().unwrap();
	let max_y = input.hole.iter().map(|p| p.1).max().unwrap();
	assert!(min_x >= 0);
	assert!(min_y >= 0);
	let mut inside = mat![false; max_x as usize + 1; max_y as usize + 1];
	let mut is_hole = mat![false; max_x as usize + 1; max_y as usize + 1];
	for x in min_x ..= max_x {
		for y in min_y ..= max_y {
			inside[x as usize][y as usize] = P::contains_p(&input.hole, P(x, y)) >= 0;
		}
	}
	for &p in &input.hole {
		is_hole[p.0 as usize][p.1 as usize] = true;
	}
	let mut cand = mat![vec![]; n; n];
	for i in 0..n {
		for &r in &g[i] {
			let orig = (input.figure.vertices[r] - input.figure.vertices[i]).abs2();
			for dx in -(max_x - min_x) ..= (max_x - min_x) {
				for dy in -(max_y - min_y) ..= (max_y - min_y) {
					if (P(dx, dy).abs2() * 1000000 - orig * 1000000).abs() <= input.epsilon * orig {
						cand[i][r].push(P(dx, dy));
					}
				}
			}
		}
	}
	let data = Data { input, dist, inside, is_hole, g, cand };
	let mut beam = vec![(0, State { out: vec![P(-1, -1); n], cand: vec![None; n] })];
	for iter in 0.. {
		let mut next = vec![];
		for s in 0..data.input.hole.len() {
			eprintln!("{} / {}", s, data.input.hole.len());
			for &(score, ref state) in &beam {
				rec(state, s, !0, &data, score, &mut next);
			}
		}
		if next.len() == 0 {
			break;
		}
		beam = next;
		eprintln!("{}: {} / {}", iter, beam[0].0, data.input.hole.len());
		output(&beam[0].1.out, &data);
	}
	
	eprintln!("Finished");
}
