#![allow(unused)]
use icfpc2021::*;

struct Config {
    /// 3, 5, 7, ...
    neighbor_size: i64,
}

fn edge_penalty(input: &Input, v1: usize, v2: usize, p1: Point, p2: Point) -> i64 {
    let abs2_before = (input.figure.vertices[v1] - input.figure.vertices[v2]).abs2();
    let abs2_after = (p1 - p2).abs2();

    let penalty1 = abs2_after * 1_000_000 - abs2_before * (1_000_000 + input.epsilon);
    let penalty2 = abs2_before * (1_000_000 - input.epsilon) - abs2_after * 1_000_000;
    0.max(penalty1).max(penalty2)
}

fn find_largest_penalty(input: &Input, vertices: &Vec<Point>) -> (i64, Option<usize>) {
    let mut most = (0, None);

    for (ei, e) in input.figure.edges.iter().enumerate() {
        let penalty = edge_penalty(&input, e.0, e.1, vertices[e.0], vertices[e.1]);
        if penalty > most.0 {
            most = (penalty, Some(ei));
        }
    }

    most
}

fn lit(v: usize, d: i64) -> i64 {
    1 + (v as i64) * 9 + d
}

fn dv(d: i64) -> Point {
    P(d % 3 - 1, d / 3 - 1)
}

// fn generate_clauses(input: &Input, vertices: &Vec<Point>, target_edge: usize) -> Vec<Vec<i64>> {
fn generate_clauses(input: &Input, vertices: &Vec<Point>, penalty_limit: i64) -> Vec<Vec<i64>> {
    let mut clauses = vec![];
    let n_vs = vertices.len();

    // 9つのリテラルを用意し、1つだけtrueになるように
    for v in 0..n_vs {
        clauses.push((0..9).map(|i| lit(v, i)).collect());

        for i in 0..9 {
            for j in 0..i {
                clauses.push(vec![-lit(v, j), -lit(v, i)])
            }
        }
    }

    // 角を構成してるやつは真ん中に固定
    for v in 0..n_vs {
        let p = vertices[v];
        if input.hole.contains(&p) {
            // dbg!(&v);
            clauses.push(vec![lit(v, 4)]);
        }
    }

    // はみ出す場所には移動しない
    for v in 0..n_vs {
        for d in 0..9 {
            let tp = vertices[v] + dv(d);
            if P::contains_p(&input.hole, tp) == -1 {
                clauses.push(vec![-lit(v, d)]);
            }
        }
    }

    // 辺の制約

    for (ei, &(v1, v2)) in input.figure.edges.iter().enumerate() {
        let p1 = vertices[v1];
        let p2 = vertices[v2];
        let current_penalty = edge_penalty(&input, v1, v2, p1, p2);

        for d1 in 0..9 {
            let tp1 = p1 + dv(d1);
            for d2 in 0..9 {
                let tp2 = p2 + dv(d2);
                let new_penalty = edge_penalty(&input, v1, v2, tp1, tp2);

                let ok;
                if true {
                    // グローバルペナルティ版
                    ok = P::contains_s(&input.hole, (tp1, tp2)) && new_penalty < penalty_limit;
                } else {
                    // 個別ペナルティ版
                    ok = P::contains_s(&input.hole, (tp1, tp2))
                        && new_penalty < penalty_limit
                        && new_penalty <= current_penalty;
                }

                if !ok {
                    clauses.push(vec![-lit(v1, d1), -lit(v2, d2)]);
                }
            }
        }
    }

    clauses
}

fn reconstruct_positions(
    input: &Input,
    positions: &Vec<Point>,
    solution: &Vec<bool>,
) -> Vec<Point> {
    let mut new_positions = positions.clone();
    for (v, p) in positions.iter().enumerate() {
        for d in 0..9 {
            if solution[lit(v, d) as usize] {
                new_positions[v] = *p + dv(d);
            }
        }
    }
    new_positions
}

//
// SATソルバ部分
//

fn write_clauses(clauses: &Vec<Vec<i64>>) {
    use std::io::Write;
    let mut writer = std::io::BufWriter::new(std::fs::File::create("sat_in.txt").unwrap());

    let max_lit = clauses
        .iter()
        .map(|clause| clause.iter().map(|l| (*l).abs()).max().unwrap())
        .max()
        .unwrap();
    writeln!(&mut writer, "p cnf {} {}", max_lit, clauses.len()).unwrap();
    for clause in clauses {
        for l in clause {
            write!(&mut writer, "{} ", l).unwrap();
        }
        writeln!(&mut writer, "0").unwrap();
    }
}

fn read_solution() -> Vec<bool> {
    use std::io::BufRead;

    let mut reader = std::io::BufReader::new(std::fs::File::open("sat_out.txt").unwrap());
    let mut line = String::new();
    reader.read_line(&mut line);

    let lits: Vec<_> = line
        .split_whitespace()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let n_lits = 1 + lits.iter().map(|l| l.abs()).max().unwrap() as usize;

    let mut sol = vec![false; n_lits];
    for l in lits {
        sol[l.abs() as usize] = l >= 0;
    }
    sol
}

fn solve_by_glucose(clauses: &Vec<Vec<i64>>) -> Vec<bool> {
    let glucose_path = "/home/takiba/Desktop/glucose-syrup-4.1/simp/glucose";
    write_clauses(clauses);

    // TODO: stdoutの最後の行見てSATかUNSATかあれしたほうがいいかもしれない
    std::process::Command::new(glucose_path)
        .args(vec!["sat_in.txt", "sat_out.txt"])
        .status()
        .unwrap()
        .success();

    read_solution()
}

//
// main
//

fn step(input: &Input, positions: Vec<Point>, penalty_limit: i64) -> Vec<Point> {
    let clauses = generate_clauses(&input, &positions, penalty_limit);
    let solution = solve_by_glucose(&clauses);
    reconstruct_positions(&input, &positions, &solution)
}

fn dump(input: &Input, positions: &Vec<Point>, i_iter: i64) {
    let output = Output {
        vertices: positions.clone(),
        bonuses: Default::default(),
    };
    let writer = std::io::BufWriter::new(
        std::fs::File::create(format!("out/viz{:03}.svg", i_iter)).unwrap(),
    );
    icfpc2021::paths::render_pose_svg(&input, &output, writer);

    let writer = std::io::BufWriter::new(
        std::fs::File::create(format!("out/sol{:03}.json", i_iter)).unwrap(),
    );
    icfpc2021::write_output_to_writer(&output, writer);

    let score = icfpc2021::compute_score(input, &output);
    dbg!(score);
}

fn main() {
    use structopt::StructOpt;

    #[derive(StructOpt, Debug)]
    struct Args {
        #[structopt(long)]
        input_path: String,

        #[structopt(long)]
        output_path: String,
        // #[structopt(long)]
        // glucose_path: String,
    }
    let args = Args::from_args();
    dbg!(&args);

    let input = read_input_from_file(&args.input_path);
    let output = read_output_from_file(&args.output_path);

    let mut positions = output.vertices.clone();

    dump(&input, &positions, 0);

    if false {
        let penalty_limit = find_largest_penalty(&input, &positions).0 * 3; // TODO: ハイパラ
        positions = step(&input, positions, penalty_limit);
    }

    let mut i_iter: i64 = 1;
    loop {
        dump(&input, &positions, i_iter);

        let (largest_penalty, largest_edge) = find_largest_penalty(&input, &positions);
        dbg!(largest_penalty, largest_edge);
        if largest_penalty == 0 {
            break;
        }

        positions = step(&input, positions, largest_penalty);

        i_iter += 1;
    }

    dump(&input, &positions, 999);
}
