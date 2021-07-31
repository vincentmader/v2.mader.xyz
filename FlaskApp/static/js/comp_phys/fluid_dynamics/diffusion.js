// CONSTANTS

const TAU = 2 * Math.PI;
const DT = 1;
const fps_goal = 60;

// // PARAMETERS

const k = 0.01;
var N = 25;
const nr_of_GS_iterations = 10;

// SETTINGS

var bool_draw_velocity = true;
var bool_draw_density = true;

// // OTHER VARIABLES

var fluid;
var canvas, ctx, W, H, o_x, o_y;
// var canvas2, ctx2, W2, H2, chart;

// CLASSES

class Fluid {
  constructor() {
    this.density = [];

    for (let y = 0; y < N; y++) {
      this.density[y] = [];
      for (let x = 0; x < N; x++) {
        if (Math.random() < 0.2) {
          this.density[y][x] = 1;
        } else {
          this.density[y][x] = 0;
        }
      }
    }
  }
  get_avg_val_from_neighbors(arr, x, y) {
    var up, left, down, right, avg;
    var nr_of_neighbors = 4;
    // left
    if (x - 1 < 0) {
      left = 0;
      nr_of_neighbors -= 1;
    } else {
      left = arr[y][x - 1];
    } // right
    if (x + 1 > N - 1) {
      right = 0;
      nr_of_neighbors -= 1;
    } else {
      right = arr[y][x + 1];
    } // down
    if (y - 1 < 0) {
      down = 0;
      nr_of_neighbors -= 1;
    } else {
      down = arr[y - 1][x];
    } // up
    if (y + 1 > N - 1) {
      up = 0;
      nr_of_neighbors -= 1;
    } else {
      up = arr[y + 1][x];
    }
    avg = (up + left + down + right) / nr_of_neighbors;
    return avg;
  }
  diffuse() {
    // var avg = [];
    // for (let y = 0; y < N; y++) {
    //   avg[y] = [];
    //   for (let x = 0; x < N; x++) {
    //     avg[y][x] = this.get_avg_val_from_neighbors(this.density, x, y);
    //   }
    // }
    for (let idx = 0; idx < nr_of_GS_iterations; idx++) {
      for (let y = 0; y < N; y++) {
        for (let x = 0; x < N; x++) {
          let d_c = this.density[y][x]; // s_c = dens_avg[y][x];
          let s_n = this.get_avg_val_from_neighbors(this.density, x, y);
          this.density[y][x] = (d_c + k * s_n) / (1 + k);
        }
      }
    }
  }
  draw_density() {
    let w = W / N;
    let h = H / N;
    let z = 1;
    for (let y = 0; y < N; y++) {
      for (let x = 0; x < N; x++) {
        let alpha = this.density[y][x]; // TODO: normalize to [0,1]?
        ctx.fillStyle = "rgba(255, 255, 255, " + alpha + ")";
        ctx.fillRect(x * w, y * h, z * w, z * h);
      }
    }
  }
  update() {
    this.draw_density();
    this.diffuse();
  }
}

function setup_event_listeners() {
  // BUTTONS
  let button_reset = document.getElementById("button_reset");
  button_reset.addEventListener("click", () => {
    init();
  });
  // SLIDERS
  document.getElementById("slider_N").addEventListener("click", function () {
    let value = document.getElementById("slider_N").value;
    N = value;
    init();
  });
}

function init() {
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");
  o_x = W / 2;
  o_y = H / 2;

  fluid = new Fluid();

  setup_event_listeners();
  animate();
}

function animate() {
  setInterval(function () {
    // clear screen
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // update fluid cells
    fluid.update();
  }, 1000 / fps_goal);
}

init();
