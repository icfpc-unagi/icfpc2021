use icfpc2021::*;
use svg::node::element::*;
use std::fs::File;

// Usage: show_pose problem.json pose.json
fn main() -> std::io::Result<()> {
  let args: Vec<_> = std::env::args().collect();
  if args.len() < 3 {
    eprintln!("{} <problem.json> <pose.json>", args[0]);
    std::process::exit(1);
  }
  let prob: Input = serde_json::from_reader(File::open(&args[1])?)?;
  let pose: Output = serde_json::from_reader(File::open(&args[2])?)?;

  let hole_polygon = paths::polygon(&prob.hole);

  let figure_path = paths::path(&prob.figure.edges, &pose.vertices);

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
