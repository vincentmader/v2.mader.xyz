import { draw_tails } from "./drawing_utils.js";
import { draw_bodies } from "./drawing_utils.js";
import { draw_velocities } from "./drawing_utils.js";
import { kepler_velocity } from "./physics_utils.js";

const line_width = 3;
const tail_length = 200;
const dt = 5e-3;
const EPSILON = 0; // 0.03
const M = 1;
const G = 1;

var W, H, o_x, o_y;
var zoom_level;
var initial_nr_of_planets;
var nr_of_planets;
var frame_idx;
var system_state, system_states;
var orbits_are_eccentric = false;
var paused = false;

function transform_coords_cart2pol(x, y) {
  const r = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2));
  const phi = Math.atan2(y, x);
  return [r, phi];
}
function transform_coords_pol2cart(r, phi) {
  const x = r * Math.cos(phi);
  const y = r * Math.sin(phi);
  return [x, y];
}

function setup_initial_system_state() {
  const y0 = [];
  for (const i of [1, 1, 0, 0, 0, 0]) {
    y0.push(i);
  }
  var r, phi, x, y, phi_v, u, v, w;
  for (const i of Array(nr_of_planets).keys()) {
    // const i = 0;
    r = 0.8;
    phi = phi = 2 * Math.PI * (i / nr_of_planets);
    x = r * Math.cos(phi);
    y = r * Math.sin(phi);
    phi_v = phi + Math.PI / 2;
    if (!orbits_are_eccentric) {
      w = kepler_velocity(r); // .03
    } else {
      w = 0.6;
    }
    u = w * Math.cos(phi_v);
    v = w * Math.sin(phi_v);
    for (const i of [1, 1, x, y, u, v]) {
      y0.push(i);
    }
  }
  return y0;
}

function get_next_system_state(system_state) {
  var new_system_state = [M, 1, 0, 0, 0, 0];
  var m, r, x, y, u, v, r;
  // for (const i of Array(nr_of_planets).keys()) {
  m = system_state[6];
  r = system_state[7];
  x = system_state[8];
  y = system_state[9];
  u = system_state[10];
  v = system_state[11];
  r = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2));
  u -= ((G * M * x) / Math.pow(r + EPSILON, 3)) * dt;
  v -= ((G * M * y) / Math.pow(r + EPSILON, 3)) * dt;
  x += u * dt;
  y += v * dt;
  var tmp, r, phi;
  for (const i of Array(nr_of_planets).keys()) {
    tmp = transform_coords_cart2pol(x, y);
    r = tmp[0];
    phi = tmp[1];
    phi += (i / nr_of_planets) * 2 * Math.PI;
    tmp = transform_coords_pol2cart(r, phi);
    x = tmp[0];
    y = tmp[1];
    for (const j of [m, r, x, y, u, v]) {
      new_system_state.push(j);
    }
  }

  return new_system_state;
}

function reset_canvas() {
  frame_idx = 0;
  system_states = [];
  system_state = setup_initial_system_state();
  system_states.push(system_state);
  initial_nr_of_planets = nr_of_planets;
}

export function main(canvas, ctx, canvas_id) {
  // define geometry
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx.lineWidth = line_width;
  // define coordinate origin on screen
  o_x = W / 2;
  o_y = H / 2;
  zoom_level = 2 / W;

  reset_canvas();

  setInterval(function () {
    nr_of_planets = Number(
      document.getElementById("nr_of_bodies_textfield").value
    );
    if (nr_of_planets != initial_nr_of_planets) {
      if (nr_of_planets == 0) {
        nr_of_planets = initial_nr_of_planets;
      } else {
        reset_canvas();
      }
    }
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // draw
    draw_bodies(ctx, system_state, nr_of_planets + 1, W, H, zoom_level);
    if (frame_idx > 1) {
      draw_tails(
        ctx,
        system_states,
        system_states.length - 1,
        tail_length,
        W,
        H,
        zoom_level
      );
      // draw_velocities(ctx, system_state, nr_of_planets + 1, W, H, zoom_level);
    }
    if (!paused) {
      system_state = get_next_system_state(system_state);
      system_states.push(system_state);
      if (system_states.length > tail_length) {
        system_states.splice(0, 1);
      }
      frame_idx += 1;
    }
  }, 1);
  document.getElementById("play/pause").addEventListener("click", function () {
    k;
    paused = !paused;
  });
  document
    .getElementById("toggle_eccentricity")
    .addEventListener("click", function () {
      orbits_are_eccentric = !orbits_are_eccentric;
      reset_canvas();
    });
}
