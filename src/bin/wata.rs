use icfpc2021::{*, util::*};

const INF: i64 = 1 << 50;

struct Data {
	input: Input,
	min_x: i64,
	max_x: i64,
	min_y: i64,
	max_y: i64,
	g: Vec<Vec<usize>>,
	parent: Vec<usize>,
	cand: Vec<Vec<Point>>,
}

fn rec(data: &Data, i: usize, order: &Vec<usize>, out: &mut Vec<Point>, used: &mut Vec<bool>, count: &mut usize) {
	if i == order.len() {
		*count += 1;
		return;
	}
	let u = order[i];
	used[u] = true;
	let r = out[data.parent[u]];
	for &d in &data.cand[u] {
		out[u] = r + d;
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
			rec(data, i + 1, order, out, used, count);
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
	dbg!(&order);
	let min_x = input.hole.iter().map(|p| p.0).min().unwrap();
	let max_x = input.hole.iter().map(|p| p.0).max().unwrap();
	let min_y = input.hole.iter().map(|p| p.1).min().unwrap();
	let max_y = input.hole.iter().map(|p| p.1).max().unwrap();
	let mut cand = vec![vec![]; n];
	for i in 0..n {
		let r = parent[i];
		if r < n {
			let orig = (input.figure.vertices[r] - input.figure.vertices[i]).abs2();
			for dx in -(max_x - min_x) ..= (max_x - min_x) {
				for dy in -(max_y - min_y) ..= (max_y - min_y) {
					if (P(dx, dy).abs2() * 1000000 - orig * 1000000).abs() <= input.epsilon * orig {
						cand[i].push(P(dx, dy));
					}
				}
			}
		}
	}
	let data = Data { input, min_x, max_x, min_y, max_y, g, parent, cand };
	let mut out = vec![P(0, 0); n];
	let mut used = vec![false; n];
	used[order[0]] = true;
	let mut count = 0;
	rec(&data, 1, &order, &mut out, &mut used, &mut count);
	dbg!(count);
	
	write_output(&Output { vertices: out });
}
