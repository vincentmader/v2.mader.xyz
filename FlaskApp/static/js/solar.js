var canvas, width, height, ctx;
var bodies = [];
let G = 1000;

function Body(x, y, v, angle, mass, radius) {
  this.x = x;
  this.y = y;
  this.vx = v * Math.cos(angle);
  this.vy = v * Math.sin(angle);
  this.m = mass;
  this.radius = radius;
  this.ax = 0;
  this.ay = 0;
  this.tail = new Tail(100);
  this.update = function (dt) {
    if (this.tail) {
      // console.log(this.tail);
      this.tail.addPoint({ x: this.x, y: this.y }); // what's going on here?
    }
    this.vx += this.ax * dt;
    this.vy += this.ay * dt;
    // console.log(this.ay);
    this.x += this.vx * dt;
    this.y += this.vy * dt;
    // console.log(this.ay);
    // console.log(this.vy);
    this.ax = 0;
    this.ay = 0;
  };
  this.draw = function () {
    ctx.beginPath();
    ctx.strokeStyle = "black";
    ctx.FillStyle = "black";
    ctx.arc(
      this.x + canvas.width / 2,
      -this.y + canvas.height / 2,
      this.radius,
      1,
      360
    );
    ctx.stroke();
    ctx.fill();

    if (this.tail) {
      // console.log(this.tail);
      this.tail.draw();
    }
  };
}

function Tail(maxLength) {
  this.points = [];
  this.maxLength = maxLength;
  this.addPoint = function (point) {
    for (let i = 1; i <= Math.min(this.maxLength, this.points.length); i++) {
      // console.log(Math.min(this.maxLength, this.points.length));
      this.points[i] = this.points[i - 1];
    }
    this.points[0] = point;
    // console.log(this.points);
  };
  this.draw = function () {
    // console.log(this.points);
    for (let i = 1; i < Math.min(this.maxLength, this.points.length); i++) {
      // console.log(idx);
      ctx.beginPath();
      ctx.moveTo(this.points[i - 1].x, this.points[i - 1].y);
      ctx.strokeStyle = "black";
      ctx.lineTo(this.points[i].x, this.points[i].y);
      ctx.stroke();
    }
  };
}

function createBodiesCrowded() {
  bodies = [];
  bodies.push(new Body(0, 0, 0, 0, 1, 20));
  bodies.push(new Body(30, 0, Math.sqrt(G / 30), Math.PI / 2, 10e-9, 2));
  bodies.push(new Body(40, 0, Math.sqrt(G / 50), Math.PI / 2, 10e-8, 3));
  bodies.push(new Body(65, 0, Math.sqrt(G / 80), Math.PI / 2, 10e-8, 5));
  bodies.push(new Body(80, 0, Math.sqrt(G / 80), Math.PI / 2, 10e-8, 5));
  bodies.push(new Body(140, 0, Math.sqrt(G / 140), Math.PI / 2, 10e-6, 10));
  bodies.push(new Body(200, 0, Math.sqrt(G / 200), Math.PI / 2, 10e-6, 7));
  bodies.push(new Body(240, 0, Math.sqrt(G / 240), Math.PI / 2, 10e-6, 6));
  bodies.push(new Body(268, 0, Math.sqrt(G / 240), Math.PI / 2, 10e-10, 1));
  bodies.push(new Body(280, 0, Math.sqrt(G / 240), Math.PI / 2, 10e-10, 1));

  for (let idx = 0; idx < 100; idx++) {
    d = 30 + Math.random() * Math.min(canvas.width, canvas.height);
    bodies.push(new Body(-d, 0, Math.sqrt(G / d), -Math.PI / 2, 10e-10, 1));
    bodies.push(new Body(d, 0, Math.sqrt(G / d), Math.PI / 2, 10e-10, 1));
    bodies.push(new Body(0, d, Math.sqrt(G / d), Math.PI, 10e-10, 1));
    bodies.push(new Body(0, -d, Math.sqrt(G / d), 0, 10e-10, 1));
  }
  return bodies;
}

function createBodiesThreeBodyProblem() {
  bodies = [];
  bodies.push(new Body(0, 0, 0, 0, 1, 30));
  bodies.push(new Body(200, 0, Math.sqrt(G / 200), Math.PI / 2, 10 ** -2, 4));
  bodies.push(
    new Body(
      212,
      0,
      Math.sqrt((G * 10 ** -2) / 12) + Math.sqrt(G / 208),
      Math.PI / 2,
      10 ** -9,
      1
    )
  );
  return bodies;
}

function createBodiesDoubleSystem() {
  bodies = [];
  bodies.push(new Body(200, 0, Math.sqrt(G / 2000), -Math.PI / 2, 1, 30));
  bodies.push(new Body(-200, 0, Math.sqrt(G / 2000), Math.PI / 2, 1, 30));
  return bodies;
}

function createBodiesQuartetSystem() {
  bodies = [];
  bodies.push(new Body(200, 0, Math.sqrt(G / 1500), -Math.PI / 2, 1, 10));
  bodies.push(new Body(-200, 0, Math.sqrt(G / 1500), Math.PI / 2, 1, 10));
  bodies.push(new Body(0, 200, Math.sqrt(G / 1500), 0, 1, 10));
  bodies.push(new Body(0, -200, Math.sqrt(G / 1500), Math.PI, 1, 10));
  return bodies;
}

function createBodiesFreeBody() {
  bodies = [];
  bodies.push(new Body(0, 0, 1, Math.PI, 1, 5));
  return bodies;
}

function createBodies() {
  bodies = createBodiesCrowded();
  // bodies = createBodiesThreeBodyProblem();
  // bodies = createBodiesDoubleSystem();
  // bodies = createBodiesQuartetSystem();
  // bodies = createBodiesFreeBody();
}

function drawBodies(ctx) {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  for (let idx = 0; idx < bodies.length; idx++) {
    bodies[idx].draw();
  }
}

function updateBodies(dt) {
  for (let idx = 0; idx < bodies.length; idx++) {
    bodies[idx].update(dt);
  }
}

function updateSystem() {
  for (let idx = 0; idx < bodies.length; idx++) {
    for (let jdx = 0; jdx < bodies.length; jdx++) {
      if (idx == jdx) continue;
      let b1 = bodies[idx];
      let b2 = bodies[jdx];
      let dist = Math.sqrt((b1.x - b2.x) ** 2 + (b1.y - b2.y) ** 2);
      let force = (G * b1.m * b2.m) / dist ** 2;

      var nx = (b2.x - b1.x) / dist;
      var ny = (b2.y - b1.y) / dist;
      b1.ax += (nx * force) / b1.m;
      b1.ay += (ny * force) / b1.m;
      // b2.ax += (nx * force) / b2.m;
      // b2.ay += (ny * force) / b2.m;
    }
  }
}

function init() {
  canvas = document.getElementById("white_square_canvas");
  ctx = canvas.getContext("2d");
  canvas.width = 600;
  canvas.height = 600;

  createBodies();

  setInterval(function () {
    drawBodies(ctx);
    updateSystem();
    updateBodies(0.1);
  }, 1);
}

init();
