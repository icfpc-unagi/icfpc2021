import json
import os
import pathlib
import subprocess

import numpy as np
import pydot

os.chdir(pathlib.Path(__file__).parent.parent / "tmp")

for k in range(1, 78+1):
    proc = subprocess.run(["neato", f"{k}.dot"], capture_output=True, text=True)
    graph, = pydot.graph_from_dot_data(proc.stdout)
    positions = {
        int(n.get_name()): [
            np.around(float(p) / 96, decimals=2)
            for p in pos.strip('"').split(",")
        ]
        for n in graph.get_nodes()
        if (pos := n.get_pos())
    }
    positions = [positions[i] for i in range(len(positions))]
    with open(f"{k}-dot.json", "w") as f:
        f.write(json.dumps({"vertices": positions}, separators=(',', ':')))
# breakpoint()