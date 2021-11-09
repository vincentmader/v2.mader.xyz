import { draw_tails } from "./drawing_utils.js";
import { draw_bodies } from "./drawing_utils.js";
import { draw_velocities } from "./drawing_utils.js";
import { kepler_velocity } from "../physics_utils.js";

const EPSILON = 0; // 0.03
const M = 1;
const G = 1;
const dt = 5e-3;

var nr_of_planets = 4;
const R0 = 0.8;

var orbits_are_eccentric = false;
var paused = false;
const tail_length = 200;

var canvas, ctx, canvas_id;
var W, H, o_x, o_y;
var zoom_level, frame_idx;
const line_width = 3;
var system_state, system_states;

function setup_initial_system_state() {
  const r = R0;
  const phi = 2 * Math.PI;
  const phi_v = phi + Math.PI / 2;

  const y0 = [];
  for (const i of [1, 1, 0, 0, 0, 0]) {
    y0.push(i);
  }
  var w;
  // set intial speed
  if (!orbits_are_eccentric) {
    w = kepler_velocity(r); // .03
  } else {
    w = 0.6;
  }
  // rotate & add other planets
  var x, y, u, v;
  for (const i of Array(nr_of_planets).keys()) {
    let delta_phi = (2 * Math.PI * i) / nr_of_planets;
    x = r * Math.cos(phi + delta_phi);
    y = r * Math.sin(phi + delta_phi);
    u = w * Math.cos(phi_v + delta_phi);
    v = w * Math.sin(phi_v + delta_phi);
    for (const i of [1, 1, x, y, u, v]) {
      y0.push(i);
    }
  }
  return y0;
}

function get_next_system_state(system_state) {
  var new_system_state = [M, 1, 0, 0, 0, 0];
  var m, r, x, y, u, v, r;
  for (const i of Array(nr_of_planets).keys()) {
    m = system_state[6 * (i + 1)];
    r = system_state[6 * (i + 1) + 1];
    x = system_state[6 * (i + 1) + 2];
    y = system_state[6 * (i + 1) + 3];
    u = system_state[6 * (i + 1) + 4];
    v = system_state[6 * (i + 1) + 5];
    r = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2));
    u -= ((G * M * x) / Math.pow(r + EPSILON, 3)) * dt;
    v -= ((G * M * y) / Math.pow(r + EPSILON, 3)) * dt;
    x += u * dt;
    y += v * dt;
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
}

function setup_event_listeners() {
  document
    .getElementById("button_toggle_pause")
    .addEventListener("click", function () {
      paused = !paused;
      if (paused) {
        document.getElementById("button_toggle_pause").innerHTML = "unpause";
      } else {
        document.getElementById("button_toggle_pause").innerHTML = "pause";
      }
    });
  document
    .getElementById("button_toggle_eccentricity")
    .addEventListener("click", function () {
      orbits_are_eccentric = !orbits_are_eccentric;
      reset_canvas();
      if (orbits_are_eccentric) {
        document.getElementById("button_toggle_eccentricity").innerHTML =
          "circ. orbits";
      } else {
        document.getElementById("button_toggle_eccentricity").innerHTML =
          "ecc. orbits";
      }
    });

  document
    .getElementById("slider_nr_of_planets")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_nr_of_planets").value;
      nr_of_planets = Number(value);
      reset_canvas();
      display_nr_of_planets();
    });

  // let slider_nr_of_planets = document.getElementById("slider_nr_of_planets");
  // slider_nr_of_planets.addEventListener("click", function () {
  //   let value = document.getElementById("slider_nr_of_planets").value;
  //   nr_of_planets = Number(value);
  //   reset_canvas();
  // });
  // nr_of_planets = 12;
}

function display_nr_of_planets() {
  document.getElementById("display_nr_of_planets").innerHTML =
    "nr of planets: " + String(nr_of_planets);
  document.getElementById("slider_nr_of_planets").value = nr_of_planets;
}

function init() {
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
  display_nr_of_planets();
  setup_event_listeners();
}

function animate() {
  setInterval(function () {
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
}

export function main(canvas1, ctx1, canvas_id1) {
  canvas = canvas1;
  ctx = ctx1;
  canvas_id = canvas_id1;
  // TODO: get rid of above, rework fct call from html template

  init();
  animate();
}
