#![allow(non_snake_case)]

use icfpc2021::{*, compute_score, util::*};

use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, io::prelude::*};
use rand::prelude::*;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
struct Data {
	data: Vec<SubmissionData>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
struct SubmissionData {
	submission_id: String,
	problem_id: String,
	submission_bonuses: String,
	submission_obtained_bonuses: String,
	submission_estimated_score: String,
}

#[derive(Clone, Debug)]
struct Submission {
	submission_id: String,
	use_globalist: usize,
	get_globalist: BTreeSet<usize>,
	score: i64,
}

const N: usize = 132;

fn get_submission<'a>(submissions: &'a Vec<Submission>, use_globalist: usize, get_globalist: &BTreeSet<usize>) -> Option<&'a Submission> {
	for s in submissions {
		if (use_globalist != !0) ^ (s.use_globalist == !0) {
			if get_globalist == &s.get_globalist {
				return Some(s);
			}
		}
	}
	None
}

fn get_score(submissions: &Vec<Submission>, use_globalist: usize, get_globalist: &BTreeSet<usize>) -> i64 {
	if let Some(s) = get_submission(submissions, use_globalist, get_globalist) {
		s.score
	} else {
		-20000
	}
}

fn compute_total_score(submissions: &Vec<Vec<Submission>>, use_globalist: &Vec<usize>, get_globalist: &Vec<BTreeSet<usize>>) -> i64 {
	let mut score = 0;
	for i in 0..N {
		score += get_score(&submissions[i], use_globalist[i], &get_globalist[i]);
	}
	score
}

fn main() {
	let mut weights = vec![];
	for i in 1..=N {
		let input: Input = serde_json::from_reader(std::fs::File::open(&format!("problems/{}.json", i)).unwrap()).unwrap();
		let weight = 1000.0 * ((input.figure.vertices.len() * input.figure.edges.len() * input.hole.len()) as f64 / 6.0).log2();
		weights.push(weight);
	}
	let mut mins = vec![];
	for line in std::io::BufReader::new(std::fs::File::open("tmp/min.txt").unwrap()).lines() {
		let line = line.unwrap();
		if line.len() > 0 {
			mins.push(line.parse::<i64>().unwrap());
		}
	}
	assert_eq!(mins.len(), N);
	
	let data: serde_json::value::Value = serde_json::from_reader(std::fs::File::open("tmp/submissions.json").unwrap()).unwrap();
	let data = data.as_array().unwrap();
	let mut submissions: Vec<Vec<Submission>> = vec![vec![]; N];
	for data in data {
		if let Ok(data) = serde_json::from_value::<Data>(data.clone()) {
			for s in data.data {
				let problem = s.problem_id.parse::<usize>().unwrap() - 1;
				let score: i64 = s.submission_estimated_score.parse().unwrap();
				let mut use_globalist = !0;
				let mut get_globalist = BTreeSet::new();
				if s.submission_bonuses.len() > 0 {
					let use_: Vec<UseBonus> = serde_json::from_str(&s.submission_bonuses).unwrap();
					for b in use_ {
						if b.bonus == BonusType::Globalist {
							use_globalist = 0;
						}
					}
				}
				if s.submission_obtained_bonuses.len() > 0 {
					let get_: Vec<Bonus> = serde_json::from_str(&s.submission_obtained_bonuses).unwrap();
					for b in get_ {
						if b.bonus == BonusType::Globalist {
							get_globalist.insert(b.problem as usize - 1);
							assert!(b.problem as usize - 1 != problem);
						}
					}
				}
				let my_score = (weights[problem] * ((mins[problem].min(score) as f64 + 1.0) / (score as f64 + 1.0)).sqrt()).round() as i64;
				let enemy_score = (weights[problem] * ((mins[problem].min(score) as f64 + 1.0) / (mins[problem] as f64 + 1.0)).sqrt()).round() as i64;
				let s = Submission { submission_id: s.submission_id, use_globalist, get_globalist, score: my_score - enemy_score };
				let mut ok = false;
				for t in &mut submissions[problem] {
					if t.use_globalist == s.use_globalist && t.get_globalist == s.get_globalist {
						if t.score.setmax(s.score) {
							t.submission_id = s.submission_id.clone();
						}
						ok = true;
					}
				}
				if !ok {
					submissions[problem].push(s);
				}
			}
		}
	}
	let mut g = vec![vec![]; N];
	for i in 0..N {
		for s in &submissions[i] {
			for &j in &s.get_globalist {
				g[j].push(i);
			}
			mins[i].setmin(s.score);
		}
	}
	for i in 0..N {
		g[i].sort();
		g[i].dedup();
	}
	
	let mut use_global = vec![!0; N];
	let mut obtain_global = vec![BTreeSet::new(); N];
	let mut score = compute_total_score(&submissions, &use_global, &obtain_global);
	eprintln!("baseline: {}", score);
	let mut best = (use_global.clone(), obtain_global.clone());
	let mut best_score = score;
	const T0: f64 = 1e5;
	const T1: f64 = 1e-0;
	let mut T = T0;
	const TN: usize = 100000000;
	let mut rng = rand::thread_rng();
	
	for t in 0..TN {
		if TN & 0xff == 0 {
			let t = t as f64 / TN as f64;
			T = T0.powf(1.0 - t) * T1.powf(t);
		}
		let i = rng.gen_range(0..N);
		if g[i].len() > 0 {
			let mut j = rng.gen_range(0..=g[i].len());
			if j == g[i].len() {
				j = !0;
			} else {
				j = g[i][j];
			}
			if use_global[i] == j {
				continue;
			}
			let mut diff = get_score(&submissions[i], j, &obtain_global[i]);
			diff -= get_score(&submissions[i], use_global[i], &obtain_global[i]);
			if j != !0 {
				obtain_global[j].insert(i);
				diff += get_score(&submissions[j], use_global[j], &obtain_global[j]);
				obtain_global[j].remove(&i);
				diff -= get_score(&submissions[j], use_global[j], &obtain_global[j]);
			}
			assert_eq!(score, compute_total_score(&submissions, &use_global, &obtain_global));
			if use_global[i] != !0 {
				let k = use_global[i];
				obtain_global[k].remove(&i);
				diff += get_score(&submissions[k], use_global[k], &obtain_global[k]);
				obtain_global[k].insert(i);
				diff -= get_score(&submissions[k], use_global[k], &obtain_global[k]);
			}
			if diff >= 0 || rng.gen_bool(f64::exp(diff as f64 / T)) {
				score += diff;
				if j != !0 {
					obtain_global[j].insert(i);
				}
				if use_global[i] != !0 {
					let k = use_global[i];
					obtain_global[k].remove(&i);
				}
				use_global[i] = j;
				if best_score.setmax(score) {
					eprintln!("{}: {}", t, best_score);
					best = (use_global.clone(), obtain_global.clone());
				}
			}
		}
	}
	assert_eq!(score, compute_total_score(&submissions, &use_global, &obtain_global));
	let (use_global, obtain_global) = best;
	for i in 0..N {
		println!("{} {}", get_submission(&submissions[i], use_global[i], &obtain_global[i]).unwrap().submission_id, use_global[i] + 1);
		if use_global[i] != !0 {
			eprintln!("use {}", i + 1);
		}
		if obtain_global[i].len() > 0 {
			eprintln!("get {}", i + 1);
		}
	}
}
