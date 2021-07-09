export { DragHandler };
import * as PIXI from "pixi.js";

class DragHandler {
  pointerMap: Map<
    number,
    { target: PIXI.DisplayObject; data: PIXI.InteractionData; offset: any }
  >;
  dropMap: Map<PIXI.DisplayObject, PIXI.DisplayObject>;

  constructor(
    private im: PIXI.InteractionManager,
    private dropArea: PIXI.DisplayObject
  ) {
    this.onDown = this.onDown.bind(this);
    this.onUp = this.onUp.bind(this);
    this.onMove = this.onMove.bind(this);
    this.pointerMap = new Map();
    this.dropMap = new Map();
  }

  register(c: PIXI.DisplayObject) {
    c.interactive = true;
    c.on("pointerdown", this.onDown)
      .on("pointerup", this.onUp)
      .on("pointerupoutside", this.onUp)
      .on("pointercancel", () => {
        console.log("ignored pointercancel event");
      })
      .on("pointermove", this.onMove);
  }

  hitTest(event: PIXI.InteractionEvent): PIXI.DisplayObject {
    // may be undefined (caused by hitting nothing)
    return this.im.hitTest(event.data.global, this.dropArea);
  }

  onDown(event: PIXI.InteractionEvent): void {
    // console.log("drag start", event);
    if (event.target !== event.currentTarget) {
      console.warn(event.target, event.currentTarget);
    }

    const data = event.data;
    const id = data.identifier;
    if (this.pointerMap.has(id)) {
      console.error("non-unique pointer ID", id);
    }

    // for simplicity, ignore multitouch on the same target
    const target = event.currentTarget;
    if (this.dropMap.has(target)) {
      return;
    }

    // console.log(event.target == this.target);
    // console.group();
    // for (const [k, v] of Object.entries(event)) {
    //   console.log(k, v);
    // }
    // console.groupEnd();
    // store a reference to the data
    // the reason for this is because of multitouch
    // we want to track the movement of this particular touch
    const base = data.getLocalPosition(target.parent);
    const offset = { x: target.x - base.x, y: target.y - base.y };
    this.pointerMap.set(id, { target, data, offset });
    const drop = this.hitTest(event);
    this.dropMap.set(target, drop);
    target.emit("dragstart", { drag: target, drop });
    // console.log("dragstart", target.name, drop?.name);
    if (drop != null) {
      drop.emit("dragenter", { drag: target, drop });
    }
  }

  onMove(event: PIXI.InteractionEvent): void {
    const value = this.pointerMap.get(event.data.identifier);
    if (value == null) {
      return;
    }
    const { target, data, offset } = value;
    if (event.currentTarget !== target) {
      return;
    }
    // console.log("drag move", event.data.getLocalPosition(target.parent));
    const newPosition = data.getLocalPosition(target.parent);
    target.position.set(newPosition.x + offset.x, newPosition.y + offset.y);

    const oldDrop = this.dropMap.get(target);
    const newDrop = this.hitTest(event);
    if (oldDrop !== newDrop) {
      if (oldDrop != null) {
        oldDrop.emit("dragleave", { drag: target, drop: oldDrop });
      }
      if (newDrop != null) {
        newDrop.emit("dragenter", { drag: target, drop: newDrop });
      }
    }
    target.emit("drag", { drag: target, drop: newDrop });
    this.dropMap.set(target, newDrop);
  }

  onUp(event: PIXI.InteractionEvent): void {
    const id = event.data.identifier;
    const value = this.pointerMap.get(id);
    if (value == null) {
      return;
    }
    const { target } = value;
    if (event.currentTarget !== target) {
      return;
    }

    this.onMove(event);
    // const tmp: any = event; console.log("drag end", tmp.target?._texture.textureCacheIds[0], tmp.currentTarget._texture.textureCacheIds[0]);
    const drop = this.dropMap.get(target);
    if (drop != null) {
      drop.emit("drop", { drag: target, drop });
    }
    target.emit("dragend", { drag: target, drop });
    this.dropMap.delete(target);
    this.pointerMap.delete(id);
  }
}
