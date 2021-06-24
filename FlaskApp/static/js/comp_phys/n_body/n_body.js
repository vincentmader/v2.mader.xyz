import { draw_line, draw_point } from "../../utils/drawing_utils.js";
import { draw_tails } from "./drawing_utils.js";

const tail_length = 100;
const line_width = 3;

// World
var nr_of_bodies;
var system_state;
var o_x, o_y;
// Drawing
var W, H;
var zoom_level;
var drawing_radius = 16;
// Statistics
var frame_idx = 0;
// Settings
var paused = false;

// convert between canvas & world coords
function xy_to_canvas_coords(x, y, W, H, zoom_level) {
  return [x / zoom_level + o_x, -y / zoom_level + o_y];
}

// draw the 3 bodies
function draw_bodies(ctx, system_state, nr_of_bodies) {
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
    draw_point(ctx, x, y, drawing_radius); // TODO: make radius variable
  }
}

// setup events for button menu
function setup_event_listeners(canvas_id) {
  // event listeners
  document
    .getElementById("restart_" + String(canvas_id))
    .addEventListener("click", function () {
      frame_idx = 0;
      zoom_level = W / 2; // TODO: implement zoom
    });
  document
    .getElementById("play/pause_" + String(canvas_id))
    .addEventListener("click", function () {
      paused = !paused;
      let button_toggle_pause = document.getElementById(
        "play/pause_" + String(canvas_id)
      );
      if (!paused) {
        button_toggle_pause.innerHTML = "pause";
      } else {
        button_toggle_pause.innerHTML = "unpause";
      }
    });
}

// main function
export function main(canvas, ctx, canvas_id, system_states) {
  // define canvas geometry
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx.lineWidth = line_width;
  // define coordinate origin on screen
  o_x = W / 2;
  o_y = H / 2;
  zoom_level = 2 / W;
  // setup event listeners
  setup_event_listeners(canvas_id);

  // animation loop
  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // get system_states array with info about n body system
    system_state = system_states[frame_idx];
    if (system_state == undefined) {
      frame_idx = 0;
      system_state = system_states[frame_idx];
      paused = true;
    }
    nr_of_bodies = system_state.length / 6;
    // draw
    draw_bodies(ctx, system_state, nr_of_bodies);
    draw_tails(ctx, system_states, frame_idx, tail_length, W, H, zoom_level);
    if (!paused) {
      frame_idx += 1;
    }

    // TODO: do below only for figure-8 simulation
    if (frame_idx == 844) {
      // TODO: find out optimal index (as close to 1 period as possible)
      frame_idx = 422;
    }
  }, 4); // TODO: influence step size?
}
