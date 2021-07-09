use icfpc2021::*;
use svg::node::element::*;

fn main() -> std::io::Result<()> {
	let prob = read_input();

	let hole_polygon = paths::polygon(&prob.hole);

	let figure_path = paths::path(&prob.figure.edges, &prob.figure.vertices);

	let svg = svg::Document::new()
		.add(
			Polygon::new()
				.set("fill", "grey")
				.set("points", hole_polygon),
		)
		.add(Path::new().set("stroke", "red").set("d", figure_path));

	svg::write(std::io::stdout(), &svg)?;

	Ok(())
}
