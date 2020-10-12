var canvas, ctx;
const π = Math.PI;
const dt = 1;
const L = 200;
const g = 1;
const R = 5;

const W = 600;
const H = 600;

function Pendulum(o_x, o_y, φ) {
  // set coords of pivot origin
  this.o_x = o_x;
  this.o_y = o_y;
  // set angular coords of pendulum mass
  this.φ = φ;
  this.ω = 0;
  this.α = 0;

  this.move = () => {
    this.α = (-g / L) * Math.sin(this.φ);
    this.ω += this.α * dt;
    this.φ += this.ω * dt;
  };

  this.draw = () => {
    let p_x = this.o_x + L * Math.sin(this.φ); // cart. coords of pendulum mass
    let p_y = this.o_y + L * Math.cos(this.φ);

    ctx.beginPath();
    ctx.strokeStyle = "white";
    ctx.fillStyle = "white";
    // draw mass
    ctx.arc(p_x + W / 2, p_y + H / 2, R, 0, 2 * Math.PI);
    ctx.stroke();
    ctx.fill();
    // draw rod
    ctx.moveTo(this.o_x + W / 2, this.o_y + H / 2);
    ctx.lineTo(p_x + W / 2, p_y + H / 2);
    ctx.stroke();
  };
}

const init = () => {
  canvas = document.getElementById("square_canvas");
  ctx = canvas.getContext("2d");
  canvas.width = W;
  canvas.height = H;

  p = new Pendulum(0, 0, π / 3);
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    p.draw();
    p.move();
  }, 20);
};

init();
