#![allow(unused)]
use core::f64;
use crate::*;
use num::integer::Roots;
use rand::prelude::*;
use std::{env, usize, vec};

pub fn main(input: &Input, output: &Output, timeout: f64, dontmoveflag: bool) -> Output {
    let tempout = false;

    let n = output.vertices.len();
    let v = input.hole.len();

    let mut first_now = output.vertices.clone();

    let mut maxnum = 0;
    for p in &input.hole {
        if maxnum < p.0 {
            maxnum = p.0;
        }

        if maxnum < p.1 {
            maxnum = p.1;
        }
    }

    for p in &output.vertices {
        if maxnum < p.0 {
            maxnum = p.0;
        }

        if maxnum < p.1 {
            maxnum = p.1;
        }
    }

    for p in &input.figure.vertices {
        if maxnum < p.0 {
            maxnum = p.0;
        }

        if maxnum < p.1 {
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

    let eps = input.epsilon;

    let vp: [P<i64>; 8] = [
        P(1, 0),
        P(0, 1),
        P(-1, 0),
        P(0, -1),
        P(1, 1),
        P(1, -1),
        P(-1, -1),
        P(-1, 1),
    ];

    let mut point_board = vec![vec![0.0; maxnum]; maxnum];

    for y in 0..maxnum {
        for x in 0..maxnum {
            if P::contains_p(&input.hole, P(y as i64, x as i64)) == -1 {
                point_board[y as usize][x as usize] = 99999.0;
            }
        }
    }

    for i in 0..300 {
        let mut flag = true;
        for y in 0..maxnum {
            for x in 0..maxnum {
                if point_board[y as usize][x as usize] > 10000.0 {
                    for k in 0..4 {
                        let ny = y as i64 + vp[k].0;
                        let nx = x as i64 + vp[k].1;
                        if ny >= 0
                            && ny < maxnum as i64
                            && nx >= 0
                            && nx < maxnum as i64
                            && point_board[ny as usize][nx as usize] == i as f64
                        {
                            point_board[y as usize][x as usize] = (i + 1) as f64;
                            flag = false;
                        }
                    }
                }
            }
        }
        if flag {
            break;
        }
    }

    for i in 0..n {
        //first_now[i] = P(maxnum as i64 / 2, maxnum as i64 / 2);
        //first_now[i] = P(maxnum as i64 - input.figure.vertices[i].0 - 1, maxnum as i64 - input.figure.vertices[i].1 - 1);
        //first_now[i] = P(maxnum as i64 * 3 / 4 - input.figure.vertices[i].0 / 2 - 1, maxnum as i64 * 3 / 4 - input.figure.vertices[i].1 / 2 - 1);
        //first_now[i] = P(maxnum as i64 - output.vertices[i].0 - 1, maxnum as i64 - output.vertices[i].1 - 1);
        //
        //first_now[i] = input.figure.vertices[i].clone();
        //first_now[i] = P(first_now[i].0 + maxnum as i64 * 6 / 10, first_now[i].1 + maxnum as i64 * 1 / 5);
        //eprintln!("{} {} {}", first_now[i].0, first_now[i].1, point_board[first_now[i].0 as usize][first_now[i].1 as usize]);

        first_now[i] = output.vertices[i].clone();
        //first_now[i] = input.figure.vertices[i].clone();
        //first_now[i] = P(input.figure.vertices[i].0 / 2 + maxnum as i64 / 4, input.figure.vertices[i].1 / 2 + maxnum as i64 / 4);
        //first_now[i] = P(input.figure.vertices[i].0 / 4 + maxnum as i64 * 3 / 8, input.figure.vertices[i].1 / 4 + maxnum as i64 * 3 / 8);

        //first_now[i] = P(thread_rng().gen_range(0..maxnum) as i64, thread_rng().gen_range(0..maxnum) as i64);
        //first_now[i] = P(thread_rng().gen_range(0..maxnum) as i64 / 2 + maxnum as i64 / 4, thread_rng().gen_range(0..maxnum) as i64 / 2 + maxnum as i64 / 4);
    }
    //write_output(&Output { vertices: first_now.clone(), bonuses: Default::default() });

    let starttime = get_time();

    /*
    for y in 0..maxnum {
        for x in 0..maxnum {
            eprint!("{} ", point_board[y as usize][x as usize]);
        }
        eprintln!("");
    }
    */

    let mut v_list = vec![vec![0; 0]; n];

    for i in &input.figure.edges {
        v_list[i.0].push(i.1);
        v_list[i.1].push(i.0);
    }

    //holeの長さからholeと頂点を決め打ちするやつ

    /*
    for i in 0..v {
        let d1 = (input.hole[i] - input.hole[(i+1)%v]).abs2();
        let d2 = (input.hole[(i + 1)% v] - input.hole[(i+2)%v]).abs2();

        for j in 0..n {
            let mut flag = 0;
            for k in &v_list[j] {
                let d3 = (input.figure.vertices[j] - input.figure.vertices[*k]).abs2();
                if (d1-d3).abs() * 1000000 <= d1*eps {flag |= 1;}
                if (d2-d3).abs() * 1000000 <= d2*eps {flag |= 2;}
            }
            if flag == 3 {
                println!("{} is {}", i, j);
            }
        }
    }
    */

    let mut dont_move = vec![false; n];

    if dontmoveflag {
        for i in 0..n {
            for j in 0..v {
                if output.vertices[i] == input.hole[j] {
                    //eprintln!(
                    //    "v{} stop by {}, pos: {} {}",
                    //    i, j, output.vertices[i].0, output.vertices[i].1
                    //);
                    dont_move[i] = true;
                }
            }
        }

        //for i in 0..n {
        //	if output.vertices[i] != P(177, 184){
        //		eprintln!("v{} stop {} {}", i, output.vertices[i].0, output.vertices[i].1);
        //		dont_move[i] = true;
        //	}
        //}

        /*
        eprintln!("start_position:");
        write_output(&Output {
            vertices: first_now.clone(),
            bonuses: Default::default(),
        });
        println!();
        */

        for _ in 0..100000 {
            let mut next_now = first_now.clone();

            for e in &input.figure.edges {
                let a = e.0;
                let b = e.1;
                let d1 = (first_now[b] - first_now[a]).abs2();
                let d2 = (input.figure.vertices[b] - input.figure.vertices[a]).abs2();

                if d1 > d2 {
                    if !dont_move[a] {
                        next_now[a].0 += (first_now[b] - first_now[a]).0 / 20;
                        next_now[a].1 += (first_now[b] - first_now[a]).1 / 20;
                    }

                    if !dont_move[b] {
                        next_now[b].0 += (first_now[a] - first_now[b]).0 / 20;
                        next_now[b].1 += (first_now[a] - first_now[b]).1 / 20;
                    }
                } else {
                    if !dont_move[a] {
                        next_now[a].0 -= (first_now[b] - first_now[a]).0 / 40;
                        next_now[a].1 -= (first_now[b] - first_now[a]).1 / 40;
                    }

                    if !dont_move[b] {
                        next_now[b].0 -= (first_now[a] - first_now[b]).0 / 40;
                        next_now[b].1 -= (first_now[a] - first_now[b]).1 / 40;
                    }
                }
            }
            first_now = next_now.clone();
        }

        println!();
    }

    let mut allbest = -9999999999999999.0;
    let mut allbest2 = -9999999999999999.0;
    let mut best_ans = first_now.clone();

    let mut point_error = vec![0.0; n];
    let mut dist_error = vec![0.0; n];
    let mut edge_error = vec![0.0; n];
    let mut v_best = vec![0.0; v];

    let ret = get_first_score(
        &input,
        &mut first_now,
        eps,
        &point_board,
        &mut point_error,
        &mut dist_error,
        &mut edge_error,
        &mut v_best,
        &v_list,
    );
    if ret.1 == 0.0 {
        allbest = ret.0;
        allbest2 = ret.0;
    }

    //eprintln!("start : {}", &allbest);

    let mut best_part = vec![0; v];
    for i in 0..v {
        best_part[i] = thread_rng().gen_range(0..n);
    }

    for ll in 0..1000000 {
        let mut now_temp = first_now.clone();

        let nowtime = get_time() - starttime;
        if nowtime >= timeout {
            break;
        }

        if ll != 0 {
            for i in 0..n {
                if thread_rng().gen_range(0..3) == 0 && ll != 0 && !dont_move[i] {
                    let mut nexty = now_temp[i].0
                        + thread_rng().gen_range(-(maxnum as i64) / 6..(maxnum as i64) / 6 + 1);
                    if nexty < 0 {
                        nexty = -nexty;
                    }
                    if nexty >= maxnum as i64 {
                        nexty = maxnum as i64 - (nexty - maxnum as i64 + 1);
                    }

                    let mut nextx = now_temp[i].1
                        + thread_rng().gen_range(-(maxnum as i64) / 6..(maxnum as i64) / 6 + 1);
                    if nextx < 0 {
                        nextx = -nextx;
                    }
                    if nextx >= maxnum as i64 {
                        nextx = maxnum as i64 - (nextx - maxnum as i64 + 1);
                    }

                    now_temp[i] = P(nexty, nextx);
                }
                //now_temp[i] = P(thread_rng().gen_range(0..maxnum) as i64 / 2 + maxnum as i64 / 4, thread_rng().gen_range(0..maxnum) as i64 / 2 + maxnum as i64 / 4);
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

        let ret = get_first_score(
            &input,
            &mut now,
            eps,
            &point_board,
            &mut point_error,
            &mut dist_error,
            &mut edge_error,
            &mut v_best,
            &v_list,
        );
        let mut bestscore = ret.0;
        let loopend = 3000000;
        let updatenum = 30000;
        let mut update = updatenum;

        let none = 9999;
        let mut prechoose = none;
        let mut premove = none;

        let mut prescore = ret;

        for mut cnt in 0..loopend {
            if update < 0 {
                break;
            }
            update -= 1;
            let now_score = prescore; //get_all_score(&input, &now, eps, &point_board);
            let target = {
                if prechoose == none {
                    thread_rng().gen_range(0..n)
                } else {
                    prechoose
                }
            };
            let move_type = {
                if prechoose == none {
                    thread_rng().gen_range(0..8)
                } else {
                    premove
                }
            };

            if dont_move[target] {
                cnt -= 1;
                continue;
            }

            if (now_score.0 - now_score.1).abs() < 1e-6 {
                let error = point_error[target] + edge_error[target] + dist_error[target];
                if error > -1e-6 {
                    premove = none;
                    prechoose = none;
                    continue;
                }
            }

            let mut move_vec = vec![0; 0];
            move_vec.push(target);
            let move_rate = thread_rng().gen_range(0..100);

            for i in &v_list[target] {
                if thread_rng().gen_range(0..100) < move_rate && !dont_move[*i] {
                    move_vec.push(*i);
                }
            }

            let mut nextcheck = false;
            for i in &move_vec {
                if !dont_move[*i] {
                    let pos = now[*i] + vp[move_type];
                    if pos.0 < 0 {
                        nextcheck = true;
                    }
                    if pos.1 < 0 {
                        nextcheck = true;
                    }
                    if pos.0 >= maxnum as i64 {
                        nextcheck = true;
                    }
                    if pos.1 >= maxnum as i64 {
                        nextcheck = true;
                    }
                }
            }

            if nextcheck {
                premove = none;
                prechoose = none;
                continue;
            }

            //now[target] = now[target] + vp[move_type];

            let temp = cnt as f64 / loopend as f64;

            let mut next_score = now_score;

            for i in &move_vec {
                let nextp = now[*i] + vp[move_type];
                let add = get_move_score(
                    false,
                    &input,
                    &mut now,
                    eps,
                    &point_board,
                    *i,
                    nextp,
                    &mut point_error,
                    &mut dist_error,
                    &mut edge_error,
                    &mut v_best,
                    &v_list,
                );
                next_score.0 += add.0;
                next_score.1 += add.1;
            }

            //println!(" temp : {} {} {}", cnt, next_score.0, next_score.1);

            if now_score.0 - next_score.0
                > thread_rng().gen_range(0..100000000) as f64 * pow3(pow3(pow3(1.0 - temp)))
                    / 200.0
                    / 1000.0
            {
                for i in &move_vec {
                    let nextp = now[*i] - vp[move_type];
                    let add = get_move_score(
                        false,
                        &input,
                        &mut now,
                        eps,
                        &point_board,
                        *i,
                        nextp,
                        &mut point_error,
                        &mut dist_error,
                        &mut edge_error,
                        &mut v_best,
                        &v_list,
                    );
                    next_score.0 += add.0;
                    next_score.1 += add.1;
                }
                prescore = next_score;
                premove = none;
                prechoose = none;
            //now[target] = now[target] - vp[move_type];
            } else {
                prescore = next_score;
                premove = move_type;
                prechoose = target;

                //eprintln!(" temp : {} {} {}", cnt, next_score.0, next_score.1);
                if next_score.0 > bestscore {
                    if tempout {
                        eprintln!("temp2 : {} {} {}", cnt, next_score.0, next_score.1);
                    }
                    bestscore = next_score.0;
                    if allbest2 < bestscore {
                        allbest2 = bestscore;
                        first_now = now.clone();
                    }

                    update = updatenum;
                }

                if allbest < next_score.0 - 0.5 && next_score.1 > -1e-5 {
                    let watacheck = compute_score(
                        &input,
                        &Output {
                            vertices: now.clone(),
                            bonuses: Default::default(),
                        },
                    );

                    if (watacheck < 10000000) {
                        eprintln!(" OK! : {} {} {} ", cnt, next_score.0, next_score.1);
                        eprintln!("wata-check : {}", watacheck);

                        next_score.0 = -watacheck as f64;
                        next_score.1 = 0.0;
                        allbest = next_score.0;
                        best_ans = now.clone();
                    } else {
                        next_score = get_first_score(
                            &input,
                            &mut now,
                            eps,
                            &point_board,
                            &mut point_error,
                            &mut dist_error,
                            &mut edge_error,
                            &mut v_best,
                            &v_list,
                        );
                    }
                    prescore = next_score;
                }
            }
        }

        //eprintln!("ans : {} {}", prescore.0, prescore.1);

        //if allbest2 == bestscore {
        //	best_part = nowpart.clone();
        //}

        if allbest >= 0.0 {
            break;
        }
    }

    //eprintln!("ans : {}", 100000.0 - allbest);
    //eprintln!("wata-check : {}", compute_score(&input, &Output { vertices: best_ans.clone() }));

    return Output {
        vertices: best_ans.clone(),
        bonuses: Default::default(),
    }
}

///差分評価を返す関数

fn get_move_score(
    firstflag: bool,
    inp: &Input,
    now: &mut Vec<P<i64>>,
    eps: i64,
    point_board: &Vec<Vec<f64>>,
    id: usize,
    next_pos: P<i64>,
    point_error: &mut Vec<f64>,
    dist_error: &mut Vec<f64>,
    edge_error: &mut Vec<f64>,
    v_best: &mut Vec<f64>,
    v_list: &Vec<Vec<usize>>,
) -> (f64, f64) {
    //設定値
    let perror_value = 20.0;
    let eerror_value = 1000.0;
    let derror_value = 100.0;
    let derror_value2 = 1000.0;

    let vs = &inp.figure.vertices;

    //最終的なスコアに影響するやつ
    let mut ans1 = 0.0;
    //これが0じゃないとそもそも解としてvalidでないやつ
    let mut ans2 = 0.0;

    //point_error その座標が外の時にエラー
    let pre_pointerror = point_error[id];
    if !firstflag {
        ans2 -= point_error[id];
    }
    point_error[id] = point_board[next_pos.0 as usize][next_pos.1 as usize] * perror_value;
    ans2 += point_error[id];

    //dist_error 距離が適切でないときにエラー
    for v in &v_list[id] {
        let d1 = (vs[id] - vs[*v]).abs2();
        if !firstflag {
            let d2 = (now[id] - now[*v]).abs2();
            let epsd = (d1 * eps) as f64 / 1000000.0;
            let mut dd = (d2 - d1).abs() as f64;
            if dd <= epsd {
                dd = 0.0;
            } else {
                dd += 0.1;
            }
            let add = -dd * derror_value;
            dist_error[id] -= add;
            dist_error[*v] -= add;
            ans2 -= add * 2.0;
        }
        {
            let d2 = (next_pos - now[*v]).abs2();
            let epsd = (d1 * eps) as f64 / 1000000.0;
            let mut dd = (d2 - d1).abs() as f64;
            if dd <= epsd {
                dd = 0.0;
            } else {
                dd += 0.1;
            }
            let add = -dd * derror_value;
            dist_error[id] += add;
            dist_error[*v] += add;
            ans2 += add * 2.0;
        }
    }

    //edge_error またがっている時にエラー
    for v in &v_list[id] {
        if !firstflag {
            let mypos = now[id];
            let my_pointerror = point_board[mypos.0 as usize][mypos.1 as usize];

            if pre_pointerror > 0.0 || point_error[*v] > 0.0 {
                let val = -derror_value2;
                edge_error[id] -= val;
                edge_error[*v] -= val;
                ans2 -= val * 2.0;
            } else {
                if !P::contains_s(&inp.hole, (mypos, now[*v])) {
                    let d2 = shortest_path(&inp.hole, mypos, now[*v]).0;
                    let d1 = ((mypos - now[*v]).abs2() as f64).sqrt();
                    let mul = (d2 - d1) / 2.0 + 1.0;
                    let val = -mul * mul * eerror_value;
                    edge_error[id] -= val;
                    edge_error[*v] -= val;
                    ans2 -= val * 2.0;
                }
            }
        }
        {
            let mypos = next_pos;
            let my_pointerror = point_board[mypos.0 as usize][mypos.1 as usize];

            if my_pointerror > 0.0 || point_error[*v] > 0.0 {
                let val = -derror_value2;
                edge_error[id] += val;
                edge_error[*v] += val;
                ans2 += val * 2.0;
            } else {
                if !P::contains_s(&inp.hole, (mypos, now[*v])) {
                    let d2 = shortest_path(&inp.hole, mypos, now[*v]).0;
                    let d1 = ((mypos - now[*v]).abs2() as f64).sqrt();
                    let mul = (d2 - d1) / 2.0 + 1.0;
                    let val = -mul * mul * eerror_value;
                    edge_error[id] += val;
                    edge_error[*v] += val;
                    ans2 += val * 2.0;
                }
            }
        }
    }

    //v-error ホールまでのそれぞれの距離

    for i in 0..inp.hole.len() {
        let hpos = inp.hole[i];

        let d1 = (hpos - now[id]).abs2() as f64;
        let d2 = (hpos - next_pos).abs2() as f64;
        ans1 += v_best[i];

        if d1 <= v_best[i] + 1e-3 && d1 < d2 {
            v_best[i] = 999999999.0;
            for j in 0..inp.figure.vertices.len() {
                let d = (hpos - now[j]).abs2() as f64;
                if d < v_best[i] && j != id {
                    v_best[i] = d;
                }
            }
        }
        if d2 < v_best[i] {
            v_best[i] = d2;
        }

        ans1 -= v_best[i];

        //eprint!("{} ", v_best[i]);
    }
    //eprintln!();

    now[id] = next_pos;
    return (ans1 + ans2, ans2);
}

fn get_first_score(
    inp: &Input,
    now: &mut Vec<P<i64>>,
    eps: i64,
    point_board: &Vec<Vec<f64>>,
    point_error: &mut Vec<f64>,
    dist_error: &mut Vec<f64>,
    edge_error: &mut Vec<f64>,
    v_best: &mut Vec<f64>,
    v_list: &Vec<Vec<usize>>,
) -> (f64, f64) {
    //設定値
    let perror_value = 20.0;
    let eerror_value = 1000.0;
    let derror_value = 100.0;
    let derror_value2 = 1000.0;

    let n = inp.figure.vertices.len();
    let v = inp.hole.len();

    for i in 0..n {
        point_error[i] = 0.0;
        dist_error[i] = 0.0;
        edge_error[i] = 0.0;
    }

    for i in 0..v {
        v_best[i] = 9999999999.0;
    }

    for i in 0..n {
        point_error[i] = point_board[now[i].0 as usize][now[i].1 as usize] * perror_value;
    }

    for i in 0..n {
        get_move_score(
            true,
            inp,
            now,
            eps,
            point_board,
            i,
            now[i],
            point_error,
            dist_error,
            edge_error,
            v_best,
            v_list,
        );
    }

    let mut ans1 = 0.0;
    let mut ans2 = 0.0;
    for i in 0..n {
        ans2 += point_error[i];
        edge_error[i] /= 2.0;
        ans2 += edge_error[i];
        dist_error[i] /= 2.0;
        ans2 += dist_error[i];

        //eprintln!("{},{},{}", point_error[i], edge_error[i], dist_error[i]);
    }

    for i in 0..v {
        ans1 -= v_best[i];
        //eprintln!("{}", v_best[i]);
    }

    return (ans1 + ans2, ans2);
}

fn hyp(a: i64, b: i64) -> i64 {
    return a * a + b * b;
}

fn pow3(a: f64) -> f64 {
    return a * a * a;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
}

// https://github.com/rust-lang/rust/issues/48564
#[cfg(target_arch = "wasm32")]
pub fn get_time() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now() as f64 / 1_000.0
}

pub fn get_new_graph(input: &Input, pre: &Vec<P<i64>>, dont_move: &Vec<bool>) -> Vec<P<i64>> {
    let mut now = pre.clone();
    for _ in 0..100000 {
        let mut next_now = now.clone();

        for e in &input.figure.edges {
            let a = e.0;
            let b = e.1;
            let d1 = (now[b] - now[a]).abs2();
            let d2 = (input.figure.vertices[b] - input.figure.vertices[a]).abs2();

            if d1 > d2 {
                if !dont_move[a] {
                    next_now[a].0 += (now[b] - now[a]).0 / 20;
                    next_now[a].1 += (now[b] - now[a]).1 / 20;
                }

                if !dont_move[b] {
                    next_now[b].0 += (now[a] - now[b]).0 / 20;
                    next_now[b].1 += (now[a] - now[b]).1 / 20;
                }
            } else {
                if !dont_move[a] {
                    next_now[a].0 -= (now[b] - now[a]).0 / 40;
                    next_now[a].1 -= (now[b] - now[a]).1 / 40;
                }

                if !dont_move[b] {
                    next_now[b].0 -= (now[a] - now[b]).0 / 40;
                    next_now[b].1 -= (now[a] - now[b]).1 / 40;
                }
            }
        }
        now = next_now.clone();
    }
    return now;
}
