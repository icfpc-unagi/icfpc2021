import * as PIXI from "pixi.js";
import { Container, DisplayObject, Graphics } from "pixi.js";
import { DragHandler } from "./dragdrop";

// lazy import
import wasm_ from "icfpc2021";
let wasm: undefined | typeof wasm_;

const urlParams = new URL(document.location.href).searchParams;

const WHITE: number = 0xffffff;

type XY = [number, number];

function abs2([x0, y0]: XY, [x1, y1]: XY): number {
  const x = x0 - x1;
  const y = y0 - y1;
  return x * x + y * y;
}

function xyFromPoint({ x, y }: { x: number; y: number }): XY {
  return [x, y];
}

PIXI.settings.RESOLUTION = window.devicePixelRatio || 1;
const app = new PIXI.Application({
  width: parseInt(urlParams.get("w") ?? "800"),
  height: parseInt(urlParams.get("h") ?? "600"),
  backgroundColor: 0x999999,
  autoDensity: true,
});
document.body.appendChild(app.view);

const mainContainer = new Container();
let guiScale = 4;
mainContainer.scale.set(guiScale); // wip
app.stage.addChild(mainContainer);

type Problem = {
  hole: XY[];
  epsilon: number;
  figure: {
    vertices: XY[];
    edges: [number, number][];
  };
  internal?: {
    reversed_hole: boolean;
  }
};

type Solution = {
  vertices: XY[];
};

// 1.json
// prettier-ignore
const sampleInput: string = '{"hole":[[45,80],[35,95],[5,95],[35,50],[5,5],[35,5],[95,95],[65,95],[55,80]],"epsilon":150000,"figure":{"edges":[[2,5],[5,4],[4,1],[1,0],[0,8],[8,3],[3,7],[7,11],[11,13],[13,12],[12,18],[18,19],[19,14],[14,15],[15,17],[17,16],[16,10],[10,6],[6,2],[8,12],[7,9],[9,3],[8,9],[9,12],[13,9],[9,11],[4,8],[12,14],[5,10],[10,15]],"vertices":[[20,30],[20,40],[30,95],[40,15],[40,35],[40,65],[40,95],[45,5],[45,25],[50,15],[50,70],[55,5],[55,25],[60,15],[60,35],[60,65],[60,95],[70,95],[80,30],[80,40]]}}';
// sampleInput.hole.reverse();

// prettier-ignore
const sampleOutput: string = '{"vertices":[[35,51],[40,60],[83,93],[34,25],[48,40],[59,70],[73,92],[29,15],[44,29],[40,18],[49,76],[36,7],[34,27],[30,17],[32,38],[40,69],[27,94],[17,93],[11,13],[18,21]]}';
// const sampleOutput: Solution = {"vertices": [[21, 28], [31, 28], [31, 87], [29, 41], [44, 43], [58, 70],[38, 79], [32, 31], [36, 50], [39, 40], [66, 77], [42, 29],[46, 49], [49, 38], [39, 57], [69, 66], [41, 70], [39, 60],[42, 25], [40, 35]]};

class EdgeObject {
  g: Graphics;
  d2Orig: number;

  constructor(
    public vertex0: Graphics,
    public vertex1: Graphics,
    public epsilon: number
  ) {
    const p0 = xyFromPoint(this.vertex0.position);
    const p1 = xyFromPoint(this.vertex1.position);
    this.g = new Graphics();
    this.d2Orig = abs2(p0, p1);
    this.update();
  }

  update(): void {
    const d2Orig = this.d2Orig;
    const p0 = xyFromPoint(this.vertex0.position);
    const p1 = xyFromPoint(this.vertex1.position);
    const d2Now = abs2(p0, p1);
    const atol = d2Orig * this.epsilon;
    const target = 1_000_000 * d2Orig;
    const ok = Math.abs(1_000_000 * d2Now - target) <= atol;
    const color = ok ? 0x0000ff : d2Now < d2Orig ? 0xcccc00 : 0xff0000;
    this.g
      .clear()
      .lineStyle({
        color: WHITE,
        width: 1,
        cap: "round" as any,
      })
      .moveTo(...p0)
      .lineTo(...p1).tint = color;
  }
}

class ProblemRenderer {
  inputJson: Problem;
  hole: Graphics;
  holeCorners: DisplayObject[];
  edges: EdgeObject[];
  vertices: Graphics[];
  epsilon: number;

