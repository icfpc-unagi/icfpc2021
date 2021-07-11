#![allow(unused)]
use icfpc2021::*;

struct Config {
    glucose_path: String,
    /// 3, 5, 7, ...
    neighbor: i64,
}

impl Config {
    /// 9
    fn n_cands(&self) -> i64 {
        self.neighbor * self.neighbor
    }

    /// 4
    fn d_center(&self) -> i64 {
        (self.n_cands() - 1) / 2
    }
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

fn lit(config: &Config, v: usize, d: i64) -> i64 {
    1 + (v as i64) * config.n_cands() + d
}

fn dv(config: &Config, d: i64) -> Point {
    let x = d % config.neighbor - (config.neighbor - 1) / 2;
    let y = d / config.neighbor - (config.neighbor - 1) / 2;
    P(x, y)
}

// fn generate_clauses(input: &Input, vertices: &Vec<Point>, target_edge: usize) -> Vec<Vec<i64>> {
fn generate_clauses(
    config: &Config,
    input: &Input,
    vertices: &Vec<Point>,
    penalty_limit: i64,
) -> Vec<Vec<i64>> {
    let mut clauses = vec![];
    let n_vs = vertices.len();

    // 9つのリテラルを用意し、1つだけtrueになるように
    for v in 0..n_vs {
        clauses.push((0..config.n_cands()).map(|i| lit(config, v, i)).collect());

        for i in 0..config.n_cands() {
            for j in 0..i {
                clauses.push(vec![-lit(config, v, j), -lit(config, v, i)])
            }
        }
    }

    // 角を構成してるやつは真ん中に固定
    for v in 0..n_vs {
        let p = vertices[v];
        if input.hole.contains(&p) {
            // dbg!(&v);
            clauses.push(vec![lit(config, v, config.d_center())]);
        }
    }

    // はみ出す場所には移動しない
    for v in 0..n_vs {
        for d in 0..config.n_cands() {
            let tp = vertices[v] + dv(config, d);
            if P::contains_p(&input.hole, tp) == -1 {
                clauses.push(vec![-lit(config, v, d)]);
            }
        }
    }

    // 辺の制約
    for (ei, &(v1, v2)) in input.figure.edges.iter().enumerate() {
        let p1 = vertices[v1];
        let p2 = vertices[v2];
        let current_penalty = edge_penalty(&input, v1, v2, p1, p2);

        for d1 in 0..config.n_cands() {
            let tp1 = p1 + dv(config, d1);
            for d2 in 0..config.n_cands() {
                let tp2 = p2 + dv(config, d2);
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
                    clauses.push(vec![-lit(config, v1, d1), -lit(config, v2, d2)]);
                }
            }
        }
    }

    clauses
}

fn reconstruct_positions(
    config: &Config,
    input: &Input,
    positions: &Vec<Point>,
    solution: &Vec<bool>,
) -> Vec<Point> {
    let mut new_positions = positions.clone();
    for (v, p) in positions.iter().enumerate() {
        for d in 0..config.n_cands() {
            if solution[lit(config, v, d) as usize] {
                new_positions[v] = *p + dv(config, d);
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

fn solve_by_glucose(config: &Config, clauses: &Vec<Vec<i64>>) -> Vec<bool> {
    write_clauses(clauses);

    // TODO: stdoutの最後の行見てSATかUNSATかあれしたほうがいいかもしれない
    std::process::Command::new(&config.glucose_path)
        .args(vec!["sat_in.txt", "sat_out.txt"])
        .status()
        .unwrap()
        .success();

    read_solution()
}

//
// main
//

fn step(config: &Config, input: &Input, positions: Vec<Point>, penalty_limit: i64) -> Vec<Point> {
    let clauses = generate_clauses(config, &input, &positions, penalty_limit);
    let solution = solve_by_glucose(config, &clauses);
    reconstruct_positions(config, &input, &positions, &solution)
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

        #[structopt(long)]
        glucose_path: String,

        #[structopt(long)]
        initial_relax: Option<f64>,

        #[structopt(short, long, default_value = "3")]
        neighbor: i64,
    }
    let args = Args::from_args();
    dbg!(&args);

    let input = read_input_from_file(&args.input_path);
    let output = read_output_from_file(&args.output_path);
    let config = Config {
        glucose_path: args.glucose_path.clone(),
        neighbor: args.neighbor,
    };
    let mut positions = output.vertices.clone();
    dump(&input, &positions, 0);

    // 最初にゆるめる？
    if let Some(initial_relax_ratio) = args.initial_relax {
        let penalty_limit =
            (find_largest_penalty(&input, &positions).0 as f64 * initial_relax_ratio) as i64;
        positions = step(&config, &input, positions, penalty_limit);
    }

    // 締めていく
    let mut i_iter: i64 = 1;
    loop {
        dump(&input, &positions, i_iter);

        let (largest_penalty, largest_edge) = find_largest_penalty(&input, &positions);
        dbg!(largest_penalty, largest_edge);
        if largest_penalty == 0 {
            break;
        }

        positions = step(&config, &input, positions, largest_penalty);

        i_iter += 1;
    }

    dump(&input, &positions, 999);
}
