use icfpc2021::{*, util::*};

use std::cell::Cell;

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

fn main() {
	let input = read_input();
	let n = input.figure.vertices.len();
	let mut g = vec![vec![]; n];
	for &(i, j) in &input.figure.edges {
		g[i].push(j);
		g[j].push(i);
	}
	let mut min = n;
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
		for b in td.bag {
			width.setmax(b.len());
		}
		min.setmin(width);
	}
	eprintln!("{}", min);
}