  constructor(problem: string) {
    console.log(problem);
    const inputJson: Problem = (wasm?.read_problem ?? JSON.parse)(problem);
    console.log(inputJson);
    this.inputJson = inputJson;
    const dropArea = new Container(); // unused...
    const dragHandler = new DragHandler(
      app.renderer.plugins.interaction,
      dropArea
    );

    const hole = new Graphics()
      .beginFill(0xffffff)
      .moveTo(...inputJson.hole[0]);
    for (const [x, y] of inputJson.hole.slice(1)) {
      hole.lineTo(x, y);
    }
    hole.closePath();

    const holeCorners = [];
    let origHole = inputJson.hole.slice();
    if (inputJson.internal?.reversed_hole) {
      origHole.reverse();
    }
    for (const [i, [x, y]] of origHole.entries()) {
      // TODO: maybe reversed
      const text = new PIXI.Text(`${i}`, {
        fontSize: 12,
        fill: 0xffffff,
        stroke: 0x000000,
        strokeThickness: 2,
        // align: "center",
      });
      text.anchor.set(0.5);
      text.position.set(x, y);
      text.scale.set(1 / guiScale);
      holeCorners.push(text);
    }
    this.holeCorners = holeCorners;

    const fig = inputJson.figure;

    const vertices: Graphics[] = [];
    for (const [x, y] of fig.vertices) {
      const g = new Graphics().beginFill(WHITE).drawCircle(0, 0, 6);
      // g.tint = 0x008000;
      g.position.set(x, y);
      g.scale.set(1 / guiScale);
      dragHandler.register(g);
      vertices.push(g);
    }

    // for (const [i, v] of vertices.entries()) {
    //   const text = new PIXI.Text(`${i}`, {
    //     fontSize: 12,
    //     fill: 0xffffff,
    //     stroke: 0x000000,
    //     strokeThickness: 2,
    //     // align: "center",
    //   });
    //   text.anchor.set(0.5);
    //   v.addChild(text);
    // }

    this.epsilon = inputJson.epsilon;
    const edges: EdgeObject[] = [];
    for (const [i, j] of fig.edges) {
      const edge = new EdgeObject(vertices[i], vertices[j], this.epsilon);
      vertices[i].on("myupdate", () => edge.update());
      vertices[j].on("myupdate", () => edge.update());
      edges.push(edge);
    }

    this.hole = hole;
    this.edges = edges;
    this.vertices = vertices;
    for (const [k, v] of vertices.entries()) {
      {
        const { x, y } = v.position;
        const atCorner = inputJson.hole.some((hole) => hole[0] === x && hole[1] === y);
        v.tint = atCorner ? 0x00ff00 : 0x008000;
      }
      v.on("drag", () => {
        let { x, y } = v.position;
        x = Math.round(x);
        y = Math.round(y);
        v.position.set(Math.round(x), Math.round(y));
        const atCorner = inputJson.hole.some((hole) => hole[0] === x && hole[1] === y);
        v.tint = atCorner ? 0x00ff00 : 0x008000;
        v.emit("myupdate");
      }).on("dragend", () => {
        const solutionJson = this.pose;
        this.runCheckSolution1(inputJson, solutionJson);
        (document.getElementById("output-json") as any).value = (
          wasm?.write_pose ?? JSON.stringify
        )(solutionJson);
      });
    }

    this.runCheckSolution1(inputJson, inputJson.figure);
  }

  runCheckSolution1(input: Problem, output: Solution): void {
    if (wasm == null) return;
    const [ok_v, ok_e] = wasm.check_solution1(input, output);
    for (const [i, ok] of ok_v.entries()) {
      if (!ok) {
        this.vertices[i].tint = 0x800080;
      }
    }
    for (const [i, ok] of ok_e.entries()) {
      if (!ok) {
        this.edges[i].g.tint = 0x800080;
      }
    }
  }

  loadSolution(pose: string): void {
    const solutionJson = (wasm?.read_pose ?? JSON.parse)(pose);
    if (this.vertices.length != solutionJson.vertices.length) {
      alert("vertices.length differs");
      return;
    }
    for (const [i, v] of solutionJson.vertices.entries()) {
      this.vertices[i].position.set(...v);
      {
        let v = this.vertices[i];
        const { x, y } = v.position;
        const atCorner = this.inputJson.hole.some((hole) => hole[0] === x && hole[1] === y);
        v.tint = atCorner ? 0x00ff00 : 0x008000;
      }
    }
    for (const edge of this.edges) {
      edge.update();
    }

    this.runCheckSolution1(this.inputJson, solutionJson);
  }

  get pose(): Solution {
    return { vertices: this.vertices.map((v) => xyFromPoint(v)) };
  }

  render(c: Container): void {
    c.removeChildren();
    c.addChild(this.hole, ...this.edges.map(({ g }) => g), ...this.holeCorners, ...this.vertices);
  }

  updateGuiScale(): void {
    for (const v of [...this.vertices, ...this.holeCorners]) {
      v.scale.set(1 / guiScale);
    }
  }
}

mainContainer.addChild(new PIXI.Text("loading wasm", { fill: "red" }));
// console.log(wasm);
// console.log(wasm.check_solution1);
// console.log(wasm.check_solution1(sampleInput, sampleOutput));

(wasm_ as any)
  .then((wasm__: any) => {
    wasm = wasm__;
  })
  .catch((e: any) => {
    console.log("failed to load wasm:", e);
  })
  .then(() => {
    let r = new ProblemRenderer(sampleInput);
    r.render(mainContainer);
    r.loadSolution(sampleOutput);

    // load problem
    {
      const fileElem: any = document.getElementById("input-json")!;
      fileElem.addEventListener("change", () => {
        const file = fileElem.files[0];
        if (file == null) return;
        const reader = new FileReader();
        reader.readAsText(file, "UTF-8");
        reader.onload = (e) => {
          const problem = e.target!.result as string;
          r = new ProblemRenderer(problem);
          r.render(mainContainer);
        };
      });
    }

    // load solution
    {
      const fileElem: any = document.getElementById("input-solution-json")!;
      fileElem.addEventListener("change", () => {
        const file = fileElem.files[0];
        if (file == null) return;
        const reader = new FileReader();
        reader.readAsText(file, "UTF-8");
        reader.onload = (e) => {
          const pose = e.target!.result as string;
          r.loadSolution(pose);
        };
      });
    }

    // gui scale
    {
      const elem: any = document.getElementById("gui-scale")!;
      elem.addEventListener("change", () => {
        guiScale = parseInt(elem.value);
        mainContainer.scale.set(guiScale);
        r.updateGuiScale();
      });
    }
  });
