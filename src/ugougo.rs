use crate::*;
use rand;
use rand::*;

const DIRS: [Point; 4] = [
	P::<i64>(1, 0),
	P::<i64>(0, 1),
	P::<i64>(-1, 0),
	P::<i64>(0, -1),
];

pub fn ugougo(problem: &Input, pose: &Output, cycles: usize) -> (Output, i32) {
	let Input {
		hole,
		figure: Figure {
			edges,
			vertices: original_vertices,
		},
		epsilon,
		..
	} = problem;
	let Output { vertices, .. } = pose;
	let mut vertices = vertices.clone();

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

	let mut score = compute_score(problem, pose);

	let mut rng = rand::thread_rng();
	let mut k = 0;
	for _ in 0..cycles {
		let a = rng.gen_range(0..n);
		let d = DIRS[rng.gen_range(0..4)];
		vertices[a] += d; // destructive
		if !P::contains_p(hole, vertices[a]).is_negative()
			&& adj[a]
				.iter()
				.map(|&(b, i)| (i, (vertices[a], vertices[b])))
				.all(|(i, d)| {
					P::contains_s(hole, d)
						&& stretch_within((d.0 - d.1).abs2(), dist2[i], *epsilon) == Ordering::Equal
				}) {
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

	(Output {
		vertices: vertices,
		bonuses: Vec::new(),
	}, k)
}
