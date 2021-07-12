import subprocess
import json


def main(set_path):
    for (problem_id, line) in enumerate(open(set_path).readlines()):
        if line.strip() == "":
            continue

        problem_id = problem_id + 1
        submission_id, globalist_source_problem = line.split(' ')
        globalist_source_problem = int(globalist_source_problem)
        print(submission_id, globalist_source_problem)

        subprocess.run(
            f'curl "https://icfpc.sx9.jp/submission?submission_id={submission_id}" '
            f'> "tmp.txt"',
            shell=True,
            check=True,
        )
        j = json.load(open("tmp.txt"))

        if globalist_source_problem != 0:
            j["bonuses"][0]["problem"] = globalist_source_problem

        with open(f"{problem_id}.json", "w") as f:
            json.dump(j, f)


if __name__ == '__main__':
    import fire
    fire.Fire(main)
