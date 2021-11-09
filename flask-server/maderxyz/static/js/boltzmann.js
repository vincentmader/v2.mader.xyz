const canvas = document.getElementById("square_canvas");
const c = canvas.getContext("2d");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const mouse = {
  x: undefined,
  y: undefined,
};
const particle_number = 1000;
var t = 0;

function dot(v1, v2) {
  return v1[0] * v1[1] + v1[1] * v2[1];
}
let transferred_momentum = 0;

function get_pressure_from_energy(arr) {
  let pressure = 0;
  for (let idx = 0; idx < arr.length; idx++) {
    let circle = arr[idx];
    let v = [circle.velocity.x, circle.velocity.y];
    let energy = (circle.mass / 2) * dot(v, v);
    pressure += energy;
  }
  return Math.floor((pressure / (canvas.width * canvas.height)) * 1000000);
}
function get_pressure_from_wall_collisions() {
  return Math.floor((transferred_momentum / 4 / t / canvas.width) * 1000000);
}

// Event Listeners
addEventListener("mousemove", (event) => {
  mouse.x = event.clientX;
  mouse.y = event.clientY;
});
addEventListener("resize", () => {
  canvas.width = innerWidth;
  canvas.height = innerHeight;
  init();
});
// Objects
function Circle(x, y, radius, color) {
  this.x = x;
  this.y = y;
  this.radius = radius;
  this.mass = this.radius ** 3;
  this.color = color;
  this.velocity = {
    x: (Math.random() - 0.5) * 15, // Random x value from -0.5 to 0.5
    y: (Math.random() - 0.5) * 15, // Random y value from -0.5 to 0.5
  };
}
Circle.prototype.draw = function () {
  c.beginPath();
  c.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
  c.fillStyle = this.color;
  c.fill();
  c.closePath();
};
Object.prototype.update = function () {
  this.draw();
  if (this.x <= 0 + this.radius) {
    this.velocity.x *= -1;
    transferred_momentum += Math.abs(2 * this.mass * this.velocity.x);
  }
  if (this.y <= 0 + this.radius) {
    this.velocity.y *= -1;
    transferred_momentum += Math.abs(2 * this.mass * this.velocity.y);
  }
  if (this.x >= canvas.width - this.radius) {
    this.velocity.x *= -1;
    transferred_momentum += Math.abs(2 * this.mass * this.velocity.x);
  }
  if (this.y >= canvas.height - this.radius) {
    this.velocity.y *= -1;
    transferred_momentum += Math.abs(2 * this.mass * this.velocity.y);
  }
  for (let idx = 0; idx < circles.length; idx++) {
    let otherCircle = circles[idx];
    if (otherCircle == this) continue;
    // d = Math.sqrt(
    //   (this.x - otherCircle.x) ** 2 + (this.y - otherCircle.y) ** 2
    // );
    // if (d < 0.8 * (this.radius + otherCircle.radius)) {
    //   // let mu = this.mass / otherCircle.mass;
    //   // let v1 = [this.velocity.x, this.velocity.y];
    //   // let v2 = [otherCircle.velocity.x, otherCircle.velocity.y];
    //   // let A = 1 - mu;
    //   // let B = [
    //   //   2 * (mu * this.velocity.x - otherCircle.velocity.x),
    //   //   2 * (mu * this.velocity.y - otherCircle.velocity.y),
    //   // ];
    //   // let C = (mu - 1) * dot(v1, v1) + 2 * dot(v1, v2);
    //   // let X1 = -B[0] + Math.sqrt(dot(B, B) - 4 * A * C) / (2 * A);
    //   // let X2 = -B[1] + Math.sqrt(dot(B, B) - 4 * A * C) / (2 * A);
    //   // let X = [X1, X2];
    //   // let Y1 = dot(v2, v2) + mu * (dot(v1, v1) - dot(X, X));
    //   // let Y2 = dot(v2, v2) + mu * (dot(v1, v1) - dot(X, X));
    //   // let Y = [Y1, Y2];
    //   // let a = Math.sqrt(Y);

    //   // console.log(X);
    //   // console.log(Math.sqrt(X) * (-this.velocity.y / dot(v1, v1));)
    //   // this.velocity.x = Math.sqrt(X) * (-this.velocity.y / dot(v1, v1));
    //   // this.velocity.y = Math.sqrt(X) * (-this.velocity.x / dot(v1, v1));
    //   // otherCircle.velocity.x = Math.sqrt(Y) * (-this.velocity.y / dot(v1, v1));
    //   // otherCircle.velocity.y = Math.sqrt(Y) * (-this.velocity.x / dot(v2, v2));
    //   this.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
    //   this.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
    //   otherCircle.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
    //   otherCircle.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
    // }
  }
  this.x += this.velocity.x; // Move x coordinate
  this.y += this.velocity.y; // Move y coordinate
};

function randomIntFromRange(from, to) {
  return from + Math.floor(Math.random() * (to - from));
}
colors = [
  // "blue",
  // "green",
  "red",
  // "yellow",
  // "magenta",
  // "orange",
  // "white",
  // "cyan",
];

// Implementation
let circles = [];
function init() {
  for (let i = 0; i < particle_number; i++) {
    const x = canvas.width / 2 + (Math.random() - 0.5) * canvas.width * 0.99;
    const y = canvas.height / 2 + (Math.random() - 0.5) * canvas.height * 0.99;
    const radius = 2; // 1.5 + Math.random() * 2;
    const color = colors[randomIntFromRange(0, colors.length)];
    circles.push(new Circle(x, y, radius, color));
  }
}

// Animation Loop
function animate() {
  requestAnimationFrame(animate); // Create an animation loop
  c.clearRect(0, 0, canvas.width, canvas.height); // Erase whole canvas
  circles.forEach((circle) => {
    circle.update();
  });
  t += 1;
  let text = String(get_pressure_from_energy(circles)) + "   ";
  text += String(get_pressure_from_wall_collisions());
  document.getElementById("infoBox").textContent = text;
  // let ay = document.getElementById("infoBox");
  // ay.innerHTML = text;
}

init();
animate();
