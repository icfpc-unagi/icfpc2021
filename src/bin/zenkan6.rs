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


fn rec(data: &Data, out: &mut Vec<Point>, cand: &Vec<Option<Vec<Point>>>, until: f64) -> bool {
	let n = out.len();
	let rem = out.iter().filter(|p| p.0 >= 0).count();
	if rem == 0 {
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
		if get_time() > until {
			break;
		}
	}
	false
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

#[derive(Clone, Debug)]
struct State {
	out: Vec<Point>,
	cand: Vec<Option<Vec<Point>>>,
	ps: Vec<usize>,
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
}

fn main() {
	let mut input = read_input();
	use rand::prelude::*;
	// let shift = rand::thread_rng().gen_range(0..input.hole.len());
	// let shift = 26; // 64
	// let shift = 17; // 68
	let shift = 52;//19;
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
	let init = State { out: vec![P(-1, -1); n], cand: vec![None; n], ps: vec![!0; data.input.hole.len()] };
	// {
	// 	let init_out = read_output_from_file("tmp.txt").vertices;
	// 	dbg!(&init_out);
	// 	for i in 0..n {
	// 		if init_out[i].0 >= 0 {
	// 			dbg!();
	// 			init.set(&data, i, init_out[i]);
	// 		}
	// 	}
	// }
	// dbg!(&init);
	
	let mut beam = vec![init];
	fn eval(s: &State, data: &Data) -> i64 {
		let mut num = 0;
		for i in 0..s.out.len() {
			if s.out[i].0 >= 0 && data.is_hole[s.out[i].0 as usize][s.out[i].1 as usize] {
				for &j in &data.g[i] {
					if s.out[j].0 >= 0 && data.is_hole[s.out[j].0 as usize][s.out[j].1 as usize] {
						num += 1;
					}
				}
			}
		}
		num
	}
	for h in 1..data.input.hole.len() {
		if beam.len() == 0 {
			break;
		}
		eprintln!("{}: {}, {}", h, beam.len(), eval(&beam[0], &data));
		{
			for k in 0..5.min(beam.len()) {
				eprintln!("  {}: {:?}", k, eval(&beam[0], &data));
			}
			let mut out = beam[0].out.clone();
			for i in 0..n {
				if out[i].0 < 0 {
					out[i] = P((min_x + max_x) / 2 + rand::thread_rng().gen_range(-10..=10), (min_y + max_y) / 2 + rand::thread_rng().gen_range(-10..=10));
				}
			}
			write_output(&Output { vertices: out, bonuses: Default::default() });
			// let stime = get_time();
			// let t = 1.0;
			// let ok = rec(&data, &mut out, &beam[0].1.cand, stime + t);
			// if !ok && get_time() < stime + t {
			// 	eprintln!("orz: {}, {}", ok, get_time() - stime);
			// } else {
			// 	eprintln!("ok");
			// }
			// for i in 0..n {
			// 	eprintln!("{}: {}", i, beam[0].1.cand[i].as_ref().unwrap_or(&vec![]).len());
			// }
		}
		let mut next = vec![];
		for mut state in beam {
			let mut p = None;
			for v in 0..n {
				if state.out[v] == data.input.hole[h] {
					p = Some(v);
					break;
				}
			}
			if let Some(p) = p {
				state.ps[h] = p;
				next.push(state);
			} else {
				next.push(state.clone());
				if state.ps[h - 1] == !0 {
					for i in 0..n {
						if state.out[i].0 < 0 && can_place(&data, &state.out, i, data.input.hole[h - 1]) {
							let mut state = state.clone();
							if state.set(&data, i, data.input.hole[h - 1]) {
								state.ps[h - 1] = i;
								let mut p = None;
								for v in 0..n {
									if state.out[v] == data.input.hole[h] {
										p = Some(v);
										break;
									}
								}
								if let Some(p) = p {
									state.ps[h] = p;
									next.push(state);
								} else {
									for &j in &data.g[state.ps[h - 1]] {
										if state.out[j].0 < 0 && can_place(&data, &state.out, j, data.input.hole[h]) {
											let mut state = state.clone();
											if state.set(&data, j, data.input.hole[h]) {
												state.ps[h] = j;
												next.push(state);
											}
										}
									}
								}
							}
						}
					}
				} else {
					for &j in &data.g[state.ps[h - 1]] {
						if state.out[j].0 < 0 && can_place(&data, &state.out, j, data.input.hole[h]) {
							let mut state = state.clone();
							if state.set(&data, j, data.input.hole[h]) {
								state.ps[h] = j;
								next.push(state);
							}
						}
					}
				}
			}
		}
		next.sort_by_key(|s| eval(s, &data));
		beam = vec![];
		for state in next.into_iter().rev() {
			let stime = get_time();
			if rec(&data, &mut state.out.clone(), &state.cand, stime + 0.01) || get_time() >= stime + 0.01 {
				beam.push(state);
			}
			if beam.len() == 1000 {
				break;
			}
		}
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
		eprintln!("#cand = {}", beam.len());
		for k in 0..beam.len() {
			eprintln!("{:.3}: trial: {}", get_time(), k);
			let mut state = beam[k].clone();
			if rec(&data, &mut state.out, &state.cand, get_time() + 60.0) {
				write_output(&Output { vertices: state.out, bonuses: Default::default() });
				eprintln!("Succeeded!!!");
				return;
			}
		}
	}
	eprintln!("orz");
	write_output(&Output { vertices: vec![], bonuses: Default::default() });
}
