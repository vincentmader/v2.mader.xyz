// get canvas from html
const canvas = document.getElementById("square_canvas");
const c = canvas.getContext("2d");
// canvas.width = window.innerWidth;
// canvas.height = window.innerHeight;
width = canvas.parentElement.clientWidth;
// height = canvas.parentElement.clientHeight;
canvas.width = width;
// canvas.height = height;
canvas.height = width;

// simulation options
var gravity_switch = 1;
var paused = false;

// Event Listeners
// ============================================================================

// get position from mouse on move
const mouse = { x: undefined, y: undefined };
addEventListener("mousemove", (event) => {
  mouse.x = event.clientX;
  mouse.y = event.clientY;
});

// update canvas on window resize
addEventListener("resize", () => {
  // canvas.width = innerWidth;
  // canvas.height = innerHeight;
  canvas.width = width;
  // canvas.height = height;
  canvas.height = width;
  init();
});

// flip gravity on space key press
addEventListener("keydown", (e) => {
  if (e.key === " ") {
    gravity_switch *= -1;
  }
});

// TODO: pause simulation on space key press (change gravity to g key)
// addEventListener("keydown", (e) => {
//   if (e.key === " ") {
//     paused = !paused;
//   }
// });

// define circle object & methods (draw/update)
// ============================================================================

function Circle(x, y, radius, color) {
  this.x = x;
  this.y = y;
  this.radius = radius;
  this.color = color;
  this.velocity = {
    // Math.random() gives value from [-0.5, 0.5]
    x: -20 + 13 * Math.random(),
    y: -15 + 13 * Math.random(),
  };
}

// method for drawing circle
Circle.prototype.draw = function () {
  c.beginPath();
  c.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
  c.fillStyle = this.color;
  c.fill();
  c.closePath();
};

// method for updating state variables (pos/vel for next time step, dt=1)
Object.prototype.update = function () {
  this.velocity.y += gravity_switch * 0.5;
  this.draw();
  if (this.x <= 0 + this.radius) {
    this.velocity.x *= -0.5;
  }
  if (this.y <= 0 + this.radius) {
    // if (this.velocity.y > 0.1) {
    this.velocity.y *= -0.5;
    // } else this.velocity.y = 0;
  }
  if (this.x >= canvas.width - this.radius) {
    this.velocity.x *= -0.5;
  }
  if (this.y >= canvas.height - this.radius) {
    // if (this.velocity.y > 0.1) {
    this.velocity.y *= -0.5;
    // } else this.velocity.y = 0;
  }
  // TODO: implement elastic collisions with each other
  // for (let idx = 0; idx < circles.length; idx++) {
  //   let otherCircle = circles[idx];
  //   if (otherCircle == this) continue;
  //   d = Math.sqrt(
  //     (this.x - otherCircle.x) ** 2 + (this.y - otherCircle.y) ** 2
  //   );
  //   if (d < 0.8 * (this.radius + otherCircle.radius)) {
  //     this.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
  //     this.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
  //     otherCircle.velocity.x *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
  //     otherCircle.velocity.y *= -1; // * (1 + (Math.random() * 0.05 - 0.025));
  //   }
  // }
  // update position
  this.x += this.velocity.x; // Move x coordinate
  this.y += this.velocity.y; // Move y coordinate
};

// create list of circle object instances
// ============================================================================

// choose random ball color
function randomIntFromRange(from, to) {
  return from + Math.floor(Math.random() * (to - from));
}
// colors = ["white"];
colors = ["blue", "red", "green", "orange", "purple", "yellow", "cyan"];

// create list
let circles = [];
// function init() {
//   for (let i = 0; i < 5; i++) {
// const x = 0.9 * canvas.width - Math.random() * 10;
// const y = 0.9 * canvas.height - Math.random() * 10;
// const radius = 0.5 + Math.random() * 3;
// const color = colors[randomIntFromRange(0, colors.length)];
// circles.push(new Circle(x, y, radius, color));
// }
// }

// animation Loop
// ============================================================================

function animate() {
  requestAnimationFrame(animate); // Create an animation loop
  c.clearRect(0, 0, canvas.width, canvas.height); // Erase whole canvas

  const x = 0.98 * canvas.width - Math.random() * 10;
  const y = 0.5 * canvas.height - Math.random() * 10;
  const radius = 8.5 + Math.random() * 4;
  const color = colors[randomIntFromRange(0, colors.length)];

  circles.push(new Circle(x, y, radius, color));
  circles.forEach((circle) => {
    circle.update();
  });
  while (paused) {
    pass;
  }
}

// init();
animate();
