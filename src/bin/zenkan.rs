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

fn can_place(data: &Data, out: &Vec<Point>, used: &Vec<bool>, u: usize, p: Point) -> bool {
	if p.0 < 0 || p.0 >= data.inside.len() as i64 || p.1 < 0 || p.1 >= data.inside[0].len() as i64 || !data.inside[p.0 as usize][p.1 as usize] {
		return false;
	}
	for &v in &data.g[u] {
		if !used[v] {
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
	for v in 0..used.len() {
		if !used[v] {
			continue;
		}
		let dist = ((p - out[v]).abs2() as f64).sqrt();
		if data.dist[u][v] as f64 * mul_ub < dist - 1e-4 {
			return false;
		}
	}
	true
}

const ZENKAN: bool = true;

fn rec(data: &Data, i: usize, out: &mut Vec<Point>, used: &mut Vec<bool>, cand: &Vec<Option<Vec<Point>>>, min: &Vec<i64>, best: &mut Vec<Point>, best_score: &mut i64, until: f64) {
	let n = out.len();
	if ZENKAN && n - i < min.iter().filter(|&&v| v > 0).count() {
		return;
	}
	if i == n {
		if best_score.setmin(min.iter().sum()) {
			eprintln!("{:.3}: {}", get_time(), best_score);
			*best = out.clone();
		}
		return;
	}
	if get_time() > until || *best_score == 0 {
		return;
	}
	let mut next = vec![];
	let mut min_size = 1 << 30;
	if ZENKAN {
		for i in 0..data.input.hole.len() {
			if min[i] == 0 {
				continue;
			}
			let mut vs = vec![];
			for v in 0..n {
				if !used[v] && can_place(data, out, used, v, data.input.hole[i]) {
					vs.push((v, data.input.hole[i]));
				}
			}
			if vs.len() == 0 {
				return;
			}
			if min_size.setmin(vs.len()) {
				next = vs;
			}
		}
	}
	for u in 0..n {
		if used[u] {
			continue;
		}
		if let Some(ps) = &cand[u] {
			if min_size.setmin(ps.len()) {
				next = ps.iter().map(|&p| (u, p)).collect();
			}
		}
	}
	let mut list = vec![];
	for (v, p) in next {
		let mut min = min.clone();
		for h in 0..data.input.hole.len() {
			min[h].setmin((p - data.input.hole[h]).abs2());
		}
		list.push((min, v, p));
	}
	if !ZENKAN {
		list.sort_by_key(|(min, _, _)| min.iter().sum::<i64>());
	}
	for (min, v, p) in list {
		out[v] = p;
		used[v] = true;
		let mut cand = cand.clone();
		for &u in &data.g[v] {
			if !used[u] {
				let list = cand[u].clone().unwrap_or(data.cand[u][v].iter().map(|&d| p + d).collect());
				cand[u] = Some(list.into_iter().filter(|&p| can_place(data, &out, &used, u, p)).collect());
			}
		}
		rec(data, i + 1, out, used, &cand, &min, best, best_score, until);
		used[v] = false;
		out[v] = P(-1, -1);
	}
}

fn main() {
	let input = read_input();
	let n = input.figure.vertices.len();
	eprintln!("n = {}, m = {}", n, input.figure.edges.len());
	let mut g = vec![vec![]; n];
	let mut dist = mat![1e20; n; n];
	for &(i, j) in &input.figure.edges {
		g[i].push(j);
		g[j].push(i);
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
	let mut data = Data { input, dist, inside, g, cand: vec![] };
	let mut best = vec![];
	let mut best_score = i64::max_value();
	for _ in 0..1 {
		eprintln!("eps = {}", data.input.epsilon);
		let mut cand = mat![vec![]; n; n];
		for i in 0..n {
			for &r in &data.g[i] {
				let orig = (data.input.figure.vertices[r] - data.input.figure.vertices[i]).abs2();
				for dx in -(max_x - min_x) ..= (max_x - min_x) {
					for dy in -(max_y - min_y) ..= (max_y - min_y) {
						if (P(dx, dy).abs2() * 1000000 - orig * 1000000).abs() <= data.input.epsilon * orig {
							cand[i][r].push(P(dx, dy));
						}
					}
				}
			}
		}
		data.cand = cand;
		for u in 0..n {
			let stime = get_time();
			let mut out = vec![P(-1, -1); n]; // 極小エラーを許す場合はここも他の候補試す必要あり
			let mut used = vec![false; n];
			out[u] = data.input.hole[0];
			used[u] = true;
			let mut min = vec![0; data.input.hole.len()];
			for i in 0..data.input.hole.len() {
				min[i] = (out[u] - data.input.hole[i]).abs2();
			}
			let mut cand = vec![None; n];
			for &v in &data.g[u] {
				let mut list = vec![];
				for &d in &data.cand[v][u] {
					let p = out[u] + d;
					if P::contains_p(&data.input.hole, p) >= 0 && P::contains_s(&data.input.hole, (out[u], p)) {
						list.push(p);
					}
				}
				cand[v] = Some(list);
			}
			rec(&data, 1, &mut out, &mut used, &cand, &min, &mut best, &mut best_score, stime + 300.0);
		}
		if data.input.epsilon == 0 {
			break;
		}
		data.input.epsilon /= 4;
	}
	eprintln!("Score = {}", best_score);
	write_output(&Output { vertices: best, bonuses: Default::default() });
}
