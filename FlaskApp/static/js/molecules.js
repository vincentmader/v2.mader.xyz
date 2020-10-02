const canvas = document.getElementById("square_canvas");
const c = canvas.getContext("2d");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const mouse = {
  x: undefined,
  y: undefined,
};

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
  this.color = color;
  this.velocity = {
    x: (Math.random() - 0.5) * 3, // Random x value from -0.5 to 0.5
    y: (Math.random() - 0.5) * 3, // Random y value from -0.5 to 0.5
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
  }
  if (this.y <= 0 + this.radius) {
    this.velocity.y *= -1;
  }
  if (this.x >= canvas.width - this.radius) {
    this.velocity.x *= -1;
  }
  if (this.y >= canvas.height - this.radius) {
    this.velocity.y *= -1;
  }
  for (let idx = 0; idx < circles.length; idx++) {
    let otherCircle = circles[idx];
    if (otherCircle == this) continue;
    d = Math.sqrt(
      (this.x - otherCircle.x) ** 2 + (this.y - otherCircle.y) ** 2
    );
    if (d < 0.8 * (this.radius + otherCircle.radius)) {
      this.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
      this.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
      otherCircle.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
      otherCircle.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
    }
  }
  this.x += this.velocity.x; // Move x coordinate
  this.y += this.velocity.y; // Move y coordinate
};

function randomIntFromRange(from, to) {
  return from + Math.floor(Math.random() * (to - from));
}
colors = ["blue", "green", "red", "yellow", "magenta", "orange", "white"];

// Implementation
let circles = [];
function init() {
  for (let i = 0; i < 1000; i++) {
    const x = canvas.width / 2 + (Math.random() - 0.5) * canvas.width * 0.99;
    const y = canvas.height / 2 + (Math.random() - 0.5) * canvas.height * 0.99;
    const radius = 0.5 + Math.random() * 3;
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
}

init();
animate();
