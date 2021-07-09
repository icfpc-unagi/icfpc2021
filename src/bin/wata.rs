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
	inside: Vec<Vec<bool>>,
	g: Vec<Vec<usize>>,
	parent: Vec<usize>,
	cand: Vec<Vec<Point>>,
}

fn compute_score(input: &Input, out: &Vec<Point>) -> i64 {
	let mut score = 0;
	for &p in &input.hole {
		let mut min = i64::max_value();
		for &q in out {
			min.setmin((p - q).abs2());
		}
		score += min;
	}
	score
}

fn rec(data: &Data, i: usize, order: &Vec<usize>, out: &mut Vec<Point>, used: &mut Vec<bool>, best: &mut Vec<Point>, best_score: &mut i64, until: f64) {
	if i == order.len() {
		if best_score.setmin(compute_score(&data.input, out)) {
			eprintln!("{:.3}: {}", get_time(), best_score);
			*best = out.clone();
		}
		return;
	}
	if get_time() > until {
		return;
	}
	let u = order[i];
	used[u] = true;
	let r = out[data.parent[u]];
	for &d in &data.cand[u] {
		out[u] = r + d;
		if out[u].0 < 0 || out[u].1 < 0 || out[u].0 >= data.inside.len() as i64 || out[u].1 >= data.inside[0].len() as i64 || !data.inside[out[u].0 as usize][out[u].1 as usize] {
			continue;
		}
		let mut ok = true;
		for &v in &data.g[u] {
			if used[v] {
				let before = (data.input.figure.vertices[v] - data.input.figure.vertices[u]).abs2();
				let after = (out[v] - out[u]).abs2();
				if (after * 1000000 - before * 1000000).abs() > data.input.epsilon * before {
					ok = false;
					break;
				}
			}
		}
		if ok {
			for &v in &data.g[u] {
				if used[v] && !P::contains_s(&data.input.hole, (out[u], out[v])) {
					ok = false;
					break;
				}
			}
			if ok {
				rec(data, i + 1, order, out, used, best, best_score, until);
			}
		}
	}
	used[u] = false;
}

fn main() {
	let input = read_input();
	let n = input.figure.vertices.len();
	let mut g = vec![vec![]; n];
	for &(i, j) in &input.figure.edges {
		g[i].push(j);
		g[j].push(i);
	}
	let mut order = vec![];
	let mut used = vec![false; n];
	let mut parent = vec![!0; n];
	for _ in 0..n {
		let mut max = (0, 0);
		let mut max_i = 0;
		for i in 0..n {
			if !used[i] {
				let mut count = 0;
				for &j in &g[i] {
					if used[j] {
						count += 1;
					}
				}
				if max.setmax((count, g[i].len())) {
					max_i = i;
				}
			}
		}
		order.push(max_i);
		used[max_i] = true;
		if parent[max_i] == !0 {
			parent[max_i] = !1;
		}
		for &j in &g[max_i] {
			if parent[j] == !0 {
				parent[j] = max_i;
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
	let mut data = Data { input, inside, g, parent, cand: vec![] };
	let mut best = vec![];
	let mut best_score = i64::max_value();
	for _ in 0..20 {
		eprintln!("eps = {}", data.input.epsilon);
		let mut cand = vec![vec![]; n];
		for i in 0..n {
			let r = data.parent[i];
			if r < n {
				let orig = (data.input.figure.vertices[r] - data.input.figure.vertices[i]).abs2();
				for dx in -(max_x - min_x) ..= (max_x - min_x) {
					for dy in -(max_y - min_y) ..= (max_y - min_y) {
						if (P(dx, dy).abs2() * 1000000 - orig * 1000000).abs() <= data.input.epsilon * orig {
							cand[i].push(P(dx, dy));
						}
					}
				}
			}
		}
		data.cand = cand;
		let stime = get_time();
		for x in min_x ..= max_x {
			for y in min_y ..= max_y {
				if data.inside[x as usize][y as usize] {
					let mut out = vec![P(x, y); n];
					let mut used = vec![false; n];
					used[order[0]] = true;
					rec(&data, 1, &order, &mut out, &mut used, &mut best, &mut best_score, stime + 10.0);
				}
			}
		}
		if data.input.epsilon == 0 {
			break;
		}
		data.input.epsilon /= 2;
	}
	eprintln!("Score = {}", best_score);
	write_output(&Output { vertices: best });
}
