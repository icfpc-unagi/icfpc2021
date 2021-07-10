import * as PIXI from "pixi.js";
import { Container, Graphics } from "pixi.js";
import { DragHandler } from "./dragdrop";

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
  width: 800,
  height: 600,
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
};

type Solution = {
  vertices: XY[];
};

// 1.json
// prettier-ignore
const sampleInput: Problem = {"hole":[[45,80],[35,95],[5,95],[35,50],[5,5],[35,5],[95,95],[65,95],[55,80]],"epsilon":150000,"figure":{"edges":[[2,5],[5,4],[4,1],[1,0],[0,8],[8,3],[3,7],[7,11],[11,13],[13,12],[12,18],[18,19],[19,14],[14,15],[15,17],[17,16],[16,10],[10,6],[6,2],[8,12],[7,9],[9,3],[8,9],[9,12],[13,9],[9,11],[4,8],[12,14],[5,10],[10,15]],"vertices":[[20,30],[20,40],[30,95],[40,15],[40,35],[40,65],[40,95],[45,5],[45,25],[50,15],[50,70],[55,5],[55,25],[60,15],[60,35],[60,65],[60,95],[70,95],[80,30],[80,40]]}};

// solution in spec-v1.0.pdf
// prettier-ignore
const sampleOutput: Solution = {"vertices": [[21, 28], [31, 28], [31, 87], [29, 41], [44, 43], [58, 70],[38, 79], [32, 31], [36, 50], [39, 40], [66, 77], [42, 29],[46, 49], [49, 38], [39, 57], [69, 66], [41, 70], [39, 60],[42, 25], [40, 35]]};

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
        color,
        width: 3 / guiScale,
        cap: "round" as any,
      })
      .moveTo(...p0)
      .lineTo(...p1);
  }
}

class ProblemRenderer {
  hole: Graphics;
  edges: EdgeObject[];
  vertices: Graphics[];
  epsilon: number;

  constructor(public inputJson: Problem) {
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

    const fig = inputJson.figure;

    const vertices: Graphics[] = [];
    for (const [x, y] of fig.vertices) {
      const g = new Graphics()
        .beginFill(0x008000)
        .drawCircle(0, 0, 6 / guiScale);
      g.position.set(x, y);
      dragHandler.register(g);
      vertices.push(g);
    }

    this.epsilon = inputJson.epsilon;
    const edges = [];
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
      v.on("drag", () => v.emit("myupdate")).on("dragend", () => {
        const { x, y } = v.position;
        v.position.set(Math.round(x), Math.round(y));
        v.emit("myupdate");
        (document.getElementById("output-json") as any).value = JSON.stringify(
          this.saveSolution()
        );
      });
    }
  }

  loadSolution(solutionJson: Solution): void {
    if (this.vertices.length != solutionJson.vertices.length) {
      alert("vertices.length differs");
      return;
    }
    for (const [i, v] of solutionJson.vertices.entries()) {
      this.vertices[i].position.set(...v);
    }
    for (const edge of this.edges) {
      edge.update();
    }
  }

  saveSolution(): Solution {
    return { vertices: this.vertices.map((v) => xyFromPoint(v)) };
  }

  render(c: Container): void {
    c.removeChildren();
    c.addChild(this.hole, ...this.edges.map(({ g }) => g), ...this.vertices);
  }
}

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
      const inputJson = JSON.parse(e.target!.result as string);
      r = new ProblemRenderer(inputJson);
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
      const solutionJson = JSON.parse(e.target!.result as string);
      r.loadSolution(solutionJson);
    };
  });
}

// gui scale
{
  const elem: any = document.getElementById("gui-scale")!;
  elem.addEventListener("change", () => {
    guiScale = parseInt(elem.value);
    mainContainer.scale.set(guiScale);
  });
}
