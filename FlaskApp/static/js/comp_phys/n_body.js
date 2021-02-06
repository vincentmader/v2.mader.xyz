import { draw_line, draw_point } from "../utils/drawing_utils.js";

const tail_length = 300;
const line_width = 3;

var canvas, ctx, frame_idx;
var W, H, o_x, o_y, zoom_level;
var data = $("#canvas").data("ys");
var system_state;
var nr_of_bodies;

function draw_tails(ctx, nr_of_bodies, frame_idx, tail_length) {
  var current_system_state, previous_system_state;
  var coords_p, coords_c, x_p, y_p, x_c, y_c;
  for (const idx of Array(tail_length).keys()) {
    // get current & previous system state
    current_system_state = data[Math.max(0, frame_idx - idx)];
    previous_system_state = data[Math.max(0, frame_idx - idx - 1)];
    for (let idx = 0; idx < nr_of_bodies; idx++) {
      // get information about body in previous & current timestep
      x_p = previous_system_state[6 * idx + 2];
      y_p = previous_system_state[6 * idx + 3];
      x_c = current_system_state[6 * idx + 2];
      y_c = current_system_state[6 * idx + 3];
      // convert to canvas coords (code units -> px)
      coords_p = xy_to_canvas_coords(x_p, y_p, W, H, zoom_level);
      coords_c = xy_to_canvas_coords(x_c, y_c, W, H, zoom_level);
      x_p = coords_p[0];
      y_p = coords_p[1];
      x_c = coords_c[0];
      y_c = coords_c[1];
      // draw line between previous & current position
      draw_line(ctx, x_p, y_p, x_c, y_c, "white");
    }
  }
}

function xy_to_canvas_coords(x, y, W, H, zoom_level) {
  return [x / zoom_level + o_x, y / zoom_level + o_y];
}

function draw_bodies(system_state, nr_of_bodies) {
  var m, R, x, y, u, v;
  var coords;
  for (let idx = 0; idx < nr_of_bodies; idx++) {
    // get information about body
    m = system_state[6 * idx];
    R = system_state[6 * idx + 1];
    x = system_state[6 * idx + 2];
    y = system_state[6 * idx + 3];
    u = system_state[6 * idx + 4];
    v = system_state[6 * idx + 5];
    // convert to canvas coords (code units -> px)
    coords = xy_to_canvas_coords(x, y, W, H, zoom_level);
    x = coords[0];
    y = coords[1];
    // draw body
    draw_point(ctx, x, y, 3); // TODO: make radius variable
  }
}

function init() {
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;
  // define coordinate origin on screen
  o_x = W / 2;
  o_y = H / 2;
  zoom_level = 2 / W;

  frame_idx = 0;
  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // get data about n body system
    system_state = data[frame_idx];
    nr_of_bodies = system_state.length / 6;
    // draw
    draw_bodies(system_state, nr_of_bodies);
    draw_tails(ctx, nr_of_bodies, frame_idx, tail_length);
    frame_idx += 1;
    // event listeners
    document.getElementById("restart").addEventListener("click", function () {
      frame_idx = 0;
      // zoom_level = W / 2;
    });
    document.getElementById("zoom_in").addEventListener("click", function () {
      zoom_level += 0.1;
    });
    document.getElementById("zoom_out").addEventListener("click", function () {
      zoom_level -= 0.1;
    });
  }, 20);
}

init();
