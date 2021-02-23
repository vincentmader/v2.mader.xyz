import { draw_point, draw_line } from "../utils/drawing_utils.js";
import { get_L2_norm } from "../utils/math_utils.js";

const zoom_level = 0.9;

var canvas, ctx;
var W, H;
var paused = false;

var x, y, r, canvas_coords, color;
var dart_hits, dart_throws, pi;

// convert (x,y) \in [0,1]^2 -> (x,y) \in [-W,W]x[-H,H]
const get_canvas_coords = (x, y) => {
  const canvas_x = (W / 2) * (1 + x * zoom_level);
  const canvas_y = (H / 2) * (1 + y * zoom_level);
  return [canvas_x, canvas_y];
};

// draw big circle (radius r=1) & big square (side length a=2)
const draw_boxes = () => {
  ctx.strokeStyle = "#222222";
  ctx.lineWidth = 3;
  // draw big circle
  ctx.beginPath();
  ctx.arc(W / 2, H / 2, (W / 2) * zoom_level, 0, 2 * Math.PI);
  ctx.stroke();
  // draw big box
  ctx.beginPath();
  canvas_coords = get_canvas_coords(-1, +1);
  ctx.moveTo(canvas_coords[0], canvas_coords[1]);
  canvas_coords = get_canvas_coords(+1, +1);
  ctx.lineTo(canvas_coords[0], canvas_coords[1]);
  ctx.stroke();
  canvas_coords = get_canvas_coords(+1, -1);
  ctx.lineTo(canvas_coords[0], canvas_coords[1]);
  ctx.stroke();
  canvas_coords = get_canvas_coords(-1, -1);
  ctx.lineTo(canvas_coords[0], canvas_coords[1]);
  ctx.stroke();
  canvas_coords = get_canvas_coords(-1, +1);
  ctx.lineTo(canvas_coords[0], canvas_coords[1]);
  ctx.stroke();
};

// handle click of play/pause button
const handle_button_click_playpause = () => {
  paused = !paused;
  if (paused) {
    document.getElementById("play/pause").innerHTML = "Unpause";
  } else {
    document.getElementById("play/pause").innerHTML = "Pause";
  }
};

// handle click of reset button
const handle_button_click_reset = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  draw_boxes();
  dart_throws = 0;
  dart_hits = 0;
};

// initialize
const init = () => {
  // setup canvas
  canvas = document.getElementById("canvas");
  ctx = canvas.getContext("2d");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;

  // run handle method for reset button to initialize canvas
  handle_button_click_reset();
  // setup buttons & event listeners
  document
    .getElementById("play/pause")
    .addEventListener("click", handle_button_click_playpause);
  document
    .getElementById("reset")
    .addEventListener("click", handle_button_click_reset);

  // run infinite loop
  setInterval(function () {
    if (paused) return;
    // throw dart, i.e. choose random point from interval [0,1]^2
    x = 2 * Math.random() - 1;
    y = 2 * Math.random() - 1;
    dart_throws += 1;
    // check for dart hit (does random point lie inside the circle)
    r = get_L2_norm([x, y]);
    if (r <= 1) {
      dart_hits += 1;
      color = "green"; // use green as color when dart hits target
    } else {
      color = "white"; // else use white
    }
    // draw point
    canvas_coords = get_canvas_coords(x, y);
    draw_point(ctx, canvas_coords[0], canvas_coords[1], 1, color, color);
    // calculate pi
    pi = ((dart_hits / dart_throws) * 4).toFixed(2);
    document.getElementById("textfield_pi").innerHTML = "Pi ~= " + pi;
  }, 1);
};

init();
