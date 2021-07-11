#![allow(unused)]
use icfpc2021::*;
use rand::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

	let input_path = &std::path::PathBuf::from(&args[1]);
	let output_path = &std::path::PathBuf::from(&args[2]);
	//let output_path= format!("{}{}{}", "../../Users/choku/Dropbox/ICFPC2021/best/", args[1], ".json");

	let filesize = std::fs::File::open(&output_path).unwrap().metadata().unwrap().len();
	//eprintln!("{}", input_path);
	//eprintln!("{}", output_path);

	let input = read_input_from_file(input_path);
	let mut output = Output { vertices: input.figure.vertices.clone(), bonuses: Default::default() };
	if filesize > 0 {
		output = read_output_from_file(output_path);
	}
	//let output = &Output { vertices: input.figure.vertices.clone() };
	dbg!(&input);
	dbg!(&output);


	let n = output.vertices.len();
	let v = input.hole.len();
	
	
	let mut first_now = output.vertices.clone();
	
	let mut maxnum = 0;
	for p in &input.hole{
		if maxnum < p.0{
			maxnum = p.0;
		}	
		
		if maxnum < p.1{
			maxnum = p.1;
		}	
	}

	for p in &output.vertices{
		if maxnum < p.0{
			maxnum = p.0;
		}	
		
		if maxnum < p.1{
			maxnum = p.1;
		}	
	}


	for p in &input.figure.vertices{
		if maxnum < p.0{
			maxnum = p.0;
		}	
		
		if maxnum < p.1{
			maxnum = p.1;
		}	
	}

	/*
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
 	*/
	
	let maxnum = (maxnum + 1) as usize;
	
	for i in 0..n {
		//first_now[i] = P(maxnum as i64 / 2, maxnum as i64 / 2);
		//first_now[i] = P(maxnum as i64 - input.figure.vertices[i].0 - 1, maxnum as i64 - input.figure.vertices[i].1 - 1);
		//first_now[i] = P(maxnum as i64 - output.vertices[i].0 - 1, maxnum as i64 - output.vertices[i].1 - 1);
		
		first_now[i] = output.vertices[i].clone();
		//first_now[i] = P(thread_rng().gen_range(0..maxnum) as i64, thread_rng().gen_range(0..maxnum) as i64);
	}

	let eps = input.epsilon;



	let vp: [P<i64>; 8] = [P(1, 0), P(0, 1), P(-1, 0), P(0, -1), P(1, 1), P(1, -1), P(-1, -1), P(-1, 1)];

	let mut point_board = vec![vec![0.0; maxnum]; maxnum];

	
	for y in 0..maxnum {
		for x in 0..maxnum {
			if P::contains_p(&input.hole, P(y as i64, x as i64)) == -1 {
				point_board[y as usize][x as usize] = 99999.0;
			}
		}
	}

	let starttime = get_time();

	for i in 0..300 {
		let mut flag = true;
		for y in 0..maxnum {
			for x in 0..maxnum {
				if point_board[y as usize][x as usize] > 100.0 {
					for k in 0..4 {
						let ny = y as i64 + vp[k].0;
						let nx = x as i64 + vp[k].1;
						if ny >= 0 && ny < maxnum as i64 && nx >= 0 && nx < maxnum as i64 && point_board[ny as usize][nx as usize] == i as f64{
							point_board[y as usize][x as usize] = (i + 1) as f64;
							flag = false;
						}
					}
				}
			}
		}
		if flag { break; }
	}

	/*
	for y in 0..maxnum {
		for x in 0..maxnum {
			eprint!("{} ", point_board[y as usize][x as usize]);
		}
		eprintln!("");
	}
	*/


	let mut v_list = vec![vec![0; 0]; n];

	for i in &input.figure.edges{
		v_list[i.0].push(i.1);
		v_list[i.1].push(i.0);
	}



	let mut allbest =  -9999999999999999.0;
	let mut allbest2 =  -9999999999999999.0;
	let mut best_ans = first_now.clone();

	let ret = get_all_score(&input, &first_now, eps, &point_board);
	if ret.1 == 0.0 {
		allbest = ret.0;
		allbest2 = ret.0;
	}

	eprintln!("start : {}", &allbest);

	let mut best_part = vec![0; v];
	for i in 0..v {
		best_part[i] = thread_rng().gen_range(0..n);
	}

	for ll in 0..1000000{
		let mut now_temp =first_now.clone();

		let nowtime = get_time() - starttime;
		if nowtime >= 600.0 { break; }
		
		for i in 0..n {

			if thread_rng().gen_range(0..3) == 0 && ll != 0{
				let mut nexty = now_temp[i].0 + thread_rng().gen_range(-(maxnum as i64)/6..(maxnum as i64)/6+1);
				if nexty < 0 {nexty = -nexty;}
				if nexty >= maxnum as i64 {nexty = maxnum as i64 - (nexty - maxnum as i64 + 1);}

				let mut nextx = now_temp[i].1 + thread_rng().gen_range(-(maxnum as i64)/6..(maxnum as i64)/6+1);
				if nextx < 0 {nextx = -nextx;}
				if nextx >= maxnum as i64 {nextx = maxnum as i64 - (nextx - maxnum as i64 + 1);}
				
				now_temp[i] = P(nexty, nextx);
			}
		}
		

		let mut now = now_temp.clone();
		
		/* 
		for i in 0..n {
			now[i] = P(thread_rng().gen_range(0..maxnum) as i64, thread_rng().gen_range(0..maxnum) as i64);
		}

		let mut nowpart = best_part.clone();
		let movenum = thread_rng().gen_range(0..v);
		nowpart[movenum] = thread_rng().gen_range(0..n);
		
		for i in 0..v {
			now[nowpart[i]] = input.hole[i].clone();
		}
		*/


		let ret = get_all_score(&input, &now, eps, &point_board);
		let mut bestscore  = ret.0;
		let updatenum = 30000;
		let mut update = updatenum;

		//eprintln!(" first_score : {}", bestscore);
		//eprintln!(" first_score2 : {}", compute_score(&input, &Output { vertices: now.clone() }));


		let loopend = 3000000;

		for cnt in 0..loopend{
			if update < 0 { break; }
			update -= 1;
			let target =  thread_rng().gen_range(0..n);
			let now_score = get_all_score(&input, &now, eps, &point_board);
			let move_type = thread_rng().gen_range(0..8);

			let mut move_vec = vec![0; 0];
			move_vec.push(target);
			let move_rate = thread_rng().gen_range(0..100);

			for i in &v_list[target] {
				if thread_rng().gen_range(0..100) < move_rate {
					move_vec.push(*i);
				}
			}

			for i in &move_vec {
				now[*i] = now[*i] + vp[move_type];
			}

			//now[target] = now[target] + vp[move_type];

			let temp =  cnt as f64 / loopend as f64;

			let next_score = get_all_score(&input, &now, eps, &point_board);

			//println!(" temp : {} {} {}", cnt, next_score.0, next_score.1);
			
			if now_score.0 - next_score.0 > thread_rng().gen_range(0..1000) as f64 * pow3(pow3(pow3(1.0 - temp))) / 200.0 {
				
				for i in &move_vec {
					now[*i] = now[*i] - vp[move_type];
				}
				
				//now[target] = now[target] - vp[move_type];
			}
			else{
				//println!(" temp : {} {} {}", cnt, next_score.0, next_score.1);
				if next_score.0 > bestscore{
					//println!(" temp : {} {} {}", cnt, next_score.0, next_score.1);
					bestscore = next_score.0;
					if allbest2 < bestscore{
						allbest2 = bestscore;
						first_now = now.clone();
					}

					update = updatenum;
				}
			
				if allbest < next_score.0 && next_score.1 == 0.0 {
					eprintln!(" OK! : {} {}", cnt, next_score.0);
					eprintln!("wata-check : {}", compute_score(&input, &Output { vertices: now.clone(), bonuses: Default::default() }));
					allbest = next_score.0;
					best_ans = now.clone();
					//write_output(&Output { vertices: best_ans.clone() })
				}
			}
		}

		//if allbest2 == bestscore {
		//	best_part = nowpart.clone();
		//}

		eprintln!("{}", bestscore);

		if allbest >= 100000.0 { break; }
	}
	
	eprintln!("ans : {}", 100000.0 - allbest);
	eprintln!("wata-check : {}", compute_score(&input, &Output { vertices: best_ans.clone(), bonuses: Default::default() }));

	write_output(&Output { vertices: best_ans.clone(), bonuses: Default::default() })
}



