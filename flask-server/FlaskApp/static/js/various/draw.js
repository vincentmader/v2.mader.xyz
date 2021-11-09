var drawing_radius = 10;

var mousedown = false;

var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");
var W = canvas.getBoundingClientRect().width;
var H = W;
canvas.width = W;
canvas.height = W;
ctx.fillStyle = "#000000";
ctx.fillRect(0, 0, canvas.width, canvas.height);
ctx.fillStyle = "#FFFFFF";

function draw_circle(x, y) {
  ctx.beginPath();
  ctx.arc(x, y, drawing_radius, 0, 2 * Math.PI);
  ctx.fill();
}

canvas.addEventListener("mousedown", (e) => {
  mousedown = true;
  var x = e.pageX;
  var y = e.pageY;
  var bounds = canvas.getBoundingClientRect();
  x -= bounds.left + window.scrollX;
  y -= bounds.top + window.scrollY;
  draw_circle(x, y);
});

canvas.addEventListener("mouseup", (e) => {
  mousedown = false;
});

canvas.addEventListener("mousemove", (e) => {
  var x = e.pageX;
  var y = e.pageY;
  var bounds = canvas.getBoundingClientRect();
  x -= bounds.left + window.scrollX;
  y -= bounds.top + window.scrollY;
  if (mousedown) {
    draw_circle(x, y);
  }
});
