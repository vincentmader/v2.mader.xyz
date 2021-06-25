import { draw_point } from "../../utils/drawing_utils.js";

const dt = 0.001;
const r_big = 0.06;
const r_atom = 2e-3;
const nr_of_atoms = 2000;
const m = 1e-2;
const M = 1;
const v_th = 10; // TODO: fix this, it's unphysical
var big_particle, atoms;
var canvas, ctx;
var W, H;
var frame_idx;
var big_particle_positions = [];

class Particle {
  constructor(r, x, y, u, v) {
    this.r = r;
    this.x = x;
    this.y = y;
    this.u = u;
    this.v = v;
  }
  updateVelocity() {
    const x_next = this.x + this.u * dt;
    const y_next = this.y + this.v * dt;
    // check for walls
    if (x_next <= this.r || x_next >= 1 - this.r) this.u *= -1;
    if (y_next <= this.r || y_next >= 1 - this.r) this.v *= -1;
    // check for big particle
    if (this != big_particle) {
      const x_bp_next = big_particle.x + big_particle.u * dt;
      const y_bp_next = big_particle.y + big_particle.v * dt;
      var dx = x_bp_next - x_next;
      var dy = y_bp_next - y_next;
      var r = Math.sqrt(dx ** 2 + dy ** 2);
      var u1, u2, v1, v2;
      if (r <= this.r + big_particle.r) {
        // u1 = big_particle.u;
        // v1 = big_particle.v;
        // u2 = this.u;
        // v2 = this.v;
        // big_particle.u = ((M - m) / (m + M)) * u1 + ((2 * m) / (m + M)) * u2;
        // big_particle.v = ((2 * M) / (m + M)) * v2 + ((m - M) / (m + M)) * v1;
        // this.u = ((m - M) / (m + M)) * u2 + ((2 * M) / (m + M)) * u1;
        // this.v = ((2 * m) / (m + M)) * v2 + ((M - m) / (m + M)) * v1;
        // this.u += (M / m) * foo;
        // this.v += (M / m) * bar;
        big_particle.u += (m / M) * this.u;
        big_particle.v += (m / M) * this.v;
        this.u *= -1;
        this.v *= -1;
      }
    }
  }
  updatePosition() {
    // console.log(this.u);
    this.x += this.u * dt;
    this.y += this.v * dt;
  }
  draw() {
    const canvas_x = W * this.x;
    const canvas_y = W * this.y;
    const canvas_r = W * this.r;
    draw_point(ctx, canvas_x, canvas_y, canvas_r);
  }
}

function initialize_bodies(nr_of_atoms) {
  var x, y, u, v;
  // initialize big particle
  x = 0.5;
  y = 0.5;
  u = 0;
  v = 0;
  big_particle = new Particle(r_big, x, y, u, v);
  // initialize small atoms
  var atom, free_spot_found;
  var dx, dy;
  atoms = [];
  for (var i = 0; i < nr_of_atoms; i++) {
    free_spot_found = false;
    while (!free_spot_found) {
      x = Math.random() * 0.9 + 0.05;
      y = Math.random() * 0.9 + 0.05;
      dx = x - big_particle.x;
      dy = y - big_particle.y;
      if (Math.sqrt(dx ** 2 + dy ** 2) ** 0.5) {
        free_spot_found = true;
      }
    }
    u = v_th * (2 * Math.random() - 1);
    v = v_th * (2 * Math.random() - 1);
    atom = new Particle(r_atom, x, y, u, v);
    atoms.push(atom);
  }
}

function draw_big_particle_positions() {
  var x, y;
  for (const i of big_particle_positions) {
    x = W * i[0];
    y = W * i[1];
    draw_point(ctx, x, y, 1, "red", "red");
  }
}

function init() {
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");

  // ctx.lineWidth = line_width;
  initialize_bodies(nr_of_atoms);

  frame_idx = 0;
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    big_particle.updateVelocity();
    big_particle.updatePosition();
    big_particle.draw();
    // console.log(big_particle.u);
    for (const p of atoms) {
      p.updateVelocity();
      p.updatePosition();
      p.draw();
    }
    big_particle_positions.push([big_particle.x, big_particle.y]);
    draw_big_particle_positions();

    frame_idx += 1;
  }, 0.001); // TODO: make changeable
}

init();
