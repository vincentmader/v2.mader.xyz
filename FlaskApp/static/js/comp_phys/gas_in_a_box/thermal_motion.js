import { draw_point } from "../../utils/drawing_utils.js";

// CONSTANTS

const TAU = 2 * Math.PI;
const DT = 1;
const m = 1;
const v0 = 3;

// PARAMETERS

const nr_of_particles = 100;
const particle_radius = 4;

const fps_goal = 60;

// OTHER VARIABLES

var canvas, ctx, W, H, o_x, o_y;
var canvas2, ctx2, W2, H2, chart;
var frame_idx;

var particles;

// CLASSES

class Particle {
  constructor() {
    this.mass = m;
    this.r = particle_radius;
    this.x = (W - 3 * this.r) * Math.random() + this.r;
    this.y = (H - 3 * this.r) * Math.random() + this.r;
    this.speed = v0 * Math.random();
    this.theta = TAU * Math.random(); // direction of movement
    this.update_velocity(); // TODO: rename? (get u and v from speed/theta)
    this.color = "white"; // TODO: make speed-dep. transition to red
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
    // get expected particle position in next time step
    let x_new = this.x + this.u * DT;
    let y_new = this.y + this.v * DT;
    // if crossing wall: reverse vel. component & move back
    // TODO: good?
    if (x_new > W - this.r || x_new < this.r) {
      this.u *= -1;
      this.x += this.u * DT;
    }
    if (y_new > W - this.r || y_new < this.r) {
      this.v *= -1;
      this.y += this.v * DT;
    }
    // update velocity angle & vector component values
    this.theta = Math.atan2(this.v, this.u);
    this.update_velocity();
  }
  update() {
    for (let p of particles) {
      handle_particle_collision(this, p);
    }
    this.handle_boundaries();
    this.update_position();
  }
}

function handle_particle_collision(p1, p2) {
  var collision_found = false;
  var x1, x2, y1, y2;
  for (let xi of [0.2, 0.4, 0.6, 0.8, 1]) {
    // get expected particle positions in next time step
    x1 = p1.x + xi * p1.u * DT;
    y1 = p1.y + xi * p1.v * DT;
    x2 = p2.x + xi * p2.u * DT;
    y2 = p2.y + xi * p2.v * DT;
    // check for collision
    let d = Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
    if (d < 2 * particle_radius) {
      collision_found = true;
      break;
    }
  }
  if (collision_found) {
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
    p2.theta = Math.atan2(p2.v, p2.u);
    p2.speed = Math.sqrt(p2.u ** 2 + p2.v ** 2);

    // TODO: unique radii
    // this.u =
    // this.u *= -1;
    // this.v *= -1;
    // p.u *= -1;
    // p.v *= -1;

    // p1.x += p1.u * DT;
    // p1.y += p1.v * DT; // TODO: proper elastic collisions!
    // p2.x += p2.u * DT;
    // p2.y += p2.v * DT;

    let d = Math.sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2);
    if (d < p2.r + p1.r) {
      p1.x += (p1.x - p2.x) / 2; // (2 * d)) * (d - 2 * p1.r);
      p1.y += (p1.y - p2.y) / 2; // (2 * d)) * (d - 2 * p1.r);
      p2.x += (p2.x - p1.x) / 2; // (2 * d)) * (d - 2 * p2.r);
      p2.y += (p2.y - p1.y) / 2; // (2 * d)) * (d - 2 * p2.r);
    }
    p1.update_velocity();
    p2.update_velocity();
  }
}

function create_chart() {
  canvas2 = document.getElementById("canvas_chart");
  ctx2 = canvas2.getContext("2d");
  W2 = canvas2.getBoundingClientRect().width;
  canvas2.height = W2 / 2;

  chart = new Chart(ctx2, {
    type: "line",
    data: {
      datasets: [
        {
          borderColor: "white",
          pointRadius: 0,
          data: [],
          showLine: true, // overrides the `line` dataset default
          label: "total energy",
        },
      ],
    },
    options: {
      // scales: {
      //   yAxes: [
      //     {
      //       display: true,
      //       ticks: {
      // suggestedMax: 1,
      // suggestedMin: -1,
      // beginAtZero: true   // minimum value will be 0.
      // type: "logarithmic",
      // },
      // },
      // ],
      // },
    },
  });
}

function calc_system_energy() {
  let energy = 0;
  for (let p of particles) {
    energy += p.mass * p.speed ** 2;
  }
  return energy;
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
  frame_idx = 0;
  create_chart();
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
    // if (frame_idx % (fps_goal / 4) === 0) {
    let e = calc_system_energy();
    chart.data.labels.push(""); // TODO: ?
    chart.data.datasets[0].data.push(e);
    chart.update();
    // }
    frame_idx += 1;
  }, 1000 / fps_goal);
}

// function animate() {
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
//   }, 20);
// }

init();
