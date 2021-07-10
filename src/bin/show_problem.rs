use icfpc2021::paths::*;
use icfpc2021::*;

// Usage: show_problem [problem.json]
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let prob = if args.len() < 2 {
        read_input()
    } else {
        read_input_from_file(&args[1])
    };

    render_problem_svg(&prob, std::io::stdout())
}
