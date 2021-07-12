import glob
import os
import subprocess
import logging
import json
logger = logging.getLogger(__name__)


def problem_id_from_path(path):
    return int(os.path.basename(path).split('.')[0])


def evaluate(problem_id, solution_path):
    return json.loads(subprocess.run(
        f"cargo run --release --bin evaluate -- problems/{problem_id}.json {solution_path}",
        shell=True,
        check=True,
        capture_output=True,
    ).stdout)["dislikes"]



def main(
        problem=None,
        glucose_path="/home/takiba/Desktop/glucose-syrup-4.1/simp/glucose",
):
    logging.basicConfig(filename='run_sat_all.log', level=logging.DEBUG)
    logger.info(f"\n{'=' * 80}\nSTART!\n{'=' * 80}")

    paths = list(glob.glob("./problems/*.json"))
    paths.sort(key=problem_id_from_path)

    for path in paths:
        problem_id = problem_id_from_path(path)
        if problem and problem_id != problem:
            continue

        if problem_id < 97:
            continue

        subprocess.run(
            f'curl "https://icfpc.sx9.jp/best_solution?problem_id={problem_id}" > "best_solutions/{problem_id}.json"',
            shell=True,
            check=True,
        )

        old_score = evaluate(problem_id, f"best_solutions/{problem_id}.json")
        if old_score == 0:
            logger.info(f"{problem_id}: SKIP")
            continue

        result = subprocess.run(
            f'cargo run --release --bin sat_hillclimber -- --glucose-path {glucose_path} --input-path "problems/{problem_id}.json" --output-path "best_solutions/{problem_id}.json"  --max-neighbor 25',
            shell=True,
        )

        if result.returncode != 0:
            logger.info(f"{problem_id}: FAIL")
        else:
            solution_path = "./out/sol999999.json"
            old_score = evaluate(problem_id, f"best_solutions/{problem_id}.json")
            new_score = evaluate(problem_id, solution_path)
            logger.info(f"{problem_id}: {old_score} -> {new_score}")

            subprocess.run(
                ["curl", "-X", "POST", "-d", f"@{solution_path}", f"https://icfpc.sx9.jp/api/submit?problem_id={problem_id}"],
                check=True)


if __name__ == '__main__':
    import fire
    fire.Fire(main)