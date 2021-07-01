// 3-body dynamics - moon in Hill sphere

// Imports
import { draw_tails } from "./drawing_utils.js";
import { kepler_velocity } from "../physics_utils.js";
import { draw_point } from "../../utils/drawing_utils.js";

const v_K = kepler_velocity;

// Physical constants
const m = 0.15;
const M = 1;
const G = 1;
const TAU = 2 * Math.PI;
// Simulation parameters
const dt = 5e-3;
const r1 = 0.4;
const r2 = 0.05;
const w1 = v_K(r1);
const w2 = v_K(r2);
const r_H = r1 * Math.sqrt(m / (3 * M));
// World
var system_states = [];
// Drawing
var W, H;
const line_width = 3;
const tail_length = 700; // TODO: make changeable
const color_sun = "white";
const color_planet = "white";
const color_moon = "white";
const drawing_radius_sun = 20;
const drawing_radius_planet = 7;
const drawing_radius_moon = 2;
// Settings
var paused = false;
var draw_tails_bool = false;
var draw_orbit_bool = false;
var draw_hill_bool = false;
// Statistics
var frame_idx = 0;

// setup event listeners for button menu
function setup_event_listeners() {
  document
    .getElementById("button_toggle_pause")
    .addEventListener("click", function () {
      paused = !paused;
      if (paused) {
        document.getElementById("button_toggle_pause").innerHTML = "unpause";
      }
    });
  document
    .getElementById("button_toggle_draw_tails")
    .addEventListener("click", function () {
      if (!draw_tails_bool) system_states = [];
      draw_tails_bool = !draw_tails_bool;
      let button_toggle_draw_tails = document.getElementById(
        "button_toggle_draw_tails"
      );
      if (draw_tails_bool) {
        button_toggle_draw_tails.innerHTML = "hide tails";
      } else {
        button_toggle_draw_tails.innerHTML = "draw tails";
      }
    });
  document
    .getElementById("button_toggle_draw_orbit")
    .addEventListener("click", function () {
      draw_orbit_bool = !draw_orbit_bool;
      let button_toggle_draw_orbit = document.getElementById(
        "button_toggle_draw_orbit"
      );
      if (draw_orbit_bool) {
        button_toggle_draw_orbit.innerHTML = "hide lunar orbit";
      } else {
        button_toggle_draw_orbit.innerHTML = "draw lunar orbit";
      }
    });
  document
    .getElementById("button_toggle_draw_hill")
    .addEventListener("click", function () {
      draw_hill_bool = !draw_hill_bool;
      let button_toggle_draw_hill = document.getElementById(
        "button_toggle_draw_hill"
      );
      if (draw_hill_bool) {
        button_toggle_draw_hill.innerHTML = "hide Hill sphere";
      } else {
        button_toggle_draw_hill.innerHTML = "draw Hill sphere";
      }
    });
}

// main
export function main(canvas, ctx) {
  // define canvas geometry
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx.lineWidth = line_width;

  // other stuff, TODO: move towards beginning
  const zoom_level = 1 / W;
  var x1, y1, x2, y2;
  var t = 0;
  var system_state;

  // setup events for button menu
  setup_event_listeners();

  // run animation loop
  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // draw star
    draw_point(ctx, W / 2, H / 2, drawing_radius_sun, color_sun, color_sun);
    // draw planet
    x1 = r1 * Math.cos(w1 * t);
    y1 = r1 * Math.sin(w1 * t);
    draw_point(
      ctx,
      x1 / zoom_level + W / 2,
      -y1 / zoom_level + H / 2,
      drawing_radius_planet,
      color_planet,
      color_planet
    );
    // draw moon
    x2 = r1 * Math.cos(w1 * t) + r2 * Math.cos(w2 * t);
    y2 = r1 * Math.sin(w1 * t) + r2 * Math.sin(w2 * t);
    draw_point(
      ctx,
      x2 / zoom_level + W / 2,
      -y2 / zoom_level + H / 2,
      drawing_radius_moon,
      color_moon,
      color_moon
    );
    // push next state to array
    system_state = [0, 0, x1, y1, 0, 0, 0, 0, x2, y2, 0, 0];
    system_states.push(system_state);
    // draw orbit
    if (draw_orbit_bool) {
      let x_c = x1 / zoom_level + W / 2;
      let y_c = -y1 / zoom_level + H / 2;
      let r = Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2) / zoom_level;
      ctx.strokeStyle = color_moon;
      ctx.beginPath();
      ctx.arc(x_c, y_c, r, 0, TAU);
      ctx.stroke();
    }
    // draw Hill sphere
    if (draw_hill_bool) {
      let x_c = x1 / zoom_level + W / 2;
      let y_c = -y1 / zoom_level + H / 2;
      let r = r_H / zoom_level;
      ctx.setLineDash([5, 5]);
      ctx.beginPath();
      ctx.arc(x_c, y_c, r, 0, TAU);
      ctx.stroke();
      ctx.setLineDash([0, 0]);
    }
    // draw tails
    if (draw_tails_bool) {
      draw_tails(
        ctx,
        system_states,
        system_states.length - 1,
        tail_length,
        W,
        H,
        zoom_level
      );
    }
    // increment time if not on pause
    if (!paused) {
      t += dt;
      frame_idx += 1;
    }
  }, 1); // TODO: influence step size?
}
