use icfpc2021::paths::render_pose_svg;
use icfpc2021::{util::*, *};
use rand::prelude::*;

const Eps: f64 = 1e-5;

//
// 浮動小数点数ベクトル用ユーティリティ
//
type Pf = P<f64>;

fn into_pf(p: Point) -> Pf {
    P(p.0 as f64, p.1 as f64)
}

fn into_pi(p: Pf) -> Point {
    P(p.0.round() as i64, p.1.round() as i64)
}

fn mul(p: Pf, x: f64) -> Pf {
    P(p.0 * x, p.1 * x)
}

fn normalize(p: Pf) -> Pf {
    mul(p, 1.0 / (p.abs2().sqrt() + 1e-8))
}

//
// 初期配置
//

fn generate_initial_positions(input: &Input) -> Vec<Pf> {
    let mut ps = vec![];
    for _ in 0..input.figure.vertices.len() {
        let th: f64 = thread_rng().gen::<f64>() * 2.0 * std::f64::consts::PI;
        ps.push(P(th.cos(), th.sin()))
    }

    ps
}

//
// 本体
//

struct Config {
    grad_clip_initail: f64,
    grad_clip_decay: f64, // `D`
    momentum: f64,        // `1.0 - F`
    alpha_replusive: f64, // `A`
    alpha_edge: f64,      // `K`
    alpha_center: f64,    // `G`
}

struct Instance {
    input: Input,
    config: Config,

    positions: Vec<Pf>,
    momentums: Vec<Pf>,

    n_vs: usize,
    n_es: usize,
    adj: Vec<Vec<bool>>,
    deg: Vec<usize>,
    center: Pf,
    grad_clip_threshold: f64,
}

impl Instance {
    fn new(input: Input, config: Config) -> Self {
        // TODO: 多重辺があるといかいう話があった気がする
        let n_vs = input.figure.vertices.len();
        let n_es = input.figure.edges.len();
        let mut deg = vec![0; n_vs];
        let mut adj = vec![vec![false; n_vs]; n_vs];
        for e in &input.figure.edges {
            deg[e.0] += 1;
            deg[e.1] += 1;
            adj[e.0][e.1] = true;
            adj[e.1][e.0] = true;
        }

        let positions = generate_initial_positions(&input);
        let momentums = vec![P(0.0, 0.0); n_vs];

        // TODO
        let center = P(0.0, 0.0);

        Self {
            grad_clip_threshold: config.grad_clip_initail,
            input,
            positions,
            momentums,
            deg,
            adj,
            n_vs,
            n_es,
            config,
            center,
        }
    }

    fn grad(&self) -> Vec<Pf> {
        let mut grad = vec![];

        for (i, pi) in self.positions.iter().enumerate() {
            let mut g = P(0.0, 0.0);

            for (j, pj) in self.positions.iter().enumerate() {
                if i == j {
                    continue;
                }

                let v = *pj - *pi; // dir
                let a = v.abs2().sqrt(); // abs
                if self.adj[i][j] {
                    g += mul(v, self.config.alpha_edge / (self.deg[i] as f64));
                }
                g -= mul(
                    v,
                    self.config.alpha_replusive / (self.n_es as f64) / (Eps + a * a * a)
                        * (self.deg[j] as f64),
                );
            }

            g -= mul(*pi - self.center, self.config.alpha_center);
            grad.push(g);
        }

        grad
    }

    fn step(&mut self) {
        let grad = self.grad();

        for (m, g) in self.momentums.iter_mut().zip(grad) {
            *m = mul(*m + g, self.config.momentum);

            let a = m.abs2().sqrt();
            if a > self.grad_clip_threshold {
                *m = mul(*m, self.grad_clip_threshold / a);
            }
        }

        for (p, m) in self.positions.iter_mut().zip(self.momentums.iter()) {
            *p += *m;
        }

        self.grad_clip_threshold *= self.config.grad_clip_decay;
    }

