use icfpc2021::{*, util::*};
use icfpc2021::paths::render_pose_svg;

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
// 状態とか設定とか色々
//

struct Config {
    scale: i64,
    alpha_edge: f64,
    alpha_hole_score: f64,
    alpha_hole_gravity: f64,
    alpha_replusive: f64,
    alpha_center: f64,
    momentum: f64,
    grad_clip: f64,
}

struct State {
    config: Config,
    input: Input,
    positions: Vec<Point>,
    momentums: Vec<Pf>,
    adjmat: Vec<Vec<bool>>,
    deg: Vec<usize>,
    center: Pf,
}

fn compute_deg(input: &Input) -> Vec<usize> {
    let mut deg = vec![0; input.figure.vertices.len()];
    for e in &input.figure.edges {
        deg[e.0] += 1;
        deg[e.1] += 1;
    }
    deg
}

fn compute_adjmat(input: &Input) -> Vec<Vec<bool>> {
    let n_vs = input.hole.len();
    let mut adjmat = vec![vec![false; n_vs]; n_vs];

    for u in 0..n_vs {
        for v in (u + 1)..n_vs {
            let b = P::contains_s(&input.hole, (input.hole[u], input.hole[v]));
            adjmat[u][v] = b;
            adjmat[v][u] = b;
        }
    }

    adjmat
}

fn compute_center(input: &Input) -> Pf {
    let max_x = input.hole.iter().map(|p| ordered_float::OrderedFloat::from(p.0 as f64)).max().unwrap();
    let max_y = input.hole.iter().map(|p| ordered_float::OrderedFloat::from(p.1 as f64)).max().unwrap();
    P(max_x.into_inner() / 2.0, max_y.into_inner() / 2.0)
}

impl State {
    fn new(raw_input: Input, config: Config) -> Self {
        let mut scaled_input = raw_input.clone();
        scaled_input.hole.iter_mut().for_each(|p| *p *= config.scale);
        scaled_input.figure.vertices.iter_mut().for_each(|p| *p *= config.scale);

        // TODO: もう少し良い初期配置を
        // let positions: Vec<_> = (0..scaled_input.figure.vertices.len()).map(|i| scaled_input.hole[i % scaled_input.hole.len()]).collect();
        let positions: Vec<_> = (0..scaled_input.figure.vertices.len())
            .map(|i| scaled_input.hole[i * scaled_input.hole.len() / scaled_input.figure.vertices.len()]).collect();
        let momentums =  vec![P(0.0, 0.0); positions.len()];
        let adjmat = compute_adjmat(&scaled_input);
        let deg = compute_deg(&scaled_input);
        let center = compute_center(&scaled_input);

        State {
            config,
            input: scaled_input,
            adjmat,
            deg,
            positions,
            momentums,
            center,
        }
    }

    fn distance_original(&self, v0: usize, v1: usize) -> f64 {
        ((self.input.figure.vertices[v0] - self.input.figure.vertices[v1]).abs2() as f64).sqrt()
    }

    fn distance_current(&self, v0: usize, v1: usize) -> f64 {
        ((self.positions[v0] - self.positions[v1]).abs2() as f64).sqrt()
    }

    fn dir(&self, v0: usize, v1: usize) -> Pf {
        normalize(into_pf(self.positions[v1] - self.positions[v0]))
    }
}

//
// 勾配
//

fn rec(i: usize, j: usize, via: &Vec<Vec<usize>>, path: &mut Vec<usize>) {
    if via[i][j] == !0 {
        path.push(i);
    } else {
        let k = via[i][j];
        rec(i, k, via, path);
        rec(k, j, via, path);
    }
}

