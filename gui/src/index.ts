import * as PIXI from "pixi.js";
import { Graphics } from "pixi.js";

PIXI.settings.RESOLUTION = window.devicePixelRatio || 1;
const app = new PIXI.Application({
  width: 800,
  height: 600,
  autoDensity: true,
});
const c = new Graphics().beginFill(0xaaaaaa).drawRect(0, 0, 800, 600);
app.stage.addChild(c);
const g = new Graphics()
  .lineStyle({ width: 3 })
  .moveTo(100, 100)
  .lineTo(200, 300);
app.stage.addChild(g);
document.body.appendChild(app.view);

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
let inputJson: Input = {"hole":[[45,80],[35,95],[5,95],[35,50],[5,5],[35,5],[95,95],[65,95],[55,80]],"epsilon":150000,"figure":{"edges":[[2,5],[5,4],[4,1],[1,0],[0,8],[8,3],[3,7],[7,11],[11,13],[13,12],[12,18],[18,19],[19,14],[14,15],[15,17],[17,16],[16,10],[10,6],[6,2],[8,12],[7,9],[9,3],[8,9],[9,12],[13,9],[9,11],[4,8],[12,14],[5,10],[10,15]],"vertices":[[20,30],[20,40],[30,95],[40,15],[40,35],[40,65],[40,95],[45,5],[45,25],[50,15],[50,70],[55,5],[55,25],[60,15],[60,35],[60,65],[60,95],[70,95],[80,30],[80,40]]}};

let hole = new Graphics().beginFill(0xffffff).moveTo(...inputJson.hole[0]);
for (const [x, y] of inputJson.hole.slice(1)) {
  hole.lineTo(x, y);
}
hole.closePath();
app.stage.addChild(hole);
