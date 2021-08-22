import { draw_point } from "../../utils/drawing_utils.js";

// CONSTANTS

const TAU = 2 * Math.PI;
const DT = 0.5;
const m = 1;
const k = 30; // electromagnetic interaction

// PARAMETERS

const nr_of_particles = 40; // 50-60 is lower bound for divergence on init
const particle_radius = 15;
var v0 = 1;

const fps_goal = 60;
var max_speed = v0;

// OTHER VARIABLES

var canvas, ctx, W, H, o_x, o_y;
var canvas2, ctx2, W2, H2, chart;
var frame_idx;

var particles;
var energy_0;

// CLASSES

class Particle {
  constructor(x, y, r) {
    this.m = m;
    this.r = r;
    this.x = x;
    this.y = y;
    this.speed = v0 * Math.random();
    this.theta = TAU * Math.random(); // direction of movement
    this.update_velocity(); // TODO: rename? (get u and v from speed/theta)
    this.color = "white";
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
    draw_point(ctx, this.x, this.y, particle_radius, color, color);
  }
  apply_forces() {
    // TODO: conserve energy?
    let x = this.x;
    let y = this.y;
    for (let p of particles) {
      if (this === p) continue;
      let r = Math.sqrt((p.x - x) ** 2 + (p.y - y) ** 2) - (this.r + p.r);
      let Fx = (k / r ** 3) * (x - p.x);
      let Fy = (k / r ** 3) * (y - p.y);
      this.u += (Fx / this.m) * DT;
      this.v += (Fy / this.m) * DT;
      this.speed = Math.sqrt(this.u ** 2 + this.v ** 2);
      this.theta = Math.atan2(this.v, this.u);
    }
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
    this.apply_forces();
    this.handle_boundaries();
    this.update_position();
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

function update_max_speed() {
  for (let idx = 0; idx < nr_of_particles; idx++) {
    let particle_speed = particles[idx].speed;
    if (particle_speed > max_speed) max_speed = particle_speed;
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

  // particles = [];
  // for (let idx = 0; idx < nr_of_particles; idx++) {
  // let x = (W - 3 * particle_radius) * Math.random() + particle_radius;
  // let y = (H - 3 * particle_radius) * Math.random() + particle_radius;
  // let p = new Particle(x, y, particle_radius);
  // particles.push(p);
  // console.log(free_spot_found);
  particles = [];
  for (let idx = 0; idx < nr_of_particles; idx++) {
    let r = particle_radius;
    let x = (W - 3 * r) * Math.random() + r;
    let y = (H - 3 * r) * Math.random() + r;
    let collision_found = true;
    while (collision_found) {
      collision_found = false;
      x = (W - 3 * r) * Math.random() + r;
      y = (H - 3 * r) * Math.random() + r;
      for (let jdx = 0; jdx < idx; jdx++) {
        let neighbor = particles[jdx];
        let distance = Math.sqrt((neighbor.x - x) ** 2 + (neighbor.y - y) ** 2);
        collision_found = distance < 2 * (neighbor.r + r);
        if (collision_found) {
          break;
        }
      }
    }
    let p = new Particle(x, y, r);
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
    // update & draw particles
    for (let p of particles) {
      p.update();
    }
    update_max_speed();
    for (let p of particles) {
      p.draw();
    }
    if (frame_idx % fps_goal === 0) {
      let e = calc_system_energy();
      if (frame_idx === 0) energy_0 = e;
      chart.data.labels.push(""); // TODO: ?
      chart.data.datasets[0].data.push(100 * (e / energy_0 - 1));
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