/// p0からp1にhole内部だけを通って行く。通過する端点の列を返す。
fn shortest_path(state: &State, p0: Point, p1: Point) -> (f64, Vec<usize>) {
    let mut adjmat = state.adjmat.clone();

    let n_hole_vs = state.input.hole.len();
    for u in 0..n_hole_vs {
        adjmat[u].push(P::contains_s(&state.input.hole, (state.input.hole[u], p0)));
        adjmat[u].push(P::contains_s(&state.input.hole, (state.input.hole[u], p1)));
    }

    let mut row0: Vec<bool> = (0..n_hole_vs).map(|i| adjmat[i][n_hole_vs]).collect();
    let mut row1: Vec<bool> = (0..n_hole_vs).map(|i| adjmat[i][n_hole_vs + 1]).collect();
    let n_vs = n_hole_vs + 2;
    row0.resize(n_vs, false);
    row1.resize(n_vs, false);
    adjmat.push(row0);
    adjmat.push(row1);

    let mut ps = state.input.hole.clone();
    ps.push(p0);
    ps.push(p1);

    let mut dst = vec![vec![1e30; n_vs]; n_vs];
    for u in 0..n_vs {
        for v in (u + 1)..n_vs {
            if adjmat[u][v] {
                let d = ((ps[u] - ps[v]).abs2() as f64).sqrt();
                dst[u][v] = d;
                dst[v][u] = d;
            }
        }
    }

    let mut via = vec![vec![!0; n_vs]; n_vs];
    for k in 0..n_vs {
        for i in 0..n_vs {
            for j in 0..n_vs {
                if dst[i][k] + dst[k][j] < dst[i][j] {
                    dst[i][j] = dst[i][k] + dst[k][j];
                    via[i][j] = k;
                }
            }
        }
    }

    let mut path = vec![];
    let s = n_hole_vs;
    let t = n_hole_vs + 1;
    rec(s, t, &via, &mut path);
    assert_eq!(path.remove(0), s);

    (dst[s][t], path)
}

fn grad_edge(state: &State) -> Vec<Pf> {
    let mut grad = vec![P(0.0, 0.0); state.positions.len()];

    let mut n_crossing_edges = 0;
    for e in &state.input.figure.edges {
        let (i, j) = *e;
        let d0 = 0.0;  // state.distance_original(i, j);
        // TODO: awoeifaweiwaefawejfweaiaweofweifweafowefaweijfweafea
        if true || P::contains_s(&state.input.hole, (state.positions[e.0], state.positions[e.1])) {
            let v = into_pf(state.positions[e.1] - state.positions[e.0]);  //state.dir(e.0, e.1);
            let d1 = v.abs2().sqrt();
            let v1 = mul(v, 1.0 / (d1 + 1e-8));
            grad[i] += mul(v1, (d1 - d0) * state.config.alpha_edge);
            grad[j] -= mul(v1, (d1 - d0) * state.config.alpha_edge);
        } else {
            n_crossing_edges += 1;

            let (d1, path) = shortest_path(state, state.positions[i], state.positions[j]);
            assert!(path.len() > 0);

            let vi = normalize(into_pf(state.input.hole[*path.first().unwrap()] - state.positions[i]));
            grad[i] += mul(vi, (d1 - d0) * state.config.alpha_edge);

            let vj = normalize(into_pf(state.input.hole[*path.last().unwrap()] - state.positions[j]));
            grad[j] += mul(vj, (d1 - d0) * state.config.alpha_edge);
        }
    }

    eprintln!("Cross: {} / {}", n_crossing_edges, state.input.figure.edges.len());

    for (g, e) in grad.iter_mut().zip(state.deg.iter()) {
        *g = mul(*g, 1.0 / (*e as f64));
    }

    grad
}


fn grad_replusive(state: &State) -> Vec<Pf> {
    let mut grad = vec![P(0.0, 0.0); state.positions.len()];

    let n_vs = state.positions.len();
    let mut adjmut = vec![vec![false; n_vs]; n_vs];
    for e in &state.input.figure.edges {
        adjmut[e.0][e.1] = true;
        adjmut[e.1][e.0] = true;
    }

    let m = state.input.figure.edges.len() as f64;

    for (i, pi) in state.positions.iter().enumerate() {
        for (j, pj) in state.positions[i + 1..].iter().enumerate() {
            /*
            if adjmut[i][j] {
                continue;
            }
             */

            let v = into_pf(*pi - *pj);
            let r2 = v.abs2();
            let v1 = mul(v, 1.0 / (r2.sqrt() + 1e-8));

            grad[i] += mul(v1, state.config.alpha_replusive / (r2 + 1e-8) / m * (state.deg[i] as f64));
            grad[j] += mul(v1, -state.config.alpha_replusive / (r2 + 1e-8) / m * (state.deg[j] as f64));
        }
    }

    grad
}

fn grad_center(state: &State) -> Vec<Pf> {
    state.positions.iter().map(|p| mul(state.center - into_pf(*p),  state.config.alpha_center)).collect()
}

