use std::ops::Add;

use icfpc2021::*;

fn main() {
	let input = read_input();
	dbg!(&input);

	let n = input.figure.vertices.len();
	//let v = input.hole.len();
	let mut now = input.figure.vertices.clone();
	let eps = input.epsilon;

	let maxnum = 100;

	let vp = [P(1, 0), P(0, 1), P(-1, 0), P(0, -1), P(1, 1), P(1, -1), P(-1, -1), P(-1, 1)];

	let mut point_board = vec![vec![0.0; 100]; 100];

	for i in 0..n {
		now[i] = P(i as i64, i as i64);
	}

	/* 
	for y in 0..maxnum {
		for x in 0..maxnum {
			for k in 0.. v {
				//let p1 = input.hole[k] - P(y, x);
				//let p2 = input.hole[(k+1) % v] - P(y, x);
				
				if false {
					point_board[y as usize][x as usize] = 99999.0;
				}
			}
		}
	}
	*/

	for i in 0..300 {
		let mut flag = true;
		for y in 0..maxnum {
			for x in 0..maxnum {
				if point_board[y as usize][x as usize] > 100.0 {
					for k in 0..4 {
						let ny = y + vp[k].0;
						let nx = x + vp[k].1;
						if ny >= 0 && ny < maxnum && nx >= 0 && nx < maxnum && point_board[ny as usize][nx as usize] == i as f64{
							point_board[y as usize][x as usize] = (i + 1) as f64;
							flag = false;
						}
					}
				}
			}
		}
		if flag { break; }
	}

	for loopcnt in 0..10000{
		let target = loopcnt % n;
		let now_score = get_all_score(&input, &now, eps, &point_board);
		let move_type = (loopcnt / n + loopcnt * 3) % 8;
		now[target] = now[target] + vp[move_type];

		if now[target].0 < 0 || now[target].1 < 0 || now[target].0 >= maxnum || now[target].1 >= maxnum {
			now[target] = now[target].add(vp[(move_type + 4) % 8]);
			continue;
		}

		let next_score = get_all_score(&input, &now, eps, &point_board);
		if next_score < now_score {
			now[target] = now[target] - vp[move_type];
		}
		else {
			eprintln!("{}", next_score);
		}
	}

	write_output(&Output { vertices: input.figure.vertices.clone() })
}



// 暫定的な評価を計算する
// 実装予定
// 加点：　dislikeの距離そのまま
// 減点（外にはみ出る）：　(はみ出た距離 + 1) * outside_value
// 減点（距離）： (多角形内部までのマンハッタン距離) * distance_value
// 実装検討
// 減点or加点：　目標点からの距離
fn get_all_score(inp: &Input, now: &Vec<P<i64>>, eps: i64, point_board: &Vec<Vec<f64>>) -> f64 {

	let outside_value = 100.0;
	let distance_value = 100.0;

	let mut score = 0.0;
	let vs = inp.figure.vertices.clone();
	let es = inp.figure.edges.clone();
	let n = vs.len();
	//let Hole = inp.hole;

	for v in 0..n{
		score -= point_board[now[v].0 as usize][now[v].1 as usize] * outside_value;
	}

	for e in es {
		let d1 = hyp(vs[e.0].0 - vs[e.1].0, vs[e.0].1 - vs[e.1].1); 
		let d2 = hyp(now[e.0].0 - now[e.1].0, now[e.0].1 - now[e.1].1);
		let mut dd = {
			if d1 > d2 {
				d1 as f64 * (1.0_f64 - eps as f64 / 1500000.0) - d2 as f64
			}
			else{
				d2 as f64 - d1 as f64 * (1.0_f64 + eps as f64 / 1500000.0)
			}
		};
		if dd < 0.0 {
			dd = 0.0;
		}
		score -= dd * distance_value;
	}

	return score;
}

fn hyp(a: i64, b: i64) -> i64{
	return a * a + b * b;
}