// 暫定的な評価を計算する
// 実装予定
// 加点：　dislikeの距離そのまま
// 減点（外に点がはみ出る）：　(はみ出た距離 + 1) * outside_value
// 減点（外に線がはみ出る）：　outside_value2
// 減点（距離）： (多角形内部までのマンハッタン距離) * distance_value
fn get_all_score(inp: &Input, now: &Vec<P<i64>>, eps: i64, point_board: &Vec<Vec<f64>>) -> (f64, f64) {

	let outside_value = 20.0;
	let outside_value2 = 1000000.0;
	let distance_value = 100.0;

	let mut score = 0.0;
	let vs = inp.figure.vertices.clone();
	let es = inp.figure.edges.clone();
	let n = vs.len();
	//let Hole = inp.hole;

	for i in 0..n {
		if now[i].0 < 0 {return (-999999999999.9, -9999999999999.9);}
		if now[i].1 < 0 {return (-999999999999.9, -9999999999999.9);}
		if now[i].0 >= point_board.len() as i64 {return (-999999999999.9, -9999999999999.9);}
		if now[i].1 >= point_board.len() as i64 {return (-999999999999.9, -9999999999999.9);}
	}

	for v in 0..n{
		score -= pow3(point_board[now[v].0 as usize][now[v].1 as usize]) * outside_value;
	}

	for e in es {
		let d1 = (vs[e.0]- vs[e.1]).abs2(); 
		let d2 = (now[e.0]- now[e.1]).abs2(); 
		let epsd = (d1 * eps) as f64 / 1000000.0;
		let mut dd = (d2 - d1).abs() as f64;
		if dd <= epsd {
			dd = 0.0;
			//dd /= 5.0;
			//inner_flag = true;
		}
		else {dd = dd - epsd + 0.1; }

		let before = (vs[e.0] - vs[e.1]).abs2();
		let after = (now[e.0] - now[e.1]).abs2();
		
		if (after * 1000000 - before * 1000000).abs() > eps * before {
			if dd < 0.1 { dd = 0.1; println!("!?"); }
		}

		
		if dd <= 1.0{
			dd /= 2.0;
		}
		else if dd <= 2.0{
			//dd /= 20.0;
		}
		else if dd <= 3.0{
			//dd /= 10.0;
		}
		
		score -= dd * distance_value;


		if !P::contains_s(&inp.hole, (now[e.0], now[e.1])) {
			score -= outside_value2;
		}
	}

	let okflag = score;

	if true {
		score += 100000.0;
		for i in &inp.hole{
			let mut min_dist = 99999999999;
			for j in 0..n {
				let dist = hyp(now[j].0 - i.0, now[j].1 - i.1);
				if dist < min_dist {
					min_dist = dist;
				}
			}
			score -= min_dist as f64;
		}
	}

	/*
	score += 100000.0;
	for i in 0..inp.hole.len() {
		score -= hyp(now[part[i]].0 -  inp.hole[i].0, now[part[i]].1 -  inp.hole[i].1) as f64;
	}
	*/
	return (score, okflag);
}