    fn render_svg(&self, out_path: impl AsRef<std::path::Path>) {
        let svg_size = 500;
        let positions = self.positions.clone();

        let min_x = positions
            .iter()
            .map(|p| ordered_float::OrderedFloat(p.0))
            .min()
            .unwrap()
            .into_inner();
        let max_x = positions
            .iter()
            .map(|p| ordered_float::OrderedFloat(p.0))
            .max()
            .unwrap()
            .into_inner();
        let min_y = positions
            .iter()
            .map(|p| ordered_float::OrderedFloat(p.1))
            .min()
            .unwrap()
            .into_inner();
        let max_y = positions
            .iter()
            .map(|p| ordered_float::OrderedFloat(p.1))
            .max()
            .unwrap()
            .into_inner();

        let longer_edge = (max_x - min_x).max(max_y - min_y) + 1e-8;
        let zoom = svg_size as f64 / longer_edge;
        let positions: Vec<_> = positions
            .into_iter()
            .map(|p| P((p.0 - min_x) * zoom, (p.1 - min_y) * zoom))
            .collect();

        let mut svg = svg::Document::new()
            .set("height", svg_size)
            .set("width", svg_size)
            .set("viewBox", (0, 0, svg_size, svg_size));

        svg = svg.add(
            svg::node::element::Path::new()
                .set("style", "fill:none;stroke:#0000ff")
                .set("d", paths::segments(&self.input.figure.edges, &positions)),
        );

        for p in &positions {
            svg = svg.add(
                svg::node::element::Circle::new()
                    .set("cx", p.0)
                    .set("cy", p.1)
                    .set("r", 3)
                    .set("style", "fill:#ffff0066;stroke:none;"),
            );
        }

        let writer = std::io::BufWriter::new(std::fs::File::create(out_path).unwrap());
        svg::write(writer, &svg);
    }
}

//
// 配置探しくん
//

fn is_inside(input: &Input, positions: &Vec<Point>) -> bool {
    for p in positions {
        if !P::contains_p(&input.hole, *p) == -1 {
            return false;
        }
    }

    for e in &input.figure.edges {
        if !P::contains_s(&input.hole, (positions[e.0], positions[e.1])) {
            return false;
        }
    }

    true
}

fn transform_zoom_rotate_shift(p: Pf, zoom: f64, rotate_theta: f64, shift: Pf) -> Pf {
    // zoom
    let p = P(p.0 * zoom, p.1 * zoom);

    // rotate
    let (sin, cos) = rotate_theta.sin_cos();
    let p = P(cos * p.0 - sin * p.1, sin * p.0 + cos * p.1);

    // shift
    let p = P(p.0 + shift.0, p.1 + shift.1);

    p
}

fn find_best_zoom(
    input: &Input,
    src_positions: &Vec<Pf>,
    rotate: f64,
    shift: Pf,
) -> (f64, Vec<Point>) {
    let mut zoom_lb = 0.0;
    let mut zoom_ub = 1e5;
    let mut positions_ret = vec![];

    for _ in 0..100 {
        let zoom = (zoom_lb + zoom_ub) / 2.0;

        let positions_f: Vec<_> = src_positions
            .iter()
            .map(|p| transform_zoom_rotate_shift(*p, zoom, rotate, shift.clone()))
            .collect();
        let positions_i: Vec<_> = positions_f.iter().map(|p| into_pi(*p)).collect();

        if is_inside(input, &positions_i) {
            zoom_lb = zoom;
            positions_ret = positions_i
        } else {
            zoom_ub = zoom;
        }
    }

    (zoom_lb, positions_ret)
}

/// 適当に中央と向きをランダムで試しできるだけ大きくする、一番大きくするところをとっとく
fn find_best_place(input: &Input, src_positions: &Vec<Pf>) -> Vec<Point> {
    let max_x = input.hole.iter().map(|p| p.0).max().unwrap() as f64;
    let max_y = input.hole.iter().map(|p| p.1).max().unwrap() as f64;

    let mut best = (0.0, vec![]);

    for _ in 0..1000 {
        let rotate = thread_rng().gen::<f64>() * 2.0 * std::f64::consts::PI;
        let shift = P(
            thread_rng().gen::<f64>() * max_x,
            thread_rng().gen::<f64>() * max_y,
        );
        if P::contains_p(&input.hole, into_pi(shift)) == -1 {
            continue;
        }

        let tmp = find_best_zoom(input, src_positions, rotate, shift);
        dbg!(rotate, shift, tmp.0);

        if tmp.0 > best.0 {
            best = tmp;
        }
    }

    best.1
}

//
// エントリポイント
//

fn main() {
    let input = read_input();
    let config = Config {
        grad_clip_initail: 0.1,
        grad_clip_decay: 0.999,
        momentum: 0.5,
        alpha_replusive: 0.5,
        alpha_edge: 1.0,
        alpha_center: 0.05,
    };
    let mut instance = Instance::new(input, config);

    for i_iter in 0..1000 {
        instance.step();
        if i_iter % 1000 == 0 {
            instance.render_svg(format!("out/{:06}.svg", i_iter))
        }
    }

    // 配置
    let positions = find_best_place(&instance.input, &instance.positions);

    let output = Output {
        vertices: positions,
        bonuses: Default::default(),
    };
    write_output(&output);

    let file = std::fs::File::create("out.svg").unwrap();
    let writer = std::io::BufWriter::new(file);
    render_pose_svg(&instance.input, &output, writer);
}
