#![allow(unused)]
use icfpc2021::*;

//
// 設定
//
struct Config {
    initial_relax: Option<f64>,
    glucose_path: String,

    /// 3, 5, 7, ...
    min_neighbor: i64,
    max_neighbor: i64,
}

//
// 本体
//

struct SatHillclimber {
    input: Input,
    config: Config,
    neighbor: i64,

    contains_p_cache: std::collections::HashMap<Point, i32>,
    contains_s_cache: std::collections::HashMap<(Point, Point), bool>,
}

impl SatHillclimber {
    pub fn new(input: Input, config: Config) -> Self {
        Self {
            neighbor: config.min_neighbor,
            input,
            config,
            contains_p_cache: Default::default(),
            contains_s_cache: Default::default(),
        }
    }

    //
    // 幾何系のユーティリティ
    //
    fn contains_p(&mut self, p: Point) -> i32 {
        if let Some(r) = self.contains_p_cache.get(&p) {
            *r
        } else {
            let r = P::contains_p(&self.input.hole, p);
            self.contains_p_cache.insert(p, r);
            r
        }
    }

    fn contains_s(&mut self, p: Point, q: Point) -> bool {
        if let Some(r) = self.contains_s_cache.get(&(p, q)) {
            *r
        } else {
            let r = P::contains_s(&self.input.hole, (p, q));
            self.contains_s_cache.insert((p, q), r);
            r
        }
    }

    //
    // ペナルティ系のユーティリティ
    //
    fn edge_penalty(&self, v1: usize, v2: usize, p1: Point, p2: Point) -> i64 {
        let abs2_before = (self.input.figure.vertices[v1] - self.input.figure.vertices[v2]).abs2();
        let abs2_after = (p1 - p2).abs2();

        let penalty1 = abs2_after * 1_000_000 - abs2_before * (1_000_000 + self.input.epsilon);
        let penalty2 = abs2_before * (1_000_000 - self.input.epsilon) - abs2_after * 1_000_000;
        0.max(penalty1).max(penalty2)
    }

    //
    // SAT生成
    //
    fn lit(&self, v: usize, d: i64) -> i64 {
        1 + (v as i64) * self.n_cands() + d
    }

    fn dv(&self, d: i64) -> Point {
        let x = d % self.neighbor - (self.neighbor - 1) / 2;
        let y = d / self.neighbor - (self.neighbor - 1) / 2;
        P(x, y)
    }

    /// 9
    fn n_cands(&self) -> i64 {
        self.neighbor * self.neighbor
    }

    /// 4
    fn d_center(&self) -> i64 {
        (self.n_cands() - 1) / 2
    }

    fn generate_clauses(&mut self, vertices: &Vec<Point>) -> Vec<Vec<i64>> {
        let mut clauses = vec![];
        let n_vs = vertices.len();

        // 9つのリテラルを用意し、1つだけtrueになるように
        for v in 0..n_vs {
            clauses.push((0..self.n_cands()).map(|i| self.lit(v, i)).collect());

            for i in 0..self.n_cands() {
                for j in 0..i {
                    clauses.push(vec![-self.lit(v, j), -self.lit(v, i)])
                }
            }
        }

        let mut improve_clause = vec![];
        for h in &self.input.hole {
            // 一番近い頂点見つける
            let (v, p) = vertices
                .iter()
                .enumerate()
                .min_by_key(|(_, p)| (**p - *h).abs2())
                .unwrap();
            let current_abs2 = (*p - *h).abs2();

            for d in 0..self.n_cands() {
                let l = self.lit(v, d);
                let tp = *p + self.dv(d);
                let next_abs2 = (tp - *h).abs2();

                // 遠くなるのは一切禁止
                if next_abs2 > current_abs2 {
                    clauses.push(vec![-l]);
                }
                // 近くなるやつ歓迎
                if next_abs2 < current_abs2 {
                    improve_clause.push(l);
                }
            }
        }
        clauses.push(improve_clause);

        // はみ出す場所には移動しない
        for v in 0..n_vs {
            for d in 0..self.n_cands() {
                let tp = vertices[v] + self.dv(d);
                if self.contains_p(tp) == -1 {
                    clauses.push(vec![-self.lit(v, d)]);
                }
            }
        }

        // 辺の制約
        for ei in 0..self.input.figure.edges.len() {
            let (v1, v2) = self.input.figure.edges[ei];

            let p1 = vertices[v1];
            let p2 = vertices[v2];
            let current_penalty = self.edge_penalty(v1, v2, p1, p2);
            assert_eq!(current_penalty, 0);

            for d1 in 0..self.n_cands() {
                let tp1 = p1 + self.dv(d1);
                for d2 in 0..self.n_cands() {
                    let tp2 = p2 + self.dv(d2);
                    let new_penalty = self.edge_penalty(v1, v2, tp1, tp2);

                    let mut ok = self.contains_s(tp1, tp2) && new_penalty == 0;

                    if !ok {
                        clauses.push(vec![-self.lit(v1, d1), -self.lit(v2, d2)]);
                    }
                }
            }
        }

        clauses
    }

