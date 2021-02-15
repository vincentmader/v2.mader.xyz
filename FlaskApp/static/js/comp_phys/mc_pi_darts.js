import { draw_point } from "../utils/drawing_utils.js";

const line_width = 2;

var canvas, ctx;
var W, H;

const nr_of_darts = 1000000000

const init = () => {
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;

  ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";

  // slider_a_x.setAttribute('onchange', () => {get_slider_values()})

  // get_slider_values()

  var x, y, r;
  var points = [];
  var dart_hits = 0
  var pi;

  var frame_idx = 0;
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    ctx.strokeStyle = "white";
    ctx.beginPath();
    ctx.arc(W / 2, H / 2, W / 2, 0, 2 * Math.PI);
    ctx.stroke();

    if (frame_idx < nr_of_darts) {
      x = 2 * Math.random() - 1;
      y = 2 * Math.random() - 1;
      r = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2))
      if (r <= 1) {
        dart_hits += 1
      }
      points.push([x, y]);
    }
    for (const p of points) {
      x = p[0];
      y = p[1];
      r = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2))
      if (r <= 1) {
        draw_point(ctx, W/2*(1+x), H/2*(1+y), 1, "green");
      } else {
        draw_point(ctx, W/2*(1+x), H/2*(1+y), 1, "white");
      }
    }

    pi = 4 * dart_hits / Math.min(frame_idx + 1, nr_of_darts)

    document.getElementById("textfield_pi").innerHTML = "Pi ~= " + pi.toFixed(2)

    frame_idx += 1;
  }, 20);
};

init();