fn hyp(a: i64, b: i64) -> i64{
	return a * a + b * b;
}

fn pow3(a: f64) -> f64{
	return a * a * a;
}

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



fn get_all_score2(inp: &Input, now: &Vec<P<i64>>, eps: i64, point_board: &Vec<Vec<f64>>) -> (f64, f64) {

	let outside_value = 20.0;
	let outside_value2 = 1000000.0;
	let distance_value = 100.0;

	let mut score = 0.0;
	let vs = inp.figure.vertices.clone();
	let es = inp.figure.edges.clone();
	let n = vs.len();
	//let Hole = inp.hole;

	for v in 0..n{
		score -= pow3(point_board[now[v].0 as usize][now[v].1 as usize]) * outside_value;
	}

	for e in es {
		let d1 = (vs[e.0]- vs[e.1]).abs2(); 
		let d2 = (now[e.0]- now[e.1]).abs2(); 
		let epsd = (d1 * eps) as f64 / 1000000.0;
		let mut dd = (d2 - d1).abs() as f64;
		if dd <= epsd {
			dd = 0.0;
			//dd /= 5.0;
			//inner_flag = true;
		}
		else {dd = dd - epsd + 0.1; }

		//let before = (vs[e.0] - vs[e.1]).abs2();
		//let after = (now[e.0] - now[e.1]).abs2();
		
		//if (after * 1000000 - before * 1000000).abs() > eps * before {
		//	if dd < 0.1 { dd = 0.1; println!("!?"); }
		//}

		
		if dd <= 1.0{
			dd /= 2.0;
		}
		else if dd <= 2.0{
			//dd /= 20.0;
		}
		else if dd <= 3.0{
			//dd /= 10.0;
		}
		
		score -= dd * distance_value;


		if !P::contains_s(&inp.hole, (now[e.0], now[e.1])) {
			score -= outside_value2;
		}
	}

	let okflag = score;

	if true {
		score += 100000.0;
		for i in &inp.hole{
			let mut min_dist = 99999999999;
			for j in 0..n {
				let dist = hyp(now[j].0 - i.0, now[j].1 - i.1);
				if dist < min_dist {
					min_dist = dist;
				}
			}
			score -= min_dist as f64;
		}
	}

	/*
	score += 100000.0;
	for i in 0..inp.hole.len() {
		score -= hyp(now[part[i]].0 -  inp.hole[i].0, now[part[i]].1 -  inp.hole[i].1) as f64;
	}
	*/
	return (score, okflag);
}