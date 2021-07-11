use crate::*;
use rand;
use rand::*;

const DIRS: [Point; 4] = [
	P::<i64>(1, 0),
	P::<i64>(0, 1),
	P::<i64>(-1, 0),
	P::<i64>(0, -1),
];

pub fn ugougo(problem: &Input, pose: &Output, cycles: i32) -> (Output, i32) {
	let Input {
		hole,
		figure: Figure {
			edges,
			vertices: original_vertices,
		},
		epsilon,
		..
	} = problem;
	let Output {
		mut vertices,
		bonuses,
	} = pose.clone();

	let globalist = bonuses
		.iter()
		.find(|&b| b.bonus == BonusType::Globalist)
		.is_some();

	let n = original_vertices.len();
	assert_eq!(n, vertices.len());

	let dist2: Vec<_> = edges
		.iter()
		.map(|&(a, b)| (original_vertices[a] - original_vertices[b]).abs2())
		.collect();

	// assert!(vertices
	// 	.iter()
	// 	.all(|&v| !P::contains_p(hole, v).is_negative()));
	// assert!(edges
	// 	.iter()
	// 	.map(|&(a, b)| (vertices[a], vertices[b]))
	// 	.enumerate()
	// 	.all(|(i, d)| P::contains_s(hole, d)
	// 		&& stretch_within((d.0 - d.1).abs2(), dist2[i], *epsilon) == Ordering::Equal));

	let mut adj = vec![vec![]; n];
	for i in 0..edges.len() {
		let (a, b) = edges[i];
		adj[a].push((b, i));
		adj[b].push((a, i));
	}

	let mut score = compute_score(problem, &pose);

	let mut rng = rand::thread_rng();
	let mut k = 0;
	for _ in 0..cycles {
		let a = rng.gen_range(0..n);
		let d = DIRS[rng.gen_range(0..4)];

		let penalty = if check_constraints_around_vertex(
			hole, edges, &vertices, &dist2, a, &adj[a], *epsilon, globalist,
		) {
			0
		} else {
			calculate_penalty(
				hole, edges, &vertices, &dist2, a, &adj[a], *epsilon, globalist,
			)
		};
		vertices[a] += d; // destructive
		let ok = if penalty == 0 {
			check_constraints_around_vertex(
				hole, edges, &vertices, &dist2, a, &adj[a], *epsilon, globalist,
			)
		} else {
			calculate_penalty(
				hole, edges, &vertices, &dist2, a, &adj[a], *epsilon, globalist,
			) <= penalty
		};
		if ok {
			let new_score = compute_score(
				problem,
				&Output {
					vertices: vertices.clone(),
					bonuses: Vec::new(),
				},
			);
			if new_score <= score {
				score = new_score;
				k += 1;
				continue; // accept change
			}
		}
		vertices[a] -= d; // revert
	}

	(Output { vertices, bonuses }, k)
}

fn check_constraints_around_vertex(
	hole: &Vec<Point>,
	edges: &Vec<(usize, usize)>,
	vertices: &Vec<Point>,
	dist2: &Vec<i64>,
	a: usize,
	adj: &[(usize, usize)],
	epsilon: i64,
	globalist: bool,
) -> bool {
	!P::contains_p(hole, vertices[a]).is_negative()
		&& if globalist {
			let prod: i64 = dist2.iter().product();
			assert!(prod > 0);
			let prod_sum: i64 = edges
				.iter()
				.enumerate()
				.map(|(i, &(a, b))| {
					prod / dist2[i] * ((vertices[a] - vertices[b]).abs2() - dist2[i])
				})
				.sum();
			1000000 * prod_sum <= prod * edges.len() as i64 * epsilon
		} else {
			adj.iter()
				.map(|&(b, i)| (i, (vertices[a], vertices[b])))
				.all(|(i, d)| {
					P::contains_s(hole, d)
						&& stretch_within((d.0 - d.1).abs2(), dist2[i], epsilon) == Ordering::Equal
				})
		}
}

fn calculate_penalty(
	hole: &Vec<Point>,
	edges: &Vec<(usize, usize)>,
	vertices: &Vec<Point>,
	dist2: &Vec<i64>,
	a: usize,
	adj: &[(usize, usize)],
	epsilon: i64,
	globalist: bool,
) -> i64 {
	let mut penalty = 0;
	if P::contains_p(hole, vertices[a]).is_negative() {
		penalty += 1000;
	}
	adj.iter()
		.map(|&(b, i)| (i, (vertices[a], vertices[b])))
		.for_each(|(i, d)| {
			if !P::contains_s(hole, d) {
				penalty += 1000
			}
			penalty += std::cmp::max(0, ((d.0 - d.1).abs2() - dist2[i]).abs() * 1000000 - epsilon * dist2[i]);
		});
	return penalty;
}
