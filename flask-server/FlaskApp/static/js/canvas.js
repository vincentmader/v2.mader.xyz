const canvas = document.querySelector("canvas");
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
    x: Math.random() - 0.5, // Random x value from -0.5 to 0.5
    y: Math.random() - 0.5, // Random y value from -0.5 to 0.5
  };
}
Circle.prototype.draw = function () {
  c.beginPath();
  c.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
  c.fillStyle = this.color;
  c.fill();
  c.closePath();
};
/* Continued on next page... */

Object.prototype.update = function () {
  this.draw();
  this.x += this.velocity.x; // Move x coordinate
  this.y += this.velocity.y; // Move y coordinate
};
// Implementation
let circles;
function init() {
  for (let i = 0; i < 800; i++) {
    const x = Math.random() * canvas.width;
    const y = Math.random() * canvas.height;
    const radius = Math.random() * 5;
    const color = "blue";
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
