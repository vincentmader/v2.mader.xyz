import { draw_point } from "../../utils/drawing_utils.js";

// CONSTANTS

const TAU = 2 * Math.PI;
const DT = 1;
const m = 1;

// PARAMETERS

const nr_of_particles = 75; // TODO: add sliders
var particle_radius = 1 / 50;
// const nr_of_particles = 200;
// const particle_radius = 5;
var v0 = 3;

const fps_goal = 60;
const max_speed = 1 * v0;

// OTHER VARIABLES

var canvas, ctx, W, H, o_x, o_y;
var canvas2, ctx2, W2, H2, chart;
var frame_idx;

var particles;
var energy_0;

// CLASSES

class Particle {
  constructor() {
    this.m = m;
    this.r = particle_radius * W;
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
    let foo = 255 * (1 - this.speed / max_speed);
    let color = "rgba(255," + foo + ", " + foo + ", 1)";
    draw_point(ctx, this.x, this.y, this.r, color, color);
  }
  handle_boundaries() {
    // get expected particle position in next time step
    let x_new = this.x + this.u * DT;
    let y_new = this.y + this.v * DT;
    // if crossing wall: reverse vel. component & move back
    if (x_new >= W - this.r) {
      this.u *= -1;
      this.x = W - this.r;
    } else if (x_new <= this.r) {
      this.u *= -1;
      this.x = this.r;
    }
    if (y_new >= H - this.r) {
      this.v *= -1;
      this.y = H - this.r;
    } else if (y_new <= this.r) {
      this.v *= -1;
      this.y = this.r;
    }
    // update velocity angle & vector component values
    this.speed = Math.sqrt(this.u ** 2 + this.v ** 2);
    this.theta = Math.atan2(this.v, this.u);
    this.update_velocity();
  }
  update() {
    this.handle_boundaries();
    this.update_position();
  }
}

function handle_particle_collision(p1, p2) {
  var collision_found = false;
  var x1, x2, y1, y2;
  for (let xi of [0.5, 1]) {
    // TODO: make xi parameter continuous?
    // get expected particle positions in next time step
    x1 = p1.x + xi * p1.u * DT;
    y1 = p1.y + xi * p1.v * DT;
    x2 = p2.x + xi * p2.u * DT;
    y2 = p2.y + xi * p2.v * DT;
    // check for collision
    let d = Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
    if (d <= p1.r + p2.r) {
      collision_found = true;
      break;
    }
  }
  if (collision_found) {
    // calculate new velocities
    let phi = Math.atan2(y2 - y1, x2 - x1) - Math.PI;
    // let phi = Math.atan2(y2 - y1, Math.max(x2 - x1, x1 - x2));
    p1.u =
      ((p1.speed * Math.cos(p1.theta - phi) * (p1.m - p2.m) +
        2 * p2.m * p2.speed * Math.cos(p2.theta - phi)) /
        (p1.m + p2.m)) *
        Math.cos(phi) +
      p1.speed * Math.sin(p1.theta - phi) * Math.cos(phi + Math.PI / 2);
    p1.v =
      ((p1.speed * Math.cos(p1.theta - phi) * (p1.m - p2.m) +
        2 * p2.m * p2.speed * Math.cos(p2.theta - phi)) /
        (p1.m + p2.m)) *
        Math.sin(phi) +
      p1.speed * Math.sin(p1.theta - phi) * Math.sin(phi + Math.PI / 2);
    p2.u =
      ((p2.speed * Math.cos(p2.theta - phi) * (p2.m - p1.m) +
        2 * p1.m * p1.speed * Math.cos(p1.theta - phi)) /
        (p2.m + p1.m)) *
        Math.cos(phi) +
      p2.speed * Math.sin(p2.theta - phi) * Math.cos(phi + Math.PI / 2);
    p2.v =
      ((p2.speed * Math.cos(p2.theta - phi) * (p2.m - p1.m) +
        2 * p1.m * p1.speed * Math.cos(p1.theta - phi)) /
        (p2.m + p1.m)) *
        Math.sin(phi) +
      p2.speed * Math.sin(p2.theta - phi) * Math.sin(phi + Math.PI / 2);
    // recalculate speed & angle
    p1.theta = Math.atan2(p1.v, p1.u);
    p1.speed = Math.sqrt(p1.u ** 2 + p1.v ** 2);
    p2.theta = Math.atan2(p2.v, p2.u);
    p2.speed = Math.sqrt(p2.u ** 2 + p2.v ** 2);

    let d = Math.sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2);
    p1.x += (p1.r + p2.r - d) * Math.cos(phi); // (p1.x - p2.x) / 2; // - p1.r * Math.cos(phi); // (2 * d)) * (d - 2 * p1.r);
    p1.y += (p1.r + p2.r - d) * Math.sin(phi); // (p1.y - p2.y) / 2; // - p1.r * Math.sin(phi); // (2 * d)) * (d - 2 * p1.r);
    p2.x -= (p1.r + p2.r - d) * Math.cos(phi); // (p2.x - p1.x) / 2; // - p2.r * Math.cos(phi); // (2 * d)) * (d - 2 * p2.r);
    p2.y -= (p1.r + p2.r - d) * Math.sin(phi); // (p2.y - p1.y) / 2; // - p2.r * Math.sin(phi); // (2 * d)) * (d - 2 * p2.r);

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
          label: "total energy error (%)",
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
    energy += p.m * p.speed ** 2;
  }
  return energy;
}

function get_possible_collision_pairs() {
  let possible_collision_pairs = [];
  let particle_idx_from_pos = {};
  let xs = [];
  for (let idx = 0; idx < particles.length; idx++) {
    let p = particles[idx];
    let x_new = p.x + p.v * DT;
    xs.push(x_new);
    particle_idx_from_pos[x_new] = idx;
  }
  let xs_sorted = xs.sort();
  for (let idx = 0; idx < xs_sorted.length; idx++) {
    let x1 = xs_sorted[idx];
    let p1 = particles[particle_idx_from_pos[x1]];
    for (let jdx = 0; jdx < xs_sorted.length; jdx++) {
      let x2 = xs_sorted[jdx];
      let p2 = particles[particle_idx_from_pos[x2]];
      if (idx > jdx) continue;
      if (Math.abs(p1.x - p2.x) <= p1.r + p2.r) {
        possible_collision_pairs.push([p1, p2]);
      }
    }
  }
  return possible_collision_pairs;
}

// function handle_user_inputs() {
// }

function setup_event_listeners() {
  // document.getElementById("slider_v0").addEventListener("click", function () {
  //   let value = document.getElementById("slider_v0").value;
  //   v0 = Number(value);
  //   console.log("new v_0: ", v0);
  //   init();
  // });
  // document.getElementById("slider_v0").value = v0;
  // document.getElementById("v0_display").innerHTML =
  //   "v0 = " + String(v0.toFixed(1));
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
  setup_event_listeners();
  animate();
}

function animate() {
  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // get possible collisions
    let possible_collision_pairs = get_possible_collision_pairs(); // TODO: rename
    for (let i of possible_collision_pairs) {
      for (let p of particles) {
        handle_particle_collision(i[0], i[1]);
      }
    }
    // update & draw particles
    for (let p of particles) {
      p.update();
      p.draw();
    }
    if (frame_idx % (fps_goal / 6) === 0) {
      let e = calc_system_energy();
      if (frame_idx === 0) energy_0 = e;
      chart.data.labels.push(""); // TODO: ?
      let energy_error = 100 * (e / energy_0 - 1);
      chart.data.datasets[0].data.push(energy_error);
      chart.update();
    }
    // handle_user_inputs();
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
