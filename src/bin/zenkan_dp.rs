use icfpc2021::{*, util::*};
use std::collections::*;

#[derive(Debug)]
enum Node {
	Leaf(usize),
	Forget(Vec<usize>, usize, Box<Node>),
	Introduce(Vec<usize>, usize, Box<Node>),
	Join(Vec<usize>, Box<Node>, Box<Node>),
}

fn construct_rec(bags: &Vec<Vec<usize>>, g: &Vec<Vec<usize>>, i: usize, parent: usize) -> Node {
	let mut child = vec![];
	for &j in &g[i] {
		if j != parent {
			child.push(j);
		}
	}
	if child.len() == 0 {
		let mut node = Node::Leaf(bags[i][0]);
		for k in 1..bags[i].len() {
			node = Node::Introduce(bags[i][0..=k].iter().cloned().collect(), bags[i][k], Box::new(node));
		}
		node
	} else {
		let mut nodes = child.iter().map(|&c| {
			let mut node = construct_rec(bags, g, c, i);
			let mut bag = bags[c].clone();
			for u in &bags[c] {
				if !bags[i].contains(u) {
					let p = bag.iter().position(|a| a == u).unwrap();
					bag.remove(p);
					node = Node::Forget(bag.clone(), *u, Box::new(node));
				}
			}
			for u in &bags[i] {
				if !bags[c].contains(u) {
					bag.push(*u);
					bag.sort();
					node = Node::Introduce(bag.clone(), *u, Box::new(node));
				}
			}
			node
		}).collect::<Vec<_>>();
		if child.len() == 1 {
			nodes.pop().unwrap()
		} else {
			let mut node = Node::Join(bags[i].clone(), Box::new(nodes.pop().unwrap()), Box::new(nodes.pop().unwrap()));
			for _ in 2..child.len() {
				node = Node::Join(bags[i].clone(), Box::new(node), Box::new(nodes.pop().unwrap()));
			}
			node
		}
	}
}

fn construct_nice_td(bags: &Vec<Vec<usize>>, es: &Vec<(usize, usize)>) -> Node {
	let n = bags.len();
	let mut g = vec![vec![]; n];
	for &(i, j) in es {
		g[i].push(j);
		g[j].push(i);
	}
	let mut r = 0;
	for i in 0..n {
		if bags[r].len() < bags[i].len() {
			r = i;
		}
	}
	let mut node = construct_rec(bags, &g, r, !0);
	for k in (0..bags[r].len()).rev() {
		node = Node::Forget(bags[r][0..k].iter().cloned().collect(), bags[r][k], Box::new(node));
	}
	node
}

