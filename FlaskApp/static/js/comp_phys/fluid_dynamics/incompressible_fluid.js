// CONSTANTS

const TAU = 2 * Math.PI;
const DT = 1;
const fps_goal = 60;

// // PARAMETERS

const k = 0.001;
var N = 25;
const nr_of_GS_iterations = 5;

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
    this.v_x = [];
    this.v_y = [];
    this.density = [];

    for (let y = 0; y < N; y++) {
      this.density[y] = [];
      this.v_x[y] = [];
      this.v_y[y] = [];
      for (let x = 0; x < N; x++) {
        if (Math.random() < 0.2) {
          this.density[y][x] = 1;
        } else {
          this.density[y][x] = 0;
        }
        this.v_x[y][x] = Math.random();
        this.v_y[y][x] = Math.random();
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
  // advect() {
  //   // var v_x_avg = [];
  //   // var v_y_avg = [];
  //   // for (let y = 0; y < N; y++) {
  //   //   v_x_avg[y] = [];
  //   //   v_y_avg[y] = [];
  //   //   for (let x = 0; x < N; x++) {
  //   //     v_x_avg[y][x] = this.get_avg_val_from_neighbors(this.v_x, x, y);
  //   //     v_y_avg[y][x] = this.get_avg_val_from_neighbors(this.v_y, x, y);
  //   //   }
  //   // }
  //   for (let idx = 0; idx < nr_of_GS_iterations; idx++) {
  //     for (let y = 0; y < N; y++) {
  //       for (let x = 0; x < N; x++) {
  //         let d_c1 = this.v_x[y][x];
  //         let d_c2 = this.v_y[y][x];

  //         let s_n1 = this.get_avg_val_from_neighbors(this.v_x, x, y);
  //         let d_n1 = (d_c1 + k * s_n1) / (1 + k);
  //         let s_n2 = this.get_avg_val_from_neighbors(this.v_y, x, y);
  //         let d_n2 = (d_c2 + k * s_n2) / (1 + k);

  //         this.v_x[y][x] = d_n1;
  //         this.v_y[y][x] = d_n2;
  //       }
  //     }
  //   }
  // }
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
  move() {
    let new_densities = [];
    for (let y = 0; y < N; y++) {
      new_densities[y] = [];
      for (let x = 0; x < N; x++) {
        let x_p = x - this.v_x[y][x] * DT;
        let y_p = y - this.v_y[y][x] * DT;

        let x_floor = Math.floor(x_p);
        let y_floor = Math.floor(y_p);
        let x_fract = x_p % 1;
        let y_fract = y_p % 1;

        function apply_bounds(x) {
          if (x < 0) {
            x += N;
          } else if (x >= N) {
            x -= N;
          }
          return x;
        }

        // if (x_floor >= N - 1) x_floor -= N;
        // if (y_floor >= N - 1) y_floor -= N;
        // if (x_floor < 0) x_floor += N;
        // if (y_floor < 0) y_floor += N;

        let z_1 = linear_interpolate(
          this.density[apply_bounds(y_floor)][apply_bounds(x_floor)],
          this.density[apply_bounds(y_floor)][apply_bounds(x_floor + 1)],
          x_fract
        );
        let z_2 = linear_interpolate(
          this.density[apply_bounds(y_floor + 1)][apply_bounds(x_floor)],
          this.density[apply_bounds(y_floor + 1)][apply_bounds(x_floor + 1)],
          x_fract
        );
        let d_n = linear_interpolate(z_1, z_2, y_fract);
        new_densities[y].push(d_n);
      }
    }
    this.density = new_densities;
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
  draw_velocity() {
    for (let y = 0; y < N; y++) {
      for (let x = 0; x < N; x++) {
        let v_x = this.v_x[y][x];
        let v_y = this.v_y[y][x];
        let theta = Math.atan2(v_y, v_x);
        let x_ctx_c = ((x + 0.5) * W) / N;
        let y_ctx_c = ((y + 0.5) * H) / N;
        let x_ctx_n = x_ctx_c + (0.5 * Math.cos(theta) * W) / N;
        let y_ctx_n = y_ctx_c + (0.5 * Math.sin(theta) * H) / N;
        ctx.strokeStyle = "red";
        ctx.beginPath();
        ctx.moveTo(x_ctx_c, y_ctx_c);
        ctx.lineTo(x_ctx_n, y_ctx_n);
        ctx.stroke();
      }
    }
  }
  update() {
    this.draw_density();
    if (N < 30) this.draw_velocity();
    this.diffuse();
    // this.advect();
    this.move();
  }
}

function linear_interpolate(a, b, k) {
  return a + k * (b - a);
}

// class Fluid {
//   constructor() {
//     this.size = N;
//     this.diff = diffusion;
//     this.visc = viscosity;

//     this.density = [];
//     this.v_x = [];
//     this.v_y = [];
//     for (let y = 0; y < this.size; y++) {
//       this.density[y] = [];
//       this.v_x[y] = [];
//       this.v_y[y] = [];
//       for (let x = 0; x < this.size; x++) {
//         this.density[y][x] = 0;
//         this.v_x[y][x] = 0;
//         this.v_y[y][x] = 0;
//       }
//     }
//   }
//   add_dye_density(x, y, amount) {
//     this.density[y][x] += amount;
//   }
//   add_velocity(x, y, amount_x, amount_y) {
//     this.v_x[y][x] += amount_x;
//     this.v_y[y][x] += amount_y;
//   }
//   diffuse() {}
//   project() {}
//   advect() {}
//   update() {
//     let N = this.size;
//     let visc = this.visc;
//     let diff = this.diff;
//     let v_x = this.v_x;
//     let v_y = this.v_y;
//     let v_x0 = this.v_x0;
//     let v_y0 = this.v_y0;
//     let s = this.s;
//     let dens = this.density;

//     this.diffuse(1, v_x0, v_x, visc, DT, 4, N);
//     this.diffuse(2, v_y0, v_y, visc, DT, 4, N);

//     this.project(v_x0, v_y0, v_x, v_y, 4, N);

//     this.advect(1, v_x, v_x0, v_xy, DT, N);
//     this.advect(1, v_y, v_y0, v_xy, DT, N);

//     this.project(v_x, v_y, v_x0, v_y0, 4, N);

//     this.diffuse(0, s, dens, diff, DT, 4, N);
//     this.advect(0, dens, s, v_x, v_y, DT, N);
//   }
// }

// function create_chart() {
//   canvas2 = document.getElementById("canvas_chart");
//   ctx2 = canvas2.getContext("2d");
//   W2 = canvas2.getBoundingClientRect().width;
//   canvas2.height = W2 / 2;

//   chart = new Chart(ctx2, {
//     type: "line",
//     data: {
//       datasets: [
//         {
//           borderColor: "white",
//           pointRadius: 0,
//           data: [],
//           showLine: true, // overrides the `line` dataset default
//           label: "total energy error (%)",
//         },
//       ],
//     },
//     options: {
//       // scales: {
//       //   yAxes: [
//       //     {
//       //       display: true,
//       //       ticks: {
//       // suggestedMax: 1,
//       // suggestedMin: -1,
//       // beginAtZero: true   // minimum value will be 0.
//       // type: "logarithmic",
//       // },
//       // },
//       // ],
//       // },
//     },
//   });
// }

function setup_event_listeners() {
  // BUTTONS
  let button_reset = document.getElementById("button_reset");
  button_reset.addEventListener("click", () => {
    init();
  });
  let button_toggle_draw_dens = document.getElementById(
    "button_toggle_draw_dens"
  );
  button_toggle_draw_dens.addEventListener("click", () => {
    bool_draw_density = !bool_draw_density;
  });
  let button_toggle_draw_vel = document.getElementById(
    "button_toggle_draw_vel"
  );
  button_toggle_draw_vel.addEventListener("click", () => {
    bool_draw_velocity = !bool_draw_velocity;
  });
  // SLIDERS
  document.getElementById("slider_N").addEventListener("click", function () {
    let value = document.getElementById("slider_N").value;
    N = value;
    init();
  });

  // document.getElementById("slider_v0").addEventListener("click", function () {
  //   let value = document.getElementById("slider_v0").value;
  //   v0 = Number(value);
  //   console.log("new v_0: ", v0);
  //   init();
  // });
  // document.getElementById("slider_v0").value = v0;
  // document.getElementById("v0_display").innerHTML =
  //   "v0 = " + String(v0.toFixed(1));
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

    // chart.data.labels.push(""); // TODO: ?
    // chart.data.datasets[0].data.push(100 * (e / energy_0 - 1));
    // chart.update();
  }, 1000 / fps_goal);
}

init();