    fn reconstruct_positions(&self, positions: &Vec<Point>, solution: &Vec<bool>) -> Vec<Point> {
        let mut new_positions = positions.clone();
        for (v, p) in positions.iter().enumerate() {
            for d in 0..self.n_cands() {
                if solution[self.lit(v, d) as usize] {
                    new_positions[v] = *p + self.dv(d);
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
            .map(|clause| clause.iter().map(|l| (*l).abs()).max().unwrap_or(0))
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

    fn read_solution() -> Option<Vec<bool>> {
        use std::io::BufRead;

        let mut reader = std::io::BufReader::new(std::fs::File::open("sat_out.txt").unwrap());
        let mut line = String::new();
        reader.read_line(&mut line);

        if line.starts_with("UNSAT") {
            None
        } else {
            let lits: Vec<_> = line
                .split_whitespace()
                .map(|l| l.parse::<i64>().unwrap())
                .collect();
            let n_lits = 1 + lits.iter().map(|l| l.abs()).max().unwrap() as usize;

            let mut sol = vec![false; n_lits];
            for l in lits {
                sol[l.abs() as usize] = l >= 0;
            }
            Some(sol)
        }
    }

    fn solve_by_glucose(&self, clauses: &Vec<Vec<i64>>) -> Option<Vec<bool>> {
        Self::write_clauses(clauses);

        // TODO: stdoutの最後の行見てSATかUNSATかあれしたほうがいいかもしれない
        std::process::Command::new(&self.config.glucose_path)
            .args(vec!["sat_in.txt", "sat_out.txt"])
            .status()
            .unwrap()
            .success();

        Self::read_solution()
    }

    //
    // メインループ部分
    //

    fn step(&mut self, positions: &Vec<Point>) -> Option<Vec<Point>> {
        let clauses = self.generate_clauses(positions);
        self.solve_by_glucose(&clauses)
            .map(|solution| self.reconstruct_positions(positions, &solution))
    }

    fn dump(&self, positions: &Vec<Point>, i_iter: i64) {
        let output = Output {
            vertices: positions.clone(),
            bonuses: Default::default(),
        };
        let writer = std::io::BufWriter::new(
            std::fs::File::create(format!("out/viz{:06}.svg", i_iter)).unwrap(),
        );
        icfpc2021::paths::render_pose_svg(&self.input, &output, writer);

        let writer = std::io::BufWriter::new(
            std::fs::File::create(format!("out/sol{:06}.json", i_iter)).unwrap(),
        );
        icfpc2021::write_output_to_writer(&output, writer);

        let score = icfpc2021::compute_score(&self.input, &output);
        dbg!(score);
    }

    /// 更新できたらSome、全く更新できなかったらNone
    fn solve(&mut self, mut positions: Vec<Point>) -> Option<Vec<Point>> {
        // アゲていく！
        let mut i_iter: i64 = 0;
        loop {
            self.dump(&positions, i_iter);

            let mut next_positions = None;
            for neighbor in self.config.min_neighbor..=self.config.max_neighbor {
                if neighbor % 2 != 1 {
                    continue;
                }
                self.neighbor = neighbor;

                next_positions = self.step(&positions);
                if next_positions.is_some() {
                    break;
                } else {
                    eprintln!("\n\n\n===\nFAILED WITH NEIGHBOR={}\n===\n\n\n", neighbor);
                }
            }

            match next_positions {
                None => {
                    return if i_iter == 0 { None } else { Some(positions) };
                }
                Some(next_positions) => {
                    positions = next_positions;
                }
            }
            i_iter += 1;
        }
    }
}

//
// main
//

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

        #[structopt(short, long, default_value = "3")]
        neighbor: i64,

        #[structopt(short, long, default_value = "11")]
        max_neighbor: i64,

        #[structopt(long)]
        initial_relax: Option<f64>,
    }
    let args = Args::from_args();
    dbg!(&args);

    let input = read_input_from_file(&args.input_path);
    let output = read_output_from_file(&args.output_path);
    let initial_score = compute_score(&input, &output);

    // 解く
    let config = Config {
        glucose_path: args.glucose_path.clone(),
        min_neighbor: args.neighbor,
        max_neighbor: args.max_neighbor,
        initial_relax: args.initial_relax,
    };
    let mut sat_calibrator = SatHillclimber::new(input, config);
    let sol = sat_calibrator.solve(output.vertices).unwrap();

    sat_calibrator.dump(&sol, 999999);
    dbg!(initial_score);
}
