import { draw_line } from "./a.js";
import { draw_point } from "./a.js";

const line_width = 2;
const tail_length = 150;

var circle_radius;

var canvas, ctx;
var W, H;
var o_x, o_y;
var L;

var data = $("#double_pendulum_canvas").data("ys");
var foo, x_1, y_1, x_2, y_2;
var phi_1, phi_2;
var phi_1p, phi_2p, phi_1c, phi_2c;
var x_1p, y_1p, x_2p, y_2p, x_1c, y_1c, x_2c, y_2c;

var frame_idx;

function get_positions_from_angles(phi_1, phi_2) {
  let x_1 = o_x + L * Math.sin(phi_1); // cart. coords of pendulum mass
  let y_1 = o_y + L * Math.cos(phi_1);
  let x_2 = x_1 + L * Math.sin(phi_2); // cart. coords of pendulum mass
  let y_2 = y_1 + L * Math.cos(phi_2);

  return [x_1, y_1, x_2, y_2];
}

function draw_tails(ctx, frame_idx, tail_length) {
  var current_system_state, previous_system_state;

  for (const idx of Array(tail_length).keys()) {
    current_system_state = data[Math.max(0, frame_idx - idx)];
    previous_system_state = data[Math.max(0, frame_idx - idx - 1)];

    phi_1p = previous_system_state[0];
    phi_2p = previous_system_state[1];
    foo = get_positions_from_angles(phi_1p, phi_2p);
    x_1p = foo[0];
    y_1p = foo[1];
    x_2p = foo[2];
    y_2p = foo[3];

    phi_1c = current_system_state[0];
    phi_2c = current_system_state[1];
    foo = get_positions_from_angles(phi_1c, phi_2c);
    x_1c = foo[0];
    y_1c = foo[1];
    x_2c = foo[2];
    y_2c = foo[3];

    draw_line(ctx, x_1p, y_1p, x_1c, y_1c, "green");
    draw_line(ctx, x_2p, y_2p, x_2c, y_2c, "red");
  }
}

function draw_double_pendulum(ctx, frame_idx) {
  const system_state = data[frame_idx];

  phi_1 = system_state[0];
  phi_2 = system_state[1];

  foo = get_positions_from_angles(phi_1, phi_2);
  x_1 = foo[0];
  y_1 = foo[1];
  x_2 = foo[2];
  y_2 = foo[3];

  draw_tails(ctx, frame_idx, tail_length);
  draw_point(ctx, o_x, o_y, circle_radius);
  draw_point(ctx, x_1, y_1, circle_radius);
  draw_point(ctx, x_2, y_2, circle_radius);
  draw_line(ctx, o_x, o_y, x_1, y_1, "white");
  draw_line(ctx, x_1, y_1, x_2, y_2, "white");
}

const init = () => {
  canvas = document.getElementById("double_pendulum_canvas");
  // W = canvas.parentElement.clientWidth;
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;

  const ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;

  // set pendulum rod length L, & coordinate origin o_x & o_y
  L = W / 4 - 10;
  o_x = W / 2;
  o_y = H / 2;
  circle_radius = W / 100;

  // start loop, draw double pendulum
  frame_idx = 0;
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw_double_pendulum(ctx, frame_idx);
    frame_idx += 1;
  }, 20);
};

init();
