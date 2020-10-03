var canvas, width, height, ctx;

var bodies = [];

function init() {
  canvas = document.getElementById("myyCanvas");
  ctx = canvas.getContext("2d");

  createBodies();

  setInterval(function () {
    updateBodies(0.1);
    drawBodies();
  }, 100);
}

function Body(x, y, v, angle, mass) {
  this.x = x;
  this.y = y;
  this.vx = v * Math.cos(angle);
  this.vy = v * Math.sin(angle);
  this.m = mass;
  this.ax = 0;
  this.ay = 0;
  this.update = function (dt) {
    this.vx += this.ax * dt;
    this.vy += this.ay * dt;
    this.x += this.vx * dt;
    this.y += this.vy * dt;
    this.ax = 0;
    this.ay = 0;
  };
  this.draw = function (c) {
    c.beginPath();
    c.arc(this.x, this.y, 5, 0, 2 * Math.PI);
    c.stroke();
  };
}

function createBodies() {
  bodies.push(new Body(100, 200, 250, Math.Pi / 2, 10));
  bodies.push(new Body(150, 300, 250, Math.Pi / 2, 10));
  bodies.push(new Body(300, 250, 350, 0, 1));
  bodies.push(new Body(350, 250, 350, 0, 5));
  bodies.push(new Body(400, 300, 0, 0, 500000000));
}

function drawBodies() {
  for (let idx = 0; idx < bodies.length; idx++) {
    bodies[idx].draw(ctx);
  }
}

function updateBodies(dt) {
  for (let idx = 0; idx < bodies.length; idx++) {
    bodies[idx].update(dt);
  }
}

init();
