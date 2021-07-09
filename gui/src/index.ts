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
