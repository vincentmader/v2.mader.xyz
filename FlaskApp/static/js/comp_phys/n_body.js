import { draw_line } from "./a.js";
import { draw_point } from "./a.js";

// const line_width = 2;
// // const tail_length = 150;
// // var circle_radius;

var canvas, ctx;
var W, H;

var o_x, o_y;
var frame_idx;

// // var L;

var data = $("#n_body_canvas").data("ys");
var system_state;
var foo;
// // var foo, x_1, y_1, x_2, y_2;
// // var phi_1, phi_2;
// // var phi_1p, phi_2p, phi_1c, phi_2c;
// // var x_1p, y_1p, x_2p, y_2p, x_1c, y_1c, x_2c, y_2c;

// // function get_positions_from_angles(phi_1, phi_2) {
// //   let x_1 = o_x + L * Math.sin(phi_1); // cart. coords of pendulum mass
// //   let y_1 = o_y + L * Math.cos(phi_1);
// //   let x_2 = x_1 + L * Math.sin(phi_2); // cart. coords of pendulum mass
// //   let y_2 = y_1 + L * Math.cos(phi_2);

// //   return [x_1, y_1, x_2, y_2];
// // }

// // function draw_tails(ctx, frame_idx, tail_length) {
// //   var current_system_state, previous_system_state;

// //   for (const idx of Array(tail_length).keys()) {
// //     current_system_state = data[Math.max(0, frame_idx - idx)];
// //     previous_system_state = data[Math.max(0, frame_idx - idx - 1)];

// //     phi_1p = previous_system_state[0];
// //     phi_2p = previous_system_state[1];
// //     foo = get_positions_from_angles(phi_1p, phi_2p);
// //     x_1p = foo[0];
// //     y_1p = foo[1];
// // x_2p = foo[2];
// // y_2p = foo[3];

// // phi_1c = current_system_state[0];
// // phi_2c = current_system_state[1];
// // foo = get_positions_from_angles(phi_1c, phi_2c);
// // x_1c = foo[0];
// // y_1c = foo[1];
// // x_2c = foo[2];
// // y_2c = foo[3];

// // draw_line(ctx, x_1p, y_1p, x_1c, y_1c, "green");
// // draw_line(ctx, x_2p, y_2p, x_2c, y_2c, "red");
// // }
// // }

// var bodies = [];
// let G = 1000;

// const draw = function () {
//   ctx.beginPath();
//   ctx.strokeStyle = "black";
//   ctx.fillStyle = "black";
//   ctx.arc(
//     this.x + canvas.width / 2,
//     -this.y + canvas.height / 2,
//     this.radius,
//     1,
//     360
//   );
//   ctx.stroke();
//   ctx.fill();

//   if (this.tail) {
//     // console.log(this.tail);
//     this.tail.draw();
//   }
// };

// function drawBodies(ctx, frame_idx) {
//   ctx.clearRect(0, 0, canvas.width, canvas.height);
//   system_state = data[frame_idx];
// const nr_of_bodies = system_state.length;
//   console.log(nr_of_bodies);
// var m, R, x, y, u, v;
// for (let idx = 0; idx < nr_of_bodies; idx++) {
//   print(idx);
//   //     m = system_state[8 * idx];
//   //     R = system_state[8 * idx + 1];
//   //     x = system_state[8 * idx + 2];
//   //     y = system_state[8 * idx + 3];
//   //     u = system_state[8 * idx + 4];
//   //     v = system_state[8 * idx + 5];
//   //     console.log(x);
//   //     // bodies[idx].draw();
//   //     // console.log(system_state);
// }
// }
//
//
function xy_to_canvas_coords(x, y, W, H, zoom_level) {
  var xlim;
  var ylim;

  var canvas_x;
  var canvas_y;

  x *= zoom_level;
  y *= zoom_level;
  x += o_x;
  y += o_y;

  return [x, y];
}

function init() {
  console.log("hello!");
  // canvas setup
  canvas = document.getElementById("n_body_canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");

  // ctx.lineWidth = line_width;

  o_x = W / 2;
  o_y = H / 2;

  frame_idx = 0;
  setInterval(function () {
    //   ctx.clearRect(0, 0, canvas.width, canvas.height);

    //   // draw_tails(ctx, frame_idx);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    system_state = data[frame_idx];
    var nr_of_bodies = system_state.length / 8;
    var m, R, x, y, u, v;
    for (let idx = 0; idx < nr_of_bodies; idx++) {
      m = system_state[8 * idx];
      R = system_state[8 * idx + 1];
      x = system_state[8 * idx + 2];
      y = system_state[8 * idx + 3];
      u = system_state[8 * idx + 4];
      v = system_state[8 * idx + 5];
      console.log(x);
      var zoom_level = 70;
      foo = xy_to_canvas_coords(x, y, W, H, zoom_level);
      x = foo[0];
      y = foo[1];
      draw_point(ctx, x, y, 1);
    }

    frame_idx += 1;

    document.getElementById("restart").addEventListener("click", function () {
      frame_idx = 0;
    });
  }, 20);
}

init();
