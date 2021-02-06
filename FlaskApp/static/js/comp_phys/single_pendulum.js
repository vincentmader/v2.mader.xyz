import { draw_line } from "../utils/drawing_utils.js";

const π = Math.PI;
const line_width = 2;
const dt = 1;
const g = 1;
const tail_length = 20;

var canvas, ctx;
var W, H, o_x, o_y;
var frame_idx;
var ys = [];
var L, r;

function Pendulum(o_x, o_y, φ) {
  // set coords of pivot origin
  o_x = o_x;
  o_y = o_y;
  // set angular coords of pendulum mass
  this.φ = φ;
  this.ω = 0;
  this.α = 0;

  this.move = () => {
    this.α = (-g / L) * Math.sin(this.φ);
    this.ω += this.α * dt;
    this.φ += this.ω * dt;
    ys.push(this.φ);
  };

  this.draw = () => {
    let p_x = o_x + L * Math.sin(this.φ); // cart. coords of pendulum mass
    let p_y = o_y + L * Math.cos(this.φ);
    // draw rod
    draw_line(ctx, o_x, o_y, p_x, p_y, "white");
    // draw mass
    ctx.beginPath();
    ctx.arc(p_x, p_y, r, 0, 2 * Math.PI);
    ctx.fill();
    // draw hinge
    ctx.beginPath();
    ctx.arc(o_x, o_x, r, 0, 2 * Math.PI);
    ctx.fill();
  };
}

function draw_tail(ctx, frame_idx, tail_length) {
  var φ_c, φ_p;

  for (const idx of Array(tail_length).keys()) {
    φ_c = ys[Math.max(0, frame_idx - idx)];
    φ_p = ys[Math.max(0, frame_idx - idx - 1)];

    // previous cart. coords of pendulum mass
    let p_x = o_x + L * Math.sin(φ_p);
    let p_y = o_y + L * Math.cos(φ_p);
    // current cart. coords of pendulum mass
    let q_x = o_x + L * Math.sin(φ_c);
    let q_y = o_y + L * Math.cos(φ_c);

    draw_line(ctx, p_x, p_y, q_x, q_y, "green");
  }
}

const init = () => {
  canvas = document.getElementById("single_pendulum_canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;

  o_x = W / 2;
  o_y = H / 2;

  ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";

  L = W / 4 - 10;
  r = W / 100;

  frame_idx = 0;
  const p = new Pendulum(W / 2, H / 2, 0.99 * π);
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw_tail(ctx, frame_idx, tail_length);
    p.draw();
    p.move();
    frame_idx += 1;
  }, 20);
};

init();
