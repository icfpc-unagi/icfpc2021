use icfpc2021::*;
use svg::node::element::*;

// Usage: show_problem [problem.json]
fn main() -> std::io::Result<()> {
  let args: Vec<_> = std::env::args().collect();
  let prob = if args.len() < 2 {
		read_input();
  } else {
		serde_json::from_reader(File::open(&args[2])?)?;
	}

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
