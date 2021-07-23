import { draw_point } from "../../utils/drawing_utils.js";

const DT = 1;
const v0 = 1;

const nr_of_particles = 100;

const particle_radius = 5;

var particles;

var canvas, ctx;
var W, H, o_x, o_y;

// var data = $("#canvas").data("ys");
// var system_state;
// var frame_idx;
// var zoom_level = 100;
// var foo;

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

class Particle {
  constructor() {
    this.x = W * Math.random();
    this.y = H * Math.random();
    this.u = v0 * (2 * Math.random() - 1); // TODO: thermal dist? -> no! shall be reached in eq
    this.v = v0 * (2 * Math.random() - 1);
  }
  update_velocity() {
    // check for boundaries
    if (this.x < 0 || this.x > W) this.u *= -1;
    if (this.y < 0 || this.y > H) this.v *= -1;
    // check for collisions
    this.handle_collisions();
  }
  update_position() {
    this.x += this.u * DT;
    this.y += this.v * DT;
  }
  draw() {
    draw_point(ctx, this.x, this.y, particle_radius);
  }
  handle_collisions() {
    for (let p of particles) {
      let x1 = this.x + this.u * DT;
      let y1 = this.y + this.v * DT;
      let x2 = p.x + p.u * DT;
      let y2 = p.y + p.v * DT;
      let r = Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
      if (r < 2 * particle_radius) {
        // TODO: unique radii
        this.u *= -1;
        this.v *= -1;
        p.u *= -1;
        p.v *= -1;
        this.x += this.u * DT;
        this.y += this.v * DT; // TODO: proper elastic collisions!
        p.x += p.u * DT;
        p.y += p.v * DT;
      }
    }
  }
  update() {
    this.update_velocity();
    this.update_position();
  }
}

function init() {
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");
  o_x = W / 2;
  o_y = H / 2;

  particles = [];
  for (let idx = 0; idx < nr_of_particles; idx++) {
    let p = new Particle();
    particles.push(p);
  }
  // ctx.lineWidth = line_width;
  animate();
}

function animate() {
  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // update & draw particles
    for (let p of particles) {
      p.update();
      p.draw();
    }
  }, 1000 / 60);
}

// function animate() {
//   frame_idx = 0;
//   setInterval(function () {
//     //   // draw_tails(ctx, frame_idx);
//     ctx.clearRect(0, 0, canvas.width, canvas.height);
//     system_state = data[frame_idx];
//     var nr_of_bodies = system_state.length / 8;
//     var m, R, x, y, u, v;
//     for (let idx = 0; idx < nr_of_bodies; idx++) {
//       m = system_state[6 * idx];
//       R = system_state[6 * idx + 1];
//       x = system_state[6 * idx + 2];
//       y = system_state[6 * idx + 3];
//       u = system_state[6 * idx + 4];
//       v = system_state[6 * idx + 5];
//       foo = xy_to_canvas_coords(x, y, W, H, zoom_level);
//       x = foo[0];
//       y = foo[1];
//       draw_point(ctx, x, y, 1);
//     }

//     frame_idx += 1;
//   }, 20);
// }

init();
