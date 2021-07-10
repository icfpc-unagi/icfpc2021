use icfpc2021::{paths::*, read_input_from_file, read_output_from_file};

// Usage: show_pose problem.json pose.json
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("{} <problem.json> <pose.json>", args[0]);
        std::process::exit(1);
    };
    let prob = read_input_from_file(&args[1]);
    let pose = read_output_from_file(&args[2]);

    render_pose_svg(&prob, &pose, std::io::stdout())
}
