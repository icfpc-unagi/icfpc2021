use icfpc2021::{*, util::*};
use td::compute_td;
use std::collections::*;

mod td {
	
	use std::cell::Cell;
	use icfpc2021::{*, util::*};

	#[derive(Clone, Debug)]
	pub struct UnionFind {
		/// size / parent
		ps: Vec<Cell<usize>>,
		pub is_root: Vec<bool>
	}
	
	impl UnionFind {
		pub fn new(n: usize) -> UnionFind {
			UnionFind { ps: vec![Cell::new(1); n], is_root: vec![true; n] }
		}
		pub fn find(&self, x: usize) -> usize {
			if self.is_root[x] { x }
			else {
				let p = self.find(self.ps[x].get());
				self.ps[x].set(p);
				p
			}
		}
		pub fn unite(&mut self, x: usize, y: usize) {
			let mut x = self.find(x);
			let mut y = self.find(y);
			if x == y { return }
			if self.ps[x].get() < self.ps[y].get() {
				::std::mem::swap(&mut x, &mut y);
			}
			*self.ps[x].get_mut() += self.ps[y].get();
			self.ps[y].set(x);
			self.is_root[y] = false;
		}
		pub fn same(&self, x: usize, y: usize) -> bool {
			self.find(x) == self.find(y)
		}
		pub fn size(&self, x: usize) -> usize {
			self.ps[self.find(x)].get()
		}
	}
	
	pub struct TreeDecomposition {
		/// leaf to root
		pub order: Vec<usize>,
		/// N(T(u))
		pub bag: Vec<Vec<usize>>,
		/// parent[u] = !0 when u is a root
		pub parent: Vec<usize>
	}
	
	impl TreeDecomposition {
		
		pub fn from_ordering(g: &Vec<Vec<usize>>, order: Vec<usize>, construct_bag: bool) -> TreeDecomposition {
			let n = g.len();
			let mut uf = UnionFind::new(n);
			let mut bag = vec![vec![]; n];
			let mut parent = vec![!0; n];
			let mut root: Vec<_> = (0..n).collect();
			let mut eliminated = vec![false; n];
			for &u in &order {
				let mut tmp = vec![];
				for &v in &g[u] {
					if uf.same(u, v) {
					} else if !eliminated[v] {
						tmp.push(v);
					} else {
						let v = root[uf.find(v)];
						parent[v] = u;
						uf.unite(u, v);
						root[uf.find(u)] = u;
						if construct_bag {
							tmp.extend(bag[v].iter().filter(|&&w| w != u));
						}
					}
				}
				eliminated[u] = true;
				if construct_bag {
					tmp.sort();
					tmp.dedup();
					bag[u] = tmp;
				}
			}
			TreeDecomposition { order, bag, parent }
		}
		
	}
	
	pub fn degree_order(g: &Vec<Vec<usize>>) -> Vec<usize> {
		let n = g.len();
		let mut order: Vec<_> = (0..n).collect();
		order.sort_by(|&i, &j| g[i].len().cmp(&g[j].len()));
		order
	}
	
	pub fn degree_order2(g: &Vec<Vec<usize>>) -> Vec<usize> {
		let n = g.len();
		let mut order = vec![];
		let mut eliminated = vec![false; n];
		let mut que = std::collections::BinaryHeap::new();
		let mut ws = vec![0; n];
		for i in 0..n {
			que.push((!0, !g[i].len(), i));
		}
		while let Some((w, _, u)) = que.pop() {
			if eliminated[u] || w != !ws[u] {
				continue;
			}
			order.push(u);
			eliminated[u] = true;
			for &v in &g[u] {
				if !eliminated[v] {
					ws[v] += 1;
					que.push((!ws[v], !g[v].len(), v));
				}
			}
		}
		order
	}
	
	pub fn degree_order3(g: &Vec<Vec<usize>>) -> Vec<usize> {
		let n = g.len();
		let mut que = std::collections::BinaryHeap::new();
		let mut deg = vec![0; n];
		for i in 0..n {
			deg[i] = g[i].len();
			que.push((deg[i], i));
		}
		let mut eliminated = vec![false; n];
		let mut order = vec![];
		while let Some((d, u)) = que.pop() {
			if eliminated[u] || deg[u] != d {
				continue;
			}
			order.push(u);
			eliminated[u] = true;
			for &v in &g[u] {
				if !eliminated[v] {
					deg[v] -= 1;
					que.push((deg[v], v));
				}
			}
		}
		order.into_iter().rev().collect()
	}
	
	pub fn k_core_order(g: &Vec<Vec<usize>>) -> Vec<usize> {
		let n = g.len();
		let mut que = std::collections::BinaryHeap::new();
		let mut deg = vec![0; n];
		for i in 0..n {
			deg[i] = g[i].len();
			que.push((!deg[i], i));
		}
		let mut eliminated = vec![false; n];
		let mut order = vec![];
		while let Some((_, u)) = que.pop() {
			if eliminated[u] {
				continue;
			}
			order.push(u);
			eliminated[u] = true;
			for &v in &g[u] {
				if !eliminated[v] {
					deg[v] -= 1;
					que.push((!deg[v], v));
				}
			}
		}
		order
	}
	
