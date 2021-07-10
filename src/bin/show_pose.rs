use icfpc2021::paths::*;
use std::fs::File;

// Usage: show_pose problem.json pose.json
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("{} <problem.json> <pose.json>", args[0]);
        std::process::exit(1);
    }
    let prob = serde_json::from_reader(File::open(&args[1])?)?;
    let pose = serde_json::from_reader(File::open(&args[2])?)?;

    render_pose_svg(&prob, &pose, std::io::stdout())
}
