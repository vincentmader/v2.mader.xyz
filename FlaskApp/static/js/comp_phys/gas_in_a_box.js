// import { draw_line } from "./a.js";
import { draw_point } from "../utils/drawing_utils.js";

const INITIAL_ZOOM_LEVEL = 70;

var canvas, ctx;
var W, H, o_x, o_y;

var data = $("#gas_in_a_box_canvas").data("ys");
var system_state;
var frame_idx;

var zoom_level = INITIAL_ZOOM_LEVEL;
var foo;

// const line_width = 2;
// // const tail_length = 150;
// // var circle_radius;

// // function draw_tails(ctx, frame_idx, tail_length) {
// //   var current_system_state, previous_system_state;

// //   for (const idx of Array(tail_length).keys()) {
// //     current_system_state = data[Math.max(0, frame_idx - idx)];
// //     previous_system_state = data[Math.max(0, frame_idx - idx - 1)];

// //     phi_1p = previous_system_state[0];
// //     phi_2p = previous_system_state[1];
// //     foo = get_positions_from_angles(phi_1p, phi_2p);
// //     x_1p = foo[0];
// //     y_1p = foo[1];
// // x_2p = foo[2];
// // y_2p = foo[3];

// // phi_1c = current_system_state[0];
// // phi_2c = current_system_state[1];
// // foo = get_positions_from_angles(phi_1c, phi_2c);
// // x_1c = foo[0];
// // y_1c = foo[1];
// // x_2c = foo[2];
// // y_2c = foo[3];

// // draw_line(ctx, x_1p, y_1p, x_1c, y_1c, "green");
// // draw_line(ctx, x_2p, y_2p, x_2c, y_2c, "red");
// // }
// // }

function xy_to_canvas_coords(x, y, W, H, zoom_level) {
  // var xlim;
  // var ylim;

  // var canvas_x;
  // var canvas_y;

  x *= W;
  y *= H;
  // x += o_x;
  // y += o_y;

  return [x, y];
}

function init() {
  canvas = document.getElementById("gas_in_a_box_canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");
  o_x = W / 2;
  o_y = H / 2;

  // ctx.lineWidth = line_width;

  frame_idx = 0;
  setInterval(function () {
    //   // draw_tails(ctx, frame_idx);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    system_state = data[frame_idx];
    var nr_of_bodies = system_state.length / 8;
    var m, R, x, y, u, v;
    for (let idx = 0; idx < nr_of_bodies; idx++) {
      m = system_state[6 * idx];
      R = system_state[6 * idx + 1];
      x = system_state[6 * idx + 2];
      y = system_state[6 * idx + 3];
      u = system_state[6 * idx + 4];
      v = system_state[6 * idx + 5];
      foo = xy_to_canvas_coords(x, y, W, H, zoom_level);
      x = foo[0];
      y = foo[1];
      draw_point(ctx, x, y, 1);
    }

    frame_idx += 1;
    document.getElementById("restart").addEventListener("click", function () {
      frame_idx = 0;
      zoom_level = INITIAL_ZOOM_LEVEL;
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
