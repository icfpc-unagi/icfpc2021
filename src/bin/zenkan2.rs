use icfpc2021::{*, util::*};

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


fn rec(data: &Data, out: &mut Vec<Point>, cand: &Vec<Option<Vec<Point>>>, until: f64) -> bool {
	let n = out.len();
	if out.iter().all(|p| p.0 >= 0) {
		return true;
	}
	if get_time() > until {
		return false;
	}
	let mut next = vec![];
	let mut min_size = 1 << 30;
	for u in 0..n {
		if out[u].0 >= 0 {
			continue;
		}
		if let Some(ps) = &cand[u] {
			if min_size.setmin(ps.len()) {
				next = ps.iter().map(|&p| (u, p)).collect();
			}
		}
	}
	for (v, p) in next {
		out[v] = p;
		let mut cand = cand.clone();
		for &u in &data.g[v] {
			if out[u].0 < 0 {
				let list = cand[u].clone().unwrap_or(data.cand[u][v].iter().map(|&d| p + d).collect());
				cand[u] = Some(list.into_iter().filter(|&p| can_place(data, &out, u, p)).collect());
			}
		}
		if rec(data, out, &cand, until) {
			return true;
		}
		out[v] = P(-1, -1);
	}
	false
}

#[derive(Clone, Debug)]
struct State {
	out: Vec<Point>,
	cand: Vec<Option<Vec<Point>>>,
	ps: Vec<usize>,
}

impl State {
	fn set(&mut self, data: &Data, u: usize, p: Point) -> bool {
		self.out[u] = p;
		for &v in &data.g[u] {
			if self.out[v].0 < 0 {
				let list: Vec<_> = self.cand[v].clone().unwrap_or(data.cand[v][u].iter().map(|&d| self.out[u] + d).collect());
				let list = list.into_iter().filter(|&p| can_place(&data, &self.out, v, p)).collect::<Vec<_>>();
				if list.len() == 0 {
					return false;
				}
				self.cand[v] = Some(list);
			}
		}
		for u in 0..self.out.len() {
			if self.out[u].0 < 0 && self.cand[u].is_some() && self.cand[u].as_ref().unwrap().len() == 1 {
				let p = self.cand[u].as_ref().unwrap()[0];
				if !self.set(data, u, p) {
					return false;
				}
				break;
			}
		}
		true
	}
}

fn main() {
	let mut input = read_input();
	use rand::prelude::*;
	let shift = rand::thread_rng().gen_range(0..input.hole.len());
	// let shift = 0;
	for _ in 0..shift {
		let p = input.hole.remove(0);
		input.hole.push(p);
	}
	let n = input.figure.vertices.len();
	eprintln!("n = {}, m = {}, #hole = {}, shift = {}", n, input.figure.edges.len(), input.hole.len(), shift);
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
	let mut inside = mat![false; max_x as usize + 1; max_y as usize + 1];
	for x in min_x ..= max_x {
		for y in min_y ..= max_y {
			inside[x as usize][y as usize] = P::contains_p(&input.hole, P(x, y)) >= 0;
		}
	}
	assert!(min_x >= 0);
	assert!(min_y >= 0);
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
	let data = Data { input, dist, inside, g, cand };
	let mut beam = vec![];
	for i in 0..n {
		let mut state = State { out: vec![P(-1, -1); n], cand: vec![None; n], ps: vec![i] };
		if state.set(&data, i, data.input.hole[0]) {
			beam.push(((0, 0), state));
		}
	}
	for h in 1..data.input.hole.len() {
		if beam.len() == 0 {
			break;
		}
		eprintln!("{}: {}, {}", h, beam.len(), beam[0].0.0);
		let mut next = vec![];
		for (k, mut state) in beam {
			let mut p = None;
			for v in 0..n {
				if state.out[v] == data.input.hole[h] {
					p = Some(v);
					break;
				}
			}
			if let Some(p) = p {
				let mut k = k;
				if h + 1 == data.input.hole.len() && !has_edge[state.ps[0]][p] {
					k.0 += 1;
					k.1 += (data.dist[state.ps[0]][p] * 10000.0) as i64;
				}
				if !has_edge[state.ps[h - 1]][p] {
					k.0 += 1;
					k.1 += (data.dist[state.ps[h - 1]][p] * 10000.0) as i64;
				}
				state.ps.push(p);
				next.push((k, state));
			} else {
				for p in 0..n {
					if state.out[p].0 < 0 {
						if can_place(&data, &state.out, p, data.input.hole[h]) {
							let mut k = k;
							if h + 1 == data.input.hole.len() && !has_edge[state.ps[0]][p] {
								k.0 += 1;
								k.1 += (data.dist[state.ps[0]][p] * 10000.0) as i64;
							}
							if !has_edge[state.ps[h - 1]][p] {
								k.0 += 1;
								k.1 += (data.dist[state.ps[h - 1]][p] * 10000.0) as i64;
							}
							let mut state = state.clone();
							if state.set(&data, p, data.input.hole[h]) {
								state.ps.push(p);
								next.push((k, state));
							}
						}
					}
				}
			}
		}
		next.sort_by_key(|&(k, _)| k);
		next.truncate(2000);
		beam = next;
	}
	if beam.len() > 0 {
		// {
		// 	let mut out = beam[0].1.out.clone();
		// 	for i in 0..n {
		// 		if out[i].0 < 0 {
		// 			out[i] = P((min_x + max_x) / 2, (min_y + max_y) / 2);
		// 		}
		// 	}
		// 	write_output(&Output { vertices: out });
		// }
		eprintln!("#cand = {}, min_skip = {}", beam.len(), beam[0].0.0);
		for k in 0..beam.len() {
			eprintln!("{:.3}: trial: {}", get_time(), k);
			let mut state = beam[k].1.clone();
			if rec(&data, &mut state.out, &state.cand, get_time() + 60.0) {
				write_output(&Output { vertices: state.out });
				eprintln!("Succeeded!!!");
				return;
			}
		}
	}
	eprintln!("orz");
	write_output(&Output { vertices: vec![] });
}