	pub fn compute_td(input: &Input) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
		let n = input.figure.vertices.len();
		let mut g = vec![vec![]; n];
		for &(i, j) in &input.figure.edges {
			g[i].push(j);
			g[j].push(i);
		}
		let mut min = n;
		let mut best = None;
		for iter in 0..4 {
			let order = match iter {
				0 => degree_order(&g),
				1 => degree_order2(&g),
				2 => degree_order3(&g),
				3 => k_core_order(&g),
				_ => unreachable!()
			};
			let td = TreeDecomposition::from_ordering(&g, order, true);
			let mut width = 0;
			for b in &td.bag {
				width.setmax(b.len());
			}
			if min.setmin(width) {
				best = Some(td);
			}
		}
		let td = best.unwrap();
		let mut bags = vec![];
		let mut es = vec![];
		for i in 0..n {
			let mut tmp = td.bag[i].clone();
			tmp.push(i);
			tmp.sort();
			bags.push(tmp);
			if td.parent[i] != !0 {
				es.push((i, td.parent[i]));
			}
		}
		(bags, es)
	}
}

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

struct Data {
	input: Input,
	dist: Vec<Vec<f64>>,
	g: Vec<Vec<usize>>,
	inside: Vec<Vec<bool>>,
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

fn dp(data: &Data, node: Node) -> BTreeSet<(Vec<Point>, Vec<usize>)> {
	match node {
		Node::Leaf(u) => {
			let mut ret = BTreeSet::new();
			for x in 0..data.inside.len() {
				for y in 0..data.inside[x].len() {
					if data.inside[x][y] {
						let p = P(x as i64, y as i64);
						let mut used = vec![];
						for s in 0..data.input.hole.len() {
							if data.input.hole[s] == p {
								used.push(s);
							}
						}
						ret.insert((vec![p], used));
					}
				}
			}
			ret
		},
		Node::Forget(bag, u, c) => {
			let mut ret = BTreeSet::new();
			let i = bag.binary_search(&u).err().unwrap();
			for (mut ps, used) in dp(data, *c) {
				ps.remove(i);
				ret.insert((ps, used));
			}
			ret
		},
		Node::Introduce(bag, u, c) => {
			let i = bag.iter().position(|&v| v == u).unwrap();
			let cr = dp(data, *c);
			let mut out = vec![P(-1, -1); data.cand.len()];
			let mut used_v = vec![false; data.cand.len()];
			for &b in &bag {
				used_v[b] = true;
			}
			let mut ret = BTreeSet::new();
			if let Some(r) = bag.iter().position(|&r| data.g[u].contains(&r)) {
				for &d in &data.cand[u][bag[r]] {
					for (ps, used) in &cr {
						for j in 0..bag.len() {
							if j < i {
								out[bag[j]] = ps[j];
							} else if j > i {
								out[bag[j]] = ps[j - 1];
							}
						}
						if can_place(data, &out, &used_v, u, ps[r] + d) {
							let mut ps = ps.clone();
							let p = ps[r] + d;
							ps.insert(i, p);
							let mut used = used.clone();
							for s in 0..data.input.hole.len() {
								if data.input.hole[s] == p {
									used.push(s);
									used.sort();
								}
							}
							ret.insert((ps, used));
						}
					}
				}
			} else {
				eprintln!("orz");
				for x in 0..data.inside.len() {
					for y in 0..data.inside[x].len() {
						let p = P(x as i64, y as i64);
						if data.inside[x][y] {
							for (ps, used) in &cr {
								for j in 0..bag.len() {
									if j < i {
										out[bag[j]] = ps[j];
									} else if j > i {
										out[bag[j]] = ps[j - 1];
									}
								}
								if can_place(data, &out, &used_v, u, p) {
									let mut ps = ps.clone();
									ps.insert(i, p);
									let mut used = used.clone();
									for s in 0..data.input.hole.len() {
										if data.input.hole[s] == p {
											used.push(s);
											used.sort();
										}
									}
									ret.insert((ps, used));
								}
							}
						}
					}
				}
			}
			ret
		},
		Node::Join(_bag, l, r) => {
			let mut cr = BTreeMap::new();
			for (ps, used) in dp(data, *r) {
				cr.entry(ps).or_insert(vec![]).push(used);
			}
			let mut ret = BTreeSet::new();
			for (ps, used) in dp(data, *l) {
				for used2 in cr.get(&ps).unwrap_or(&vec![]) {
					let mut used = used.clone();
					used.extend(used2);
					used.sort();
					used.dedup();
					ret.insert((ps.clone(), used));
				}
			}
			ret
		}
	}
}

fn main() {
	let input = read_input();
	let n = input.figure.vertices.len();
	let td = tree_decomposition::read_tree_decomposition(&format!("tree_decomposition/{}.txt", std::env::args().nth(1).unwrap()));
	let bags = td.bag_vs;
	let es = td.es;
	// let (bags, es) = compute_td(&input);
	let root = construct_nice_td(&bags, &es);
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
	let min = dp(&data, root);
	dbg!(min);
}
