import glob
import os
import subprocess
import logging
import json
import multiprocessing
import multiprocessing.pool
import functools
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


def doit(problem_id, glucose_path):
    work_dir = f"rsap/{problem_id}/"

    subprocess.run(
        f'curl "https://icfpc.sx9.jp/best_solution?problem_id={problem_id}" > "best_solutions/{problem_id}.json"',
        shell=True,
        check=True,
    )

    old_score = evaluate(problem_id, f"best_solutions/{problem_id}.json")
    if old_score == 0:
        return f"{problem_id}: SKIP"

    result = subprocess.run(
        f'cargo run --release --bin sat_hillclimber -- '
        f'--glucose-path {glucose_path} '
        f'--input-path "problems/{problem_id}.json" '
        f'--output-path "best_solutions/{problem_id}.json" '
        f'--work-dir {work_dir} '
        f'--max-neighbor 11 ',
        shell=True,
    )

    if result.returncode != 0:
        return f"{problem_id}: FAIL"
    else:
        solution_path = f"{work_dir}/sol999999.json"
        old_score = evaluate(problem_id, f"best_solutions/{problem_id}.json")
        new_score = evaluate(problem_id, solution_path)

        subprocess.run(
            ["curl", "-X", "POST", "-d", f"@{solution_path}",
             f"https://icfpc.sx9.jp/api/submit?problem_id={problem_id}"],
            check=True)

        return f"{problem_id}: {old_score} -> {new_score}"


def main(
        problem=None,
        glucose_path="/home/takiba/Desktop/glucose-syrup-4.1/simp/glucose",
        n_threads=8,
):
    logging.basicConfig(filename='run_sat_all.log', level=logging.DEBUG)
    logger.info(f"\n{'=' * 80}\nSTART!\n{'=' * 80}")

    if problem:
        problem_ids = [problem]
    else:
        paths = list(glob.glob("./problems/*.json"))
        problem_ids = list(map(problem_id_from_path, paths))
        problem_ids.sort()

    # problem_ids = [p for p in problem_ids if p >= 100]

    tpool = multiprocessing.pool.ThreadPool(n_threads)
    results = tpool.imap(
        functools.partial(doit, glucose_path=glucose_path),
        problem_ids,
        chunksize=1
    )
    for result in results:
        logger.info(result)


if __name__ == '__main__':
    import fire
    fire.Fire(main)