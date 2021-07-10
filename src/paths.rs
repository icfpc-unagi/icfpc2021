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

pub fn path<T: std::fmt::Display>(edges: &[(usize, usize)], vertices: &[P<T>]) -> String {
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
    let hole_polygon = paths::polygon(&prob.hole);
    let figure_path = paths::path(&prob.figure.edges, &prob.figure.vertices);

    let svg = svg::Document::new()
        .set("height", 500)
        .set("width", 500)
        .set(
            "viewBox",
            (
                0,
                0,
                prob.hole.iter().map(|p| p.0).max().unwrap(),
                prob.hole.iter().map(|p| p.1).max().unwrap(),
            ),
        )
        .add(
            Polygon::new()
                .set("fill", "grey")
                .set("points", hole_polygon),
        )
        .add(Path::new().set("stroke", "red").set("d", figure_path));

    svg::write(w, &svg)
}

pub fn render_pose_svg<W: io::Write>(prob: &Input, pose: &Output, w: W) -> io::Result<()> {
    let hole_polygon = paths::polygon(&prob.hole);
    let figure_path = paths::path(&prob.figure.edges, &pose.vertices);

    let svg = svg::Document::new()
        .set("height", 500)
        .set("width", 500)
        .set(
            "viewBox",
            (
                0,
                0,
                prob.hole.iter().map(|p| p.0).max().unwrap(),
                prob.hole.iter().map(|p| p.1).max().unwrap(),
            ),
        )
        .add(
            Polygon::new()
                .set("fill", "grey")
                .set("points", hole_polygon),
        )
        .add(Path::new().set("stroke", "red").set("d", figure_path));

    svg::write(w, &svg)
}