#[derive(Debug)]
struct Data {
	input: Input,
	dist: Vec<Vec<f64>>,
	g: Vec<Vec<usize>>,
	inside: Vec<Vec<bool>>,
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

const UB: usize = 10000;

fn dp(data: &Data, node: Node) -> BTreeMap<(Vec<Point>, u128), Vec<Point>> {
	match node {
		Node::Leaf(u) => {
			let mut ret = BTreeMap::new();
			for x in 0..data.inside.len() {
				for y in 0..data.inside[x].len() {
					if data.inside[x][y] {
						let p = P(x as i64, y as i64);
						let mut out = vec![P(-1, -1); data.g.len()];
						out[u] = p;
						let mut used = 0;
						for i in 0..data.input.hole.len() {
							if data.input.hole[i] == p {
								used |= 1 << i;
							}
						}
						ret.insert((vec![p], used), out);
					}
				}
			}
			dbg!(ret.len());
			ret
		},
		Node::Forget(bag, u, c) => {
			let mut ret = BTreeMap::new();
			let i = bag.binary_search(&u).err().unwrap();
			for ((mut ps, used), out) in dp(data, *c) {
				ps.remove(i);
				ret.insert((ps, used), out);
			}
			dbg!(ret.len(), bag);
			if ret.len() > UB {
				let mut list = ret.into_iter().collect::<Vec<_>>();
				list.sort_by_key(|a| 10000 - u128::count_ones(a.0.1));
				list.truncate(UB);
				ret = list.into_iter().collect();
			}
			ret
		},
		Node::Introduce(bag, u, c) => {
			let i = bag.iter().position(|&v| v == u).unwrap();
			let cr = dp(data, *c);
			let mut ret = BTreeMap::new();
			if let Some(&r) = bag.iter().find(|&r| data.g[u].contains(&r)) {
				for &d in &data.cand[u][r] {
					for ((ps, used), out) in &cr {
						let p = out[r] + d;
						if can_place(data, &out, u, p) {
							let mut ps = ps.clone();
							ps.insert(i, p);
							let mut out = out.clone();
							out[u] = p;
							let mut used = *used;
							for i in 0..data.input.hole.len() {
								if data.input.hole[i] == p {
									used |= 1 << i;
								}
							}
							ret.insert((ps, used), out);
						}
					}
				}
			} else {
				eprintln!("orz");
				for x in 0..data.inside.len() {
					for y in 0..data.inside[x].len() {
						let p = P(x as i64, y as i64);
						if data.inside[x][y] {
							for ((ps, used), out) in &cr {
								if can_place(data, &out, u, p) {
									let mut ps = ps.clone();
									ps.insert(i, p);
									let mut out = out.clone();
									out[u] = p;
									let mut used = *used;
									for i in 0..data.input.hole.len() {
										if data.input.hole[i] == p {
											used |= 1 << i;
										}
									}
									ret.insert((ps, used), out);
								}
							}
						}
					}
				}
			}
			if ret.len() > UB {
				let mut list = ret.into_iter().collect::<Vec<_>>();
				list.sort_by_key(|a| 10000 - u128::count_ones(a.0.1));
				list.truncate(UB);
				ret = list.into_iter().collect();
			}
			dbg!(ret.len(), bag);
			ret
		},
		Node::Join(_bag, l, r) => {
			let mut cr = BTreeMap::new();
			for ((ps, used), out) in dp(data, *r) {
				cr.entry(ps).or_insert(vec![]).push((used, out));
			}
			let mut ret = BTreeMap::new();
			for ((ps, used), out) in dp(data, *l) {
				for (used2, out2) in cr.get(&ps).unwrap_or(&vec![]) {
					let mut out = out.clone();
					for i in 0..out.len() {
						if out[i].0 < 0 && out2[i].0 >= 0 {
							out[i] = out2[i];
						}
					}
					ret.insert((ps.clone(), used | used2), out);
				}
			}
			if ret.len() > UB {
				let mut list = ret.into_iter().collect::<Vec<_>>();
				list.sort_by_key(|a| 10000 - u128::count_ones(a.0.1));
				list.truncate(UB);
				ret = list.into_iter().collect();
			}
			dbg!(ret.len(), _bag);
			ret
		}
	}
}

fn main() {
	let input = read_input();
	eprintln!("hole = {}", input.hole.len());
	let n = input.figure.vertices.len();
	let td = tree_decomposition::read_tree_decomposition(&format!("tree_decomposition/{}.txt", std::env::args().nth(1).unwrap()));
	let mut bags = td.bag_vs;
	for bag in &mut bags {
		bag.sort();
	}
	let es = td.es;
	// let (bags, es) = compute_td(&input);
	let root = construct_nice_td(&bags, &es);
	// dbg!(&root);
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
	let data = Data { input, dist, g, inside, cand };
	let ret = dp(&data, root);
	if let Some(out) = ret.get(&(vec![], (1 << data.input.hole.len()) - 1)) {
		let mut out = out.clone();
		for i in 0..n {
			if out[i].0 < 0 {
				out[i] = P((min_x + max_x) / 2, (min_y + max_y) / 2);
			}
		}
		write_output(&Output { vertices: out });
	} else {
		eprintln!("orz");
		write_output(&Output { vertices: vec![] });
	}
}
