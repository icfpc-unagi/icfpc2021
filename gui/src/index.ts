import * as PIXI from "pixi.js";
import { Container, Graphics } from "pixi.js";
import { DragHandler } from "./dragdrop";

PIXI.settings.RESOLUTION = window.devicePixelRatio || 1;
const app = new PIXI.Application({
  width: 800,
  height: 600,
  autoDensity: true,
});
document.body.appendChild(app.view);

const mainContainer = new Container();
const guiScale = 4;
mainContainer.scale.set(guiScale); // wip
app.stage.addChild(mainContainer);

type Input = {
  hole: [number, number][];
  epsilon: number;
  figure: {
    vertices: [number, number][];
    edges: [number, number][];
  };
};

// 1.json
// prettier-ignore
const inputJson: Input = {"hole":[[45,80],[35,95],[5,95],[35,50],[5,5],[35,5],[95,95],[65,95],[55,80]],"epsilon":150000,"figure":{"edges":[[2,5],[5,4],[4,1],[1,0],[0,8],[8,3],[3,7],[7,11],[11,13],[13,12],[12,18],[18,19],[19,14],[14,15],[15,17],[17,16],[16,10],[10,6],[6,2],[8,12],[7,9],[9,3],[8,9],[9,12],[13,9],[9,11],[4,8],[12,14],[5,10],[10,15]],"vertices":[[20,30],[20,40],[30,95],[40,15],[40,35],[40,65],[40,95],[45,5],[45,25],[50,15],[50,70],[55,5],[55,25],[60,15],[60,35],[60,65],[60,95],[70,95],[80,30],[80,40]]}};

function drawProblem(inputJson: Input) {
  mainContainer.removeChildren();
  const hole = new Graphics().beginFill(0xffffff).moveTo(...inputJson.hole[0]);
  for (const [x, y] of inputJson.hole.slice(1)) {
    hole.lineTo(x, y);
  }
  hole.closePath();
  mainContainer.addChild(hole);

  const fig = inputJson.figure;
  for (const [i, j] of fig.edges) {
    const g = new Graphics().lineStyle({
      color: 0x0000ff,
      width: 3 / guiScale,
      cap: "round" as any,
    });
    g.moveTo(...fig.vertices[i]).lineTo(...fig.vertices[j]);
    mainContainer.addChild(g);
  }

  for (const [x, y] of fig.vertices) {
    const g = new Graphics().beginFill(0x008000).drawCircle(0, 0, 6 / guiScale);
    g.position.set(x, y);
    g.interactive = true;
    g.on("pointerdown", () => {
      // todo
    });
    mainContainer.addChild(g);
  }
}

drawProblem(inputJson);

const fileElem: any = document.getElementById("input-json")!;
fileElem.addEventListener("change", () => {
  const file = fileElem.files[0];
  if (file == null) return;
  const reader = new FileReader();
  reader.readAsText(file, "UTF-8");
  reader.onload = (e) => {
    const inputJson: Input = JSON.parse(e.target!.result as string);
    drawProblem(inputJson);
  };
});
