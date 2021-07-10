use std::io::BufRead;

#[derive(Debug)]
pub struct TreeDecomposition {
    bag_vs: Vec<Vec<usize>>,
    es: Vec<(usize, usize)>,
}


pub fn read_tree_decomposition(path: impl AsRef<std::path::Path>) -> TreeDecomposition {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut td = TreeDecomposition {
        bag_vs: vec![],
        es: vec![]
    };

    for line in reader.lines() {
        let line = line.unwrap();

        let tokens = line.split(' ').collect::<Vec<_>>();

        match tokens[0] {
            "s" => {
                assert_eq!(tokens[1], "td");
                let n_bags: usize = tokens[2].parse().unwrap();
                td.bag_vs.resize(n_bags, vec![]);
            }
            "b" => {
                let bag_id = tokens[1].parse::<usize>().unwrap() - 1;
                for t in &tokens[2..] {
                    let v = t.parse::<usize>().unwrap() - 1;
                    td.bag_vs[bag_id].push(v);
                }
            }
            _ => {
                let bag_id1 = tokens[0].parse::<usize>().unwrap() - 1;
                let bag_id2 = tokens[1].parse::<usize>().unwrap() - 1;
                td.es.push((bag_id1, bag_id2));
            }
        }
    }

    td
}
