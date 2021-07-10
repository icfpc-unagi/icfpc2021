use icfpc2021::{*, util::*};


fn main() {
    println!("digraph G {{");
    println!("  rankdir=\"LR\";");

    for entry in glob::glob("./problems/*.json").unwrap() {
        let path = entry.unwrap();

        let filename = path.to_str().unwrap()
            .split("/")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .to_owned();
        let problem_id: i64 = filename.split('.').collect::<Vec<_>>()[0].parse().unwrap();

        let input = read_input_from_file(path);

        for bonus in &input.bonuses {
            println!("  {} -> {} [ label=\"{:?}\" ];", problem_id, bonus.problem, bonus.bonus)
        }
    }

    println!("}}");
}