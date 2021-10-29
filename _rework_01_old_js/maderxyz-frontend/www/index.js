import * as wasm from "maderxyz-frontend";

import { Universe } from "maderxyz-frontend";

const universe = Universe.new();

var frame_idx;
var canvas, ctx;

const init = () => {
  frame_idx = 0;
  canvas = document.getElementById("canvas");
  canvas.width = canvas.height;
  ctx = canvas.getContext("2d");
  start_render_loop();
};

const start_render_loop = () => {
  if (frame_idx % 100 == 0) {
    console.log(universe.name());
  }
  universe.tick();
  requestAnimationFrame(start_render_loop);
  frame_idx += 1;
};

init();
