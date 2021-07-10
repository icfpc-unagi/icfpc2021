use crate::*;
use std::fmt::*;
use std::io;
use svg::node::element::*;

pub fn polygon<T: std::fmt::Display>(points: &[P<T>]) -> String {
    let mut s = String::new();
    for P(x, y) in points {
        write!(s, "{},{} ", x, y).unwrap();
    }
    s
}

fn polygon_path<T: std::fmt::Display>(points: &[P<T>]) -> String {
    let mut s = String::new();
    for (i, P(x, y)) in points.iter().enumerate() {
        write!(s, "{}{} {}", if i == 0 { "M" } else { "L" }, x, y).unwrap();
    }
    s
}

pub fn segments<T: std::fmt::Display>(edges: &[(usize, usize)], vertices: &[P<T>]) -> String {
    let mut s = String::new();
    for &(i, j) in edges {
        write!(
            s,
            "M{} {}L{} {}",
            vertices[i].0, vertices[i].1, vertices[j].0, vertices[j].1
        )
        .unwrap();
    }
    s
}

pub fn render_problem_svg<W: io::Write>(prob: &Input, w: W) -> io::Result<()> {
    render_svg(prob, &prob.figure.vertices, w)
}

pub fn render_pose_svg<W: io::Write>(prob: &Input, pose: &Output, w: W) -> io::Result<()> {
    render_svg(prob, &pose.vertices, w)
}

fn render_svg<W: io::Write>(prob: &Input, vertices: &Vec<Point>, w: W) -> io::Result<()> {
    let padding = 1;
    let width = prob.hole.iter().map(|p| p.0).max().unwrap() + padding;
    let height = prob.hole.iter().map(|p| p.1).max().unwrap() + padding;
    let mut hole_polygon = polygon_path(&prob.hole);
    hole_polygon.push_str(&polygon_path(&[
        P(0, 0),
        P(width, 0),
        P(width, height),
        P(0, height),
    ]));
    let figure_path = paths::segments(&prob.figure.edges, &vertices);

    let svg = svg::Document::new()
        // .set("height", 500)
        .set("width", 500)
        .set("viewBox", (0, 0, width, height))
        .add(
            Path::new()
                .set("style", "fill:#00000066;fill-rule:evenodd;stroke:none;")
                .set("d", hole_polygon),
        )
        .add(
            Path::new()
                .set("style", "fill:none;stroke:#ff0000;stroke-linecap:round")
                .set("d", figure_path),
        );

    svg::write(w, &svg)
}
