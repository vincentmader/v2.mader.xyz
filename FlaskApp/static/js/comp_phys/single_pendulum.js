import { draw_line } from "./a.js";

const π = Math.PI;
const line_width = 2;
const dt = 1;
const g = 1;
const R = 20;
const tail_length = 20;

var canvas, ctx;
var W, H;
var L;
var circle_radius;
var o_x, o_y;
var ys = [];
var frame_idx;

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

    ctx.beginPath();
    ctx.strokeStyle = "white";
    ctx.fillStyle = "white";
    // draw mass
    ctx.arc(p_x, p_y, circle_radius, 0, 2 * Math.PI);
    // ctx.stroke();
    ctx.fill();

    ctx.beginPath();
    ctx.arc(o_x, o_x, circle_radius, 0, 2 * Math.PI);
    // ctx.stroke();
    ctx.fill();

    // draw rod
    ctx.beginPath();
    ctx.moveTo(o_x, o_y);
    ctx.lineTo(p_x, p_y);
    ctx.stroke();
  };
}

function draw_tail(ctx, frame_idx, tail_length) {
  var φ_c, φ_p;

  for (const idx of Array(tail_length).keys()) {
    φ_c = ys[Math.max(0, frame_idx - idx)];
    φ_p = ys[Math.max(0, frame_idx - idx - 1)];

    let p_x = o_x + L * Math.sin(φ_p);
    let p_y = o_y + L * Math.cos(φ_p); // cart. coords of pendulum mass

    let q_x = o_x + L * Math.sin(φ_c);
    let q_y = o_y + L * Math.cos(φ_c); // cart. coords of pendulum mass

    draw_line(ctx, p_x, p_y, q_x, q_y, "green");
  }
}

const init = () => {
  canvas = document.getElementById("single_pendulum_canvas");
  // W = canvas.parentElement.clientWidth;
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;

  o_x = W / 2;
  o_y = H / 2;

  ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;

  L = W / 4 - 10;
  circle_radius = W / 100;

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