fn grad_hole_gravity(state: &State) -> Vec<Pf> {
    let mut grad = vec![P(0.0, 0.0); state.positions.len()];
    // let mut loss_hole = 0.0;
    for h in &state.input.hole {
        for (i, p) in state.positions.iter().enumerate() {
            let v = into_pf(*h - *p);
            let r2 = v.abs2() + 1.0;
            let d = mul(v, 1.0 / r2);

            // loss_hole += -state.config.alpha_hole / r2.sqrt();
            grad[i] += mul(d, state.config.alpha_hole_gravity / r2);
        }
    }
    grad
}

fn grad_hole_score(state: &State) -> Vec<Pf> {
    let mut grad = vec![P(0.0, 0.0); state.positions.len()];

    for h in &state.input.hole {
        let (i, p) = state.positions.iter().enumerate().min_by_key(|(_, p)| (**p - *h).abs2()).unwrap();
        let v = *h - *p;
        grad[i] += into_pf(v) * state.config.alpha_hole_score;
    }

    grad
}

/// 各頂点への勾配を返す
fn grad(state: &State) -> Vec<Pf> {
    let mut grad = grad_edge(state);

    for (g, gh) in grad.iter_mut().zip(grad_replusive(&state)) {
        *g += gh;
    }

    for (g, gh) in grad.iter_mut().zip(grad_center(&state)) {
        *g += gh;
    }

    /*
    for (g, gh) in grad.iter_mut().zip(grad_hole_gravity(&state)) {
        *g += gh;
    }

    for (g, gh) in grad.iter_mut().zip(grad_hole_score(&state)) {
        *g += gh;
    }
     */

    grad
}

/// 基本posをvecだけ動かすんだけど、はみ出す場合は中に留める
fn update_point(hole: &Vec<Point>, p: Point, v: Pf) -> Point {
    let tp = into_pi(into_pf(p) + v);
    return tp;  // TODO fwaaweifawofpweaif

    if P::contains_p(hole, tp) >= 0 {
        tp
    } else {
        // TODO: 全く動かさないのではなく壁にぶつかるまで動かした方が良さそうだが後で……

        // TODO: awpefiawejfweaiawefpaweifawefaweiwaoefapweoai
        p
    }
}

fn step(state: &mut State, eta: f64) {
    let grad = grad(&state);

    for ((p,m), g) in state.positions.iter_mut().zip(state.momentums.iter_mut()).zip(grad.into_iter()) {
        // momentum
        *m = mul(*m, state.config.momentum) + g;

        // gradient clipping
        let abs = m.abs2().sqrt();
        if abs > state.config.grad_clip {
            *m = mul(*m, state.config.grad_clip / abs);
        }

        *p = update_point(&state.input.hole, *p, *m * eta);

    }

    // dbg!(&state.positions);
}

//
// エントリポイント
//



fn main() {
    let raw_input = read_input();

    let config = Config {
        scale: 1000,
        alpha_edge: 1e-3,
        alpha_hole_score: 1e-1,
        alpha_hole_gravity: 1e2,
        alpha_replusive: 3e7,
        alpha_center: 0.0,
        momentum: 0.5,
        grad_clip: 1e9,
    };


    let config = Config {
        scale: 1000,
        alpha_edge: 3e-1,  // K
        alpha_hole_score: 0.0,
        alpha_hole_gravity: 0.0,
        alpha_replusive: 1e13,  // A
        alpha_center: 1e-2,  //0.05,
        momentum: 0.9,  // 1 - F
        grad_clip: 1e8,
    };

    let mut state = State::new(raw_input.clone(), config);

    let mut eta = 1.0;
    for i_iter in 0..100000 {
        state.config.grad_clip *= 0.999;  // D

        // eta *= 0.99995;
        dbg!(&i_iter, &eta);
        step(&mut state, eta);

        if i_iter % 1000 == 0 {
            let positions = state.positions.iter().map(|p| P(p.0 / state.config.scale, p.1 / state.config.scale)).collect();
            let raw_output = Output { vertices: positions, bonuses: Default::default() };
            let file = std::fs::File::create(format!("out/{:06}.svg", i_iter)).unwrap();
            let writer = std::io::BufWriter::new(file);
            render_pose_svg(&raw_input, &raw_output, writer);
        }
    }

    let positions = state.positions.iter().map(|p| P(p.0 / state.config.scale, p.1 / state.config.scale)).collect();
    let raw_output = Output { vertices: positions, bonuses: Default::default() };
    write_output(&raw_output);

    let file = std::fs::File::create("out.svg").unwrap();
    let writer = std::io::BufWriter::new(file);
    render_pose_svg(&raw_input, &raw_output, writer);
}