import {draw_point} from '../utils/drawing_utils.js';

const dt  = 0.003
const line_width = 2;

var canvas, ctx;
var W, H;
var a_x, a_y, w_x, w_y, phi, tail_length

var slider_a_x = document.getElementById("slider_a_x")
var slider_a_y = document.getElementById("slider_a_y")
var slider_w_x = document.getElementById("slider_w_x")
var slider_w_y = document.getElementById("slider_w_y")
var slider_phi = document.getElementById("slider_phi")
var slider_tail_length = document.getElementById("slider_tail_length")


const get_slider_values = () => {
    a_x = slider_a_x.value / 1000;
    a_y = slider_a_y.value / 1000;
    w_x = slider_w_x.value;
    w_y = slider_w_y.value;
    phi = slider_phi.value / 1000;
    tail_length = slider_tail_length.value;
}


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

  var x,y
  var x_canvas, y_canvas
  var t = 0
  setInterval(function () {

    get_slider_values()


    document.getElementById("display_a_x").innerHTML = "a_x = " + a_x
    document.getElementById("display_a_y").innerHTML = "a_y = " + a_y
    document.getElementById("display_w_x").innerHTML = "w_x = " + w_x
    document.getElementById("display_w_y").innerHTML = "w_y = " + w_y
    document.getElementById("display_phi").innerHTML = "phi = " + phi
    document.getElementById("display_tail_length").innerHTML = "L = " + tail_length
    //   // "$$T=" +
    //   "T = " + T.toFixed(3);
    // + "$$";
    // MathJax.Hub.Queue(["Typeset", MathJax.Hub, temperature_slider]);

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // x = a_x * Math.cos(w_x * t + phi)
    // y = a_y * Math.cos(w_y * t)
    // past_positions.push([x, y])

    for (let i=0; i<tail_length; i++) {
      t = i * dt
      x = a_x * Math.cos(w_x * t + phi)
      y = a_y * Math.cos(w_y * t)
      // past_positions.push([x, y])
      // x = past_positions[i][0]
      // y = past_positions[i][1]
      x_canvas = 40*x + W/2
      y_canvas = 40*y + H/2
      draw_point(ctx, x_canvas, y_canvas, 1, 'red')

      // if (past_positions.length > tail_length) {
      //   past_positions.shift()
      // }
    }



    t += dt
  }, 1);
};

init();
