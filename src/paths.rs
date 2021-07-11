use crate::*;
use std::fmt::*;
use std::io;
use svg::node;
use svg::node::element;

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
	let padding = 2;
	let all_points = prob.hole.iter().chain(vertices.iter());
	let left = all_points.clone().map(|p| p.0).min().unwrap() - padding;
	let right = all_points.clone().map(|p| p.0).max().unwrap() + padding;
	let top = all_points.clone().map(|p| p.1).min().unwrap() - padding;
	let bottom = all_points.map(|p| p.1).max().unwrap() + padding;
	let mut hole_polygon = polygon_path(&prob.hole);
	hole_polygon.push_str(&polygon_path(&[
		P(left, top),
		P(right, top),
		P(right, bottom),
		P(left, bottom),
	]));

	let mut edges_ok = Vec::new();
	let mut edges_short = Vec::new();
	let mut edges_long = Vec::new();
	let mut edges_out = Vec::new();
	for &(i, j) in prob.figure.edges.iter() {
		if P::contains_s(&prob.hole, (vertices[i], vertices[j])) {
			match stretch_within(
				(vertices[i] - vertices[j]).abs2(),
				(prob.figure.vertices[i] - prob.figure.vertices[j]).abs2(),
				prob.epsilon,
			) {
				Ordering::Less => &mut edges_short,
				Ordering::Equal => &mut edges_ok,
				Ordering::Greater => &mut edges_long,
			}
		} else {
			&mut edges_out
		}
		.push((i, j))
	}
	let figure_ok_path = paths::segments(&edges_ok, &vertices);
	let figure_short_path = paths::segments(&edges_short, &vertices);
	let figure_long_path = paths::segments(&edges_long, &vertices);
	let figure_out_path = paths::segments(&edges_out, &vertices);

	let mut svg = svg::Document::new()
		.set("height", 500)
		.set("width", 500)
		.set("viewBox", (left, top, right - left, bottom - top))
		.add(
			element::Path::new()
				.set("class", "hole")
				.set("style", "fill:#00000066;fill-rule:evenodd;")
				.set("d", hole_polygon),
		);
	for bonus in &prob.bonuses {
		svg = svg.add(
			element::Circle::new()
				.set("cx", bonus.position.0)
				.set("cy", bonus.position.1)
				.set("r", 5)
				.set("style", "fill:#ffff0066;")
				.set("title", format!("{:?}", bonus.bonus)),
		);
	}
	svg = svg.add(
		element::Path::new()
			.set("class", "ok")
			.set("style", "stroke:#0000ff;stroke-linecap:round")
			.set("d", figure_ok_path),
	);
	if !figure_short_path.is_empty() {
		svg = svg.add(
			element::Path::new()
				.set("class", "short")
				.set("style", "stroke:#00ff99;stroke-linecap:round")
				.set("d", figure_short_path),
		);
	}
	if !figure_long_path.is_empty() {
		svg = svg.add(
			element::Path::new()
				.set("class", "long")
				.set("style", "stroke:#ff0099;stroke-linecap:round")
				.set("d", figure_long_path),
		);
	}
	if !figure_out_path.is_empty() {
		svg = svg.add(
			element::Path::new()
				.set("class", "out")
				.set("style", "stroke:#ff0000;stroke-linecap:round")
				.set("d", figure_out_path),
		);
	}

	let mut g = element::Group::new().set("fill", "#333333");
	for (i, p) in prob.hole.iter().enumerate() {
		g = g.add(
			element::Circle::new()
				.set("cx", p.0)
				.set("cy", p.1)
				.set("r", 0.5)
				.add(element::Title::new().add(node::Text::new(format!("hole:{}", i)))),
		);
	}
	svg = svg.add(g);

	let mut tabindex = 0;
	let mut g = element::Group::new().set("fill", "#999999");
	for (i, p) in vertices.iter().enumerate() {
		g = g.add(
			element::Circle::new()
				.set("cx", p.0)
				.set("cy", p.1)
				.set("r", 0.5)
				.set("tabindex", tabindex)
				.set("i", i)
				.add(element::Title::new().add(node::Text::new(format!("v:{}", i)))),
		);
		tabindex += 1;
	}
	svg = svg.add(g);

	svg::write(w, &svg)
}
