const canvas = document.getElementById("canvas2");
canvas.height = 400;
canvas.width = 400;
const ctx = canvas.getContext("2d");

const background = new Image();
background.src = "/static/media/black.png";
const img_sun = new Image();
img_sun.src = "/static/media/sun.png";
const img_earth = new Image();
img_earth.src = "/static/media/earth.png";

// window.addEventListener("keydown", function (e) {
//   if (e.key == " ") {
//   }
// });

const L = 1;
const g = 1;

const Sun = function () {
  this.x = 0;
  this.y = -200;
  this.v_x = 10;
  this.v_y = 0;
  this.width = 30;
  this.height = 30;
  this.applyGravity = () => {
    this.theta = Math.atan(-Math.abs(L - this.y) / this.x);
    this.alpha = -(g / L) * Math.sin(this.theta);
    this.a = this.alpha * L;
    console.log(this.theta);
    this.v_x += (this.a * Math.sin(this.theta)) / fps;
    this.v_y += (this.a * Math.cos(this.theta)) / fps;
  };
  this.move = () => {
    this.x += this.v_x / fps;
    this.y += this.v_y / fps;
  };
};
const Earth = function () {
  this.x = 0;
  this.y = 0;
  this.width = 30;
  this.height = 30;
};

function drawImage(img, x, y, W, H) {
  x += canvas.width / 2 - W / 2;
  ctx.drawImage(img, x, -y, W, H);
}

let fpsInterval, startTime, now, then, elapsed;
function startAnimating(fps) {
  fpsInterval = 1000 / fps;
  then = Date.now();
  startTime = then;
  earth = new Earth();
  sun = new Sun();
  animate();
}
function animate() {
  requestAnimationFrame(animate);
  now = Date.now();
  elapsed = now - then;
  if (elapsed > fpsInterval) {
    then = now - (elapsed % fpsInterval);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawImage(background, 0, 0, canvas.width, canvas.height);
    drawImage(img_earth, earth.x, earth.y, earth.width, earth.height);
    drawImage(img_sun, sun.x, sun.y, sun.width, sun.height);
    sun.applyGravity();
    sun.move();

    requestAnimationFrame(animate);
  }
}
let fps = 120;
startAnimating(fps);
