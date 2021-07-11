use icfpc2021::{*, util::*};

pub fn get_time() -> f64 {
	static mut STIME: f64 = -1.0;
	let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
	unsafe {
		if STIME < 0.0 {
			STIME = ms;
		}
		(ms - STIME) / 5.0
	}
}

struct Data {
	input: Input,
	dist: Vec<Vec<f64>>,
	inside: Vec<Vec<bool>>,
	g: Vec<Vec<usize>>,
	parent: Vec<usize>,
	last: Vec<bool>,
	cand: Vec<Vec<Point>>,
}

const BEST_SEARCH: bool = true;

fn rec(data: &Data, i: usize, order: &Vec<usize>, out: &mut Vec<Point>, used: &mut Vec<bool>, min: &Vec<i64>, best: &mut Vec<Point>, best_score: &mut i64, until: f64) {
	if i == order.len() {
		if best_score.setmin(min.iter().sum()) {
			eprintln!("{:.3}: {}", get_time(), best_score);
			*best = out.clone();
		}
		return;
	}
	if get_time() > until || *best_score == 0 {
		return;
	}
	let u = order[i];
	used[u] = true;
	let r = out[data.parent[u]];
	let mut cand = vec![];
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
				let mul_ub = (1.0 + data.input.epsilon as f64 * 1e-6).sqrt();
				for v in 0..order.len() {
					if used[v] {
						let dist = ((out[u] - out[v]).abs2() as f64).sqrt();
						if data.dist[u][v] as f64 * mul_ub < dist - 1e-4 {
							ok = false;
							break;
						}
					}
				}
				if ok {
					let mut min = min.clone();
					for i in 0..data.input.hole.len() {
						min[i].setmin((out[u] - data.input.hole[i]).abs2());
					}
					cand.push((min, r + d));
				}
			}
		}
	}
	if BEST_SEARCH {
		cand.sort_by_key(|(min, _)| min.iter().sum::<i64>());
	} else {
		cand.sort();
	}
	let mut mins: Vec<Vec<i64>> = vec![];
	for (min, p) in cand {
		out[u] = p;
		let mut ok = true;
		if data.last[u] {
			for min2 in &mins {
				if (0..data.input.hole.len()).all(|i| min2[i] <= min[i]) {
					ok = false;
					break;
				}
			}
		}
		if !ok {
			break;
		}
		rec(data, i + 1, order, out, used, &min, best, best_score, until);
		mins.push(min);
	}
	used[u] = false;
}

fn main() {
    let no_search = match std::env::var("WATA_NO_SEARCH") {
        Ok(value) => match value.parse::<i64>() {
            Ok(value) => value != 0,
            Err(_) => false,
        },
        Err(_) => false,
    };

	let input = read_input();
	let n = input.figure.vertices.len();
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
	let mut order = vec![];
	let mut used = vec![false; n];
	let mut parent = vec![!0; n];
	let mut last = vec![true; n];
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
			if !used[j] {
				last[max_i] = false;
			}
		}
	}
	let mut n2 = 0;
	for i in 0..n {
		if !last[i] {
			n2 += 1;
		}
	}
	eprintln!("n = {}, n2 = {}", n, n2);
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
	let mut data = Data { input, dist, inside, g, parent, last, cand: vec![] };
	let mut best = vec![];
	let mut best_score = i64::max_value();
	for _ in 0..5 {
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
		let mut ps = vec![];
		for x in min_x ..= max_x {
			for y in min_y ..= max_y {
				if data.inside[x as usize][y as usize] {
					ps.push(P(x, y));
				}
			}
		}
		use rand::prelude::*;
		ps.shuffle(&mut rand::thread_rng());
		let p = ps[0];
		let stime = get_time();
		let mut out = vec![p; n];
		let mut used = vec![false; n];
		let mut min = vec![0; data.input.hole.len()];
		for i in 0..data.input.hole.len() {
			min[i] = (p - data.input.hole[i]).abs2();
		}
		used[order[0]] = true;
		rec(&data, 1, &order, &mut out, &mut used, &min, &mut best, &mut best_score, stime + 10.0);

        // もし何かが見つかったら直ちに終了する
        if no_search && best.len() > 0 {
            break;
        }

		// if best.len() > 0 {
		// 	let stime = get_time();
		// 	let x = best[order[0]].0;
		// 	let y = best[order[0]].1;
		// 	let mut out = vec![P(x, y); n];
		// 	let mut used = vec![false; n];
		// 	let mut min = vec![0; data.input.hole.len()];
		// 	for i in 0..data.input.hole.len() {
		// 		min[i] = (P(x, y) - data.input.hole[i]).abs2();
		// 	}
		// 	used[order[0]] = true;
		// 	rec(&data, 1, &order, &mut out, &mut used, &min, &mut best, &mut best_score, stime + 10.0);
		// 	if stime + 10.0 > get_time() {
		// 		break;
		// 	}
		// }
		if data.input.epsilon == 0 {
			break;
		}
		data.input.epsilon /= 2;
	}
	eprintln!("Score = {}", best_score);
	write_output(&Output { vertices: best, bonuses: Vec::new() });
}
