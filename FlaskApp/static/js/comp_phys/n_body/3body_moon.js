import { draw_tails } from "./drawing_utils.js";
import { draw_velocities } from "./drawing_utils.js";
import { kepler_velocity } from "../physics_utils.js";
import { draw_point } from "../../utils/drawing_utils.js";

const line_width = 3;
const tail_length = 700;
const dt = 5e-3;
const M = 1;
const G = 1;

var W, H, o_x, o_y;
var frame_idx;
var paused = false;

var planets = [];

// class Planet {
//   constructor(r, phi, v, psi) {
//     this.r = r;
//     this.phi = phi;
//     this.v = v;
//     this.psi = psi;
//   }
// }

// const initialize_system = () => {
//   Sun = Planet(0, 0, 0, 0);
//   Earth = Planet(1, 0, 0, 1);
// };

const v_K = kepler_velocity;
// const v_K = (r, M = 1, G = 1) => {
//   return Math.sqrt((G * M) / r);
// };

export function main(canvas, ctx) {
  // define geometry
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx.lineWidth = line_width;
  // define coordinate origin on screen
  o_x = W / 2;
  o_y = H / 2;
  const zoom_level = 1 / W;

  const r1 = 0.4;
  const w1 = v_K(r1);
  const r2 = 0.05;
  const w2 = v_K(r2);
  var x1, y1, x2, y2;
  var t = 0;
  var system_states = [];
  var system_state;
  // var nr_of_planets = 2;
  var draw_tails_bool = false;

  document
    .getElementById("button_toggle_draw_tails")
    .addEventListener("click", function () {
      if (!draw_tails_bool) system_states = [];
      draw_tails_bool = !draw_tails_bool;
    });

  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // draw
    draw_point(ctx, W / 2, H / 2, 10);

    x1 = r1 * Math.cos(w1 * t);
    y1 = r1 * Math.sin(w1 * t);
    draw_point(ctx, x1 / zoom_level + W / 2, -y1 / zoom_level + H / 2, 3);
    x2 = r1 * Math.cos(w1 * t) + r2 * Math.cos(w2 * t);
    y2 = r1 * Math.sin(w1 * t) + r2 * Math.sin(w2 * t);
    draw_point(ctx, x2 / zoom_level + W / 2, -y2 / zoom_level + H / 2, 1);

    system_state = [0, 0, x1, y1, 0, 0, 0, 0, x2, y2, 0, 0];
    system_states.push(system_state);
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
    // draw_velocities(ctx, system_state, nr_of_planets + 1, W, H, zoom_level);
    if (!paused) {
      t += 0.005;
    }
  }, 1);
}
