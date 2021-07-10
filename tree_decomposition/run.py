import glob
import json
import os
import subprocess


def generate_dimacs_graph(figure):
    n_vs = len(figure["vertices"])
    es = figure["edges"]

    lines = []
    lines.append(f"p tw {n_vs} {len(es)}")
    lines.extend(
        f"{e[0] + 1} {e[1] + 1}"
        for e in es
    )
    return "\n".join(lines)


def problem_id_from_path(path):
    return int(os.path.basename(path).split('.')[0])


def main(problems_dir="../problems", solver_dir="./PACE2017-TrackA", out_dir="./"):
    paths = list(glob.glob(problems_dir + "/*.json"))

    paths.sort(key=problem_id_from_path)

    for path in paths:
        j = json.load(open(path))

        resp = subprocess.run(["java", "-classpath", solver_dir, "tw.exact.MainDecomposer"], text=True, input=generate_dimacs_graph(j["figure"]), capture_output=True)

        with open(f"{out_dir}/{problem_id_from_path(path)}.txt", "w") as f:
            f.write(resp.stdout)

        lines = resp.stdout.split("\n")
        print(int(lines[0].split(' ')[3]) - 1)


if __name__ == '__main__':
    import fire
    fire.Fire(main)
