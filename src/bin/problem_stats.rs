use icfpc2021::*;

#[derive(Debug)]
struct ProblemStat {
    problem_id: i64,
    url: String,
    score_weight: f64,
    max_coord: i64,
    mean_edge_len: f64,
    tol_for_mean_edge: f64,
    epsilon: i64,
    n_hole_vs: usize,
    n_figure_vs: usize,
    n_figure_es: usize,
    n_triangles: usize,
}

fn max_coord(input: &Input) -> i64 {
    input.hole.iter().map(|p| p.0.max(p.1)).max().unwrap()
}

fn n_triangles(figure: &Figure) -> usize {
    let n_vs = figure.vertices.len();
    let mut adj: Vec<Vec<bool>> = vec![vec![false; n_vs]; n_vs];
    for e in &figure.edges {
        adj[e.0][e.1] = true;
        adj[e.1][e.0] = true;
    }

    let mut n_triangles = 0;
    for u in 0..n_vs {
        for v in (u + 1)..n_vs {
            if !adj[u][v] {
                continue;
            }

            for w in (v + 1)..n_vs {
                if adj[u][w] && adj[w][v] {
                    n_triangles += 1;
                }
            }
        }
    }

    n_triangles
}

fn mean_edge_len(figure: &Figure) -> f64 {
    let mut lens = vec![];
    for e in &figure.edges {
        let p0 = figure.vertices[e.0];
        let p1 = figure.vertices[e.1];
        let d = ((p0 - p1).abs2() as f64).sqrt();
        lens.push(d);
    }

    lens.iter().sum::<f64>() / (lens.len() as f64)
}

impl ProblemStat {
    pub fn new(problem_id: i64, input: &Input) -> Self {
        let mean_edge_len = mean_edge_len(&input.figure);
        let tol_for_mean_edge =
            (((input.epsilon as f64) / 1_000_000.0 + 1.0).sqrt() - 1.0) * mean_edge_len;

        ProblemStat {
            problem_id,
            url: format!("https://poses.live/problems/{}", problem_id),
            score_weight: 1000.0
                * (((input.figure.vertices.len() * input.figure.edges.len() * input.hole.len())
                    as f64)
                    / 6.0)
                    .log2(),
            max_coord: max_coord(input),
            mean_edge_len,
            tol_for_mean_edge,
            epsilon: input.epsilon,
            n_hole_vs: input.hole.len(),
            n_figure_vs: input.figure.vertices.len(),
            n_figure_es: input.figure.edges.len(),
            n_triangles: n_triangles(&input.figure),
        }
    }

    pub fn println(&self) {
        print!("{}\t{}\t{}", self.problem_id, self.url, self.score_weight);
        print!(
            "\t{}\t{}\t{}\t{}",
            self.max_coord, self.epsilon, self.mean_edge_len, self.tol_for_mean_edge
        );
        print!(
            "\t{}\t{}\t{}\t{}",
            self.n_hole_vs, self.n_figure_vs, self.n_figure_es, self.n_triangles
        );
        println!();
    }
}

fn main() {
    let mut problem_stats = vec![];

    for entry in glob::glob("./problems/*.json").unwrap() {
        let path = entry.unwrap();

        let path = path.to_str().unwrap();
        let filename = path
            .split("/")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .to_owned();
        let problem_id: i64 = filename.split('.').collect::<Vec<_>>()[0].parse().unwrap();

        let file = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);
        let input: Input = serde_json::from_reader(reader).unwrap();

        problem_stats.push(ProblemStat::new(problem_id, &input));
    }

    problem_stats.sort_by_key(|ps| ps.problem_id);

    for ps in problem_stats {
        ps.println()
    }
}
