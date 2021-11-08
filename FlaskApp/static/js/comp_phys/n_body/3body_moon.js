// 3-body dynamics - moon in Hill sphere

// Imports
import { draw_tails } from "./drawing_utils.js";
import { kepler_velocity } from "../physics_utils.js";
import { draw_point } from "../../utils/drawing_utils.js";
const v_K = kepler_velocity;

// physical constants
const m = 0.15;
const M = 1;
const G = 1;
const TAU = 2 * Math.PI;
// simulation parameters
const dt = 1.5e-2;
const r1 = 0.4;
const r2 = 0.065;
const w1 = v_K(r1);
const w2 = v_K(r2);
const r_H = r1 * Math.sqrt(m / (3 * M));

// world
var system_states = [];
var t;
// drawing
var canvas, ctx, W, H;
const line_width = 3;
const tail_length = 250; // TODO: make changeable
const color_sun = "white";
const color_planet = "white";
const color_moon = "white";
var drawing_radius_sun; // = 20;
var drawing_radius_planet; // = 7;
var drawing_radius_moon; // = 2;

// settings
var paused = false;
var draw_tails_bool = false;
var draw_orbit_bool = false;
var draw_hill_bool = false;

// setup event listeners for button menu
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

function animate() {
  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // draw star
    draw_point(ctx, W / 2, H / 2, drawing_radius_sun, color_sun, color_sun);
    // draw planet
    var x1, y1, x2, y2;
    const zoom_level = 1 / W;
    var system_state;
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
    }
  }, 1000 / 60); // TODO: influence step size?
}

function init() {
  canvas = document.getElementById("canvas");
  ctx = canvas.getContext("2d");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx.lineWidth = line_width;

  drawing_radius_sun = W / 10;
  drawing_radius_planet = W / 35;
  drawing_radius_moon = W / 100;

  t = 0;
  // setup events for button menu
  setup_event_listeners();
}

// canvas = canvas2;
// ctx = ctx2;
// TODO: get rid of above?

init();
animate();
