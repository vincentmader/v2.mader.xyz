// const canvas = document.getElementById("square_canvas");
// const ctx = canvas.getContext("2d");
// canvas.width = window.innerWidth;
// canvas.height = window.innerHeight;

// const particle_number = 1000;

const q = 1;
const m = 1;
const E = 6;
const B = 1;
const dt = 1;
var W, H, canvas, ctx;

// // // Objects
// function Circle(x, y, radius, color) {
//   this.x = x;
//   this.y = y;
//   this.vx = 1;
//   this.vy = 0;
//   this.radius = radius;
//   this.mass = this.radius ** 3;
//   this.color = color;
//   this.velocity = {
//     x: (Math.random() - 0.5) * 15, // Random x value from -0.5 to 0.5
//     y: (Math.random() - 0.5) * 15, // Random y value from -0.5 to 0.5
//   };
//   this.draw = function () {
//     ctx.beginPath();
//     ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
//     ctx.fillStyle = this.color;
//     ctx.fill();
//     ctx.closePath();
//   };
//   // this.updateVelocity = function () {
//   //   let ax = (q * (E - this.vx * B)) / m;
//   //   this.vx += ax * dt;
//   // };
//   // this.updatePosition = function () {
//   //   this.x += this.vx * dt;
//   //   this.y += this.vy * dt;
//   // };
//   this.update = function (dt) {
//     // this.updateVelocity();
//     // this.updatePosition();
//     this.draw();
//     // console.log("hey");
//   };
// }
// // Circle.prototype.draw = function () {
// //   c.beginPath();
// //   c.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
// //   c.fillStyle = this.color;
// //   c.fill();
// //   c.closePath();
// // };
// // Object.prototype.update = function () {
// //   this.draw();
// //   if (this.x <= 0 + this.radius) {
// //     this.velocity.x *= -1;
// //     transferred_momentum += Math.abs(2 * this.mass * this.velocity.x);
// //   }
// //   if (this.y <= 0 + this.radius) {
// //     this.velocity.y *= -1;
// //     transferred_momentum += Math.abs(2 * this.mass * this.velocity.y);
// //   }
// //   if (this.x >= canvas.width - this.radius) {
// //     this.velocity.x *= -1;
// //     transferred_momentum += Math.abs(2 * this.mass * this.velocity.x);
// //   }
// //   if (this.y >= canvas.height - this.radius) {
// //     this.velocity.y *= -1;
// //     transferred_momentum += Math.abs(2 * this.mass * this.velocity.y);
// //   }
// //   for (let idx = 0; idx < circles.length; idx++) {
// //     let otherCircle = circles[idx];
// //     if (otherCircle == this) continue;
// //     d = Math.sqrt(
// //       (this.x - otherCircle.x) ** 2 + (this.y - otherCircle.y) ** 2
// //     );
// //     if (d < 0.8 * (this.radius + otherCircle.radius)) {
// // let mu = this.mass / otherCircle.mass;
// // let v1 = [this.velocity.x, this.velocity.y];
// // let v2 = [otherCircle.velocity.x, otherCircle.velocity.y];
// // let A = 1 - mu;
// // let B = [
// //   //   2 * (mu * this.velocity.x - otherCircle.velocity.x),
// //   //   2 * (mu * this.velocity.y - otherCircle.velocity.y),
// //   // ];
// //   // let C = (mu - 1) * dot(v1, v1) + 2 * dot(v1, v2);
// //   // let X1 = -B[0] + Math.sqrt(dot(B, B) - 4 * A * C) / (2 * A);
// //   // let X2 = -B[1] + Math.sqrt(dot(B, B) - 4 * A * C) / (2 * A);
// //   // let X = [X1, X2];
// //   // let Y1 = dot(v2, v2) + mu * (dot(v1, v1) - dot(X, X));
// //   // let Y2 = dot(v2, v2) + mu * (dot(v1, v1) - dot(X, X));
// //   // let Y = [Y1, Y2];
// //   // let a = Math.sqrt(Y);

// //   // console.log(X);
// //   // console.log(Math.sqrt(X) * (-this.velocity.y / dot(v1, v1));)
// //   // this.velocity.x = Math.sqrt(X) * (-this.velocity.y / dot(v1, v1));
// //   // this.velocity.y = Math.sqrt(X) * (-this.velocity.x / dot(v1, v1));
// //   // otherCircle.velocity.x = Math.sqrt(Y) * (-this.velocity.y / dot(v1, v1));
// //   // otherCircle.velocity.y = Math.sqrt(Y) * (-this.velocity.x / dot(v2, v2));
// //   this.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
// //   this.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
// //   otherCircle.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
// //   otherCircle.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
// // }
// // }
// // this.x += this.velocity.x; // Move x coordinate
// // this.y += this.velocity.y; // Move y coordinate
// // };

