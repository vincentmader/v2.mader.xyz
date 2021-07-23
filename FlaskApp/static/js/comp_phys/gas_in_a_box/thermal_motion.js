import { draw_point } from "../../utils/drawing_utils.js";

const TAU = 2 * Math.PI;
const DT = 1;
const m = 1;
const v0 = 1;

const nr_of_particles = 20;

const particle_radius = 10;

var particles;

var canvas, ctx;
var W, H, o_x, o_y;

// var data = $("#canvas").data("ys");
// var system_state;
// const tail_length = 150;
// function draw_tails(ctx, frame_idx, tail_length) {
//   var current_system_state, previous_system_state;

//   for (const idx of Array(tail_length).keys()) {
//     current_system_state = data[Math.max(0, frame_idx - idx)];
//     previous_system_state = data[Math.max(0, frame_idx - idx - 1)];

//     phi_1p = previous_system_state[0];
//     phi_2p = previous_system_state[1];
//     foo = get_positions_from_angles(phi_1p, phi_2p);
//     x_1p = foo[0];
//     y_1p = foo[1];
// x_2p = foo[2];
// y_2p = foo[3];

// phi_1c = current_system_state[0];
// phi_2c = current_system_state[1];
// foo = get_positions_from_angles(phi_1c, phi_2c);
// x_1c = foo[0];
// y_1c = foo[1];
// x_2c = foo[2];
// y_2c = foo[3];
// draw_line(ctx, x_1p, y_1p, x_1c, y_1c, "green");
// draw_line(ctx, x_2p, y_2p, x_2c, y_2c, "red");
// }
// }

class Particle {
  constructor() {
    this.mass = m;
    this.x = W * Math.random();
    this.y = H * Math.random();
    this.speed = v0 * Math.random();
    this.theta = TAU * Math.random(); // direction of movement
    this.update_velocity();
  }
  update_velocity() {
    this.u = this.speed * Math.cos(this.theta);
    this.v = this.speed * Math.sin(this.theta);
  }
  update_position() {
    this.x += this.u * DT;
    this.y += this.v * DT;
  }
  draw() {
    draw_point(ctx, this.x, this.y, particle_radius);
  }
  handle_boundaries() {
    let u = this.u;
    let v = this.v;
    if (this.x < 0 || this.x > W) u *= -1;
    if (this.y < 0 || this.y > H) v *= -1;
    this.theta = Math.atan2(v, u);
  }
  update() {
    for (let p of particles) {
      handle_particle_collision(this, p);
    }
    this.handle_boundaries();
    this.update_velocity();
    this.update_position();
  }
}

function handle_particle_collision(p1, p2) {
  // check for collision in next time step
  let x1 = p1.x + p1.u * DT;
  let y1 = p1.y + p1.v * DT;
  let x2 = p2.x + p2.u * DT;
  let y2 = p2.y + p2.v * DT;
  let r = Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
  if (r < 2 * particle_radius) {
    // calculate new velocities
    let phi = Math.atan2(y2 - y1, Math.max(x2 - x1, x1 - x2));
    p1.u =
      ((p1.speed * Math.cos(p1.theta - phi) * (p1.mass - p2.mass) +
        2 * p2.mass * p2.speed * Math.cos(p2.theta - phi)) /
        (p1.mass + p2.mass)) *
        Math.cos(phi) +
      p1.speed * Math.sin(p1.theta - phi) * Math.cos(phi + Math.PI / 2);
    p1.v =
      ((p1.speed * Math.cos(p1.theta - phi) * (p1.mass - p2.mass) +
        2 * p2.mass * p2.speed * Math.cos(p2.theta - phi)) /
        (p1.mass + p2.mass)) *
        Math.sin(phi) +
      p1.speed * Math.sin(p1.theta - phi) * Math.sin(phi + Math.PI / 2);
    p2.v =
      ((p2.speed * Math.cos(p2.theta - phi) * (p2.mass - p1.mass) +
        2 * p1.mass * p1.speed * Math.cos(p1.theta - phi)) /
        (p2.mass + p1.mass)) *
        Math.cos(phi) +
      p2.speed * Math.sin(p2.theta - phi) * Math.cos(phi + Math.PI / 2);
    p2.v =
      ((p2.speed * Math.cos(p2.theta - phi) * (p2.mass - p1.mass) +
        2 * p1.mass * p1.speed * Math.cos(p1.theta - phi)) /
        (p2.mass + p1.mass)) *
        Math.sin(phi) +
      p2.speed * Math.sin(p2.theta - phi) * Math.sin(phi + Math.PI / 2);
    // recalculate speed & angle
    p1.theta = Math.atan2(p1.v, p1.u);
    p1.speed = Math.sqrt(p1.u ** 2 + p1.v ** 2);
    console.log(p1.theta);
    p2.theta = Math.atan2(p2.v, p2.u);
    p2.speed = Math.sqrt(p2.u ** 2 + p2.v ** 2);

    // TODO: unique radii
    // this.u =
    // this.u *= -1;
    // this.v *= -1;
    // p.u *= -1;
    // p.v *= -1;

    p1.x += p1.u * DT;
    p1.y += p1.v * DT; // TODO: proper elastic collisions!
    p2.x += p2.u * DT;
    p2.y += p2.v * DT;
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