// colors = ["blue"]; //, "green", "red", "yellow", "orange", "white", "cyan", ];
// function randomIntFromRange(from, to) {
//   return from + Math.floor(Math.random() * (to - from));
// }

// // Implementation
// let circles = [];
// function init() {
//   for (let i = 0; i < particle_number; i++) {
//     const x = canvas.width / 2;
//     const y = canvas.height;
//     const radius = 2; // 1.5 + Math.random() * 2;
//     const color = colors[randomIntFromRange(0, colors.length)];
//     circles.push(new Circle(x, y, radius, color));
//   }
// }

// // Animation Loop
// function animate() {
//   requestAnimationFrame(animate); // Create an animation loop
//   ctx.clearRect(0, 0, canvas.width, canvas.height); // Erase whole canvas
//   circles.forEach((circle) => {
//     circle.update();
//   });

// // init();
// // animate();

// const canvas = document.getElementById("square_canvas");
// const ctx = canvas.getContext("2d");
// canvas.width = window.innerWidth;
// canvas.height = window.innerHeight;

// // Objects
// function Circle(x, y, radius, color) {
//   this.x = x;
//   this.y = y;
//   this.radius = radius;
//   this.vx = 0;
//   this.vy = 0.001 + 0.001 * (2 * Math.random() - 1);
//   this.color = color;
//   this.updateVelocity = function () {
//     this.ay = -this.vx * B;
//     this.ax = (q * (E - this.vy * B)) / m;
//     this.vx += this.ax * dt;
//     this.vy += this.ay * dt;
//   };
//   this.updatePosition = function () {
//     this.x += this.vx * dt;
//     this.y += this.vy * dt;
//   };
//   this.draw = function () {
//     ctx.beginPath();
//     ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
//     ctx.fillStyle = this.color;
//     ctx.fill();
//     ctx.closePath();
//   };
// }
// Object.prototype.update = function () {
//   this.updateVelocity();
//   this.updatePosition();
//   this.draw();
// };

// colors = ["white"];
// function randomIntFromRange(from, to) {
//   return from + Math.floor(Math.random() * (to - from));
// }

// let circles = [];
// function animate() {
//   requestAnimationFrame(animate); // Create an animation loop
//   ctx.clearRect(0, 0, canvas.width, canvas.height); // Erase whole canvas

//   const x = canvas.width / 2;
//   const y = canvas.width;
//   const radius = 3;
//   const color = colors[randomIntFromRange(0, colors.length)];
//   circles.push(new Circle(x, y, radius, color));
//   circles.forEach((circle) => {
//     circle.update();
//   });
// }

// animate();

canvas = document.getElementById("canvas");
ctx = canvas.getContext("2d");
W = canvas.getBoundingClientRect().width;
H = W;
canvas.width = W;
canvas.height = W;

// function randomIntFromRange(from, to) {
//   return from + Math.floor(Math.random() * (to - from));
// }
// const velocities = [1.6, 1.1, 0.9, 0.8, 0.7, 0.6];
// Objects
function Circle(x, y, radius, color) {
  this.x = canvas.width / 2;
  this.y = canvas.height;
  this.radius = 1;
  this.vx = 0;
  // this.vy = -velocities[randomIntFromRange(0, velocities.length)];
  this.vy = -6 + 0.5 * (Math.random() - 0.5);
  this.draw = function () {
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
    ctx.fillStyle = this.color;
    ctx.fill();
    ctx.closePath();
  };
  this.color = "white";
  if (Math.abs(-this.vy - 6) <= 0.001) {
    this.color = "green";
  }
  if (-this.vy > 6.001) {
    this.color = "red";
  }
  if (-this.vy < 5.999) {
    this.color = "blue";
  }
  this.update = function () {
    this.draw();
    this.vx += ((q * (E + this.vy * B)) / m) * dt;
    this.x += this.vx * dt; // Move x coordinate
    this.y += this.vy * dt; // Move y coordinate
  };
}

let circles = [];
function animate() {
  requestAnimationFrame(animate); // Create an animation loop
  ctx.clearRect(0, 0, canvas.width, canvas.height); // Erase whole canvas

  for (let idx = 0; idx < 30; idx++) {
    circles.push(new Circle());
  }
  circles.forEach((circle) => {
    circle.update();
  });
}

animate();
