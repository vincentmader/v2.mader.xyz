import { get_boltzmann_probability } from "./physics_utils.js";
import { apply_periodic_bounds } from "./physics_utils.js";

const line_width = 2;
const J = 1;
const mu = 1;
// const T = 3;

var canvas, ctx;
var W, H;
var paused = true;

function initialize_grid(N) {
  var grid, row, random_entry;

  grid = [];
  for (let i = 0; i < N; i++) {
    row = [];
    for (let j = 0; j < N; j++) {
      // random choice: -1 or +1
      // random_entry = [-1, +1][Math.round(Math.random())];
      // row.push(random_entry);
      row.push(-1);
    }
    grid.push(row);
  }
  return grid;
}

function draw_grid(grid) {
  var x, y, w, h, color;
  const N = grid.length;

  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      // get position and geometry of cell
      x = (W / N) * i;
      y = (H / N) * j;
      w = W / N;
      h = H / N;
      // get color for cell
      if (grid[i][j] == -1) {
        color = "black";
      } else if (grid[i][j] == +1) {
        color = "white";
      }
      // draw rect
      var z = 0.1;
      ctx.fillStyle = "#333333";
      ctx.fillRect(x - z * w, y - z * h, (1 + z) * w, (1 + z) * h);
      ctx.fillStyle = color;
      ctx.fillRect(x, y, w, h);
    }
  }
}

// function flip_spin(grid, i, j) {
//   const spin = grid[i][j];
//   if (spin == -1) {
//     grid[i][j] = +1;
//   } else if (spin == +1) {
//     grid[i][j] = -1;
//   }
//   return grid;
// }

// function flip_random_spin(grid) {
//   const N = grid.length;
//   // choose random grid cell
//   const i = Math.floor(N * Math.random());
//   const j = Math.floor(N * Math.random());
//   // flip if E_flip<0, else only with probability e^(-E_flip/kT)
//   const dE = get_flip_energy(grid, i, j);
//   if (dE <= 0) {
//     grid = flip_spin(grid, i, j);
//   } else if (Math.random() < get_boltzmann_probability(dE, T)) {
//     grid = flip_spin(grid, i, j);
//   }
//   return grid;
// }

function get_next_grid_state(N, grid) {
  var new_grid = [];
  var new_row = [];
  var entry, new_entry;
  var nr_of_neighbors, neighbor;
  var k_bc, l_bc;
  for (let i = 0; i < N; i++) {
    new_row = [];
    for (let j = 0; j < N; j++) {
      entry = grid[i][j];

      nr_of_neighbors = 0;
      // for (let k = Math.max(i - 1, 0); k <= Math.min(i + 1, N - 1); k++) {
      //   for (let l = Math.max(j - 1, 0); l <= Math.min(j + 1, N - 1); l++) {
      for (let k = i - 1; k <= i + 1; k++) {
        for (let l = j - 1; l <= j + 1; l++) {
          if (i == k && l == j) {
            continue;
          }
          if (k < 0) k_bc = k + N;
          else if (k >= N) k_bc = k - N;
          else k_bc = k;
          if (l < 0) l_bc = l + N;
          else if (l >= N) l_bc = l - N;
          else l_bc = l;

          neighbor = grid[k_bc][l_bc];
          // console.log(neighbor);
          if (neighbor == 1) {
            nr_of_neighbors += 1;
          }
        }
      }

      if (nr_of_neighbors < 2) {
        new_entry = -1;
      } else if (nr_of_neighbors == 2) {
        if (entry == +1) {
          new_entry = +1;
        } else {
          new_entry = -1;
        }
      } else if (nr_of_neighbors == 3) {
        new_entry = +1;
        console.log("aaaaa");
      } else if (3 < nr_of_neighbors) {
        new_entry = -1;
      }
      //     new_grid[i][j];
      new_row.push(new_entry);
    }
    new_grid.push(new_row);
  }
  return new_grid;
}

function flip_grid_entry(N, grid, x, y) {
  const i = Math.floor((x / W) * N);
  const j = Math.floor((y / H) * N);
  grid[i][j] *= -1;
  return grid;
}

function getCursorPosition(canvas, event) {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  // console.log("x: " + x + " y: " + y);
  return [x, y];
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

  document.getElementById("play/pause").addEventListener("click", function () {
    paused = !paused;
  });
  canvas.addEventListener("mousedown", function (e) {
    const pos = getCursorPosition(canvas, e);
    grid = flip_grid_entry(N, grid, pos[0], pos[1]);
  });

  const N = 20;
  // const flips_before_draw = 500;
  var grid = initialize_grid(N);
  // var temperature_slider, Bfield_slider;

  var frame_idx = 0;
  setInterval(function () {
    // temperature_slider = document.getElementById("button_pause");

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // for (let i = 0; i < flips_before_draw; i++) {
    //   grid = flip_random_spin(grid);
    // }
    console.log(frame_idx);
    draw_grid(grid);
    if (!paused) {
      grid = get_next_grid_state(N, grid);
      frame_idx += 1;
    }
    if (paused) document.getElementById("play/pause").innerHTML = "Unpause";
    if (!paused) document.getElementById("play/pause").innerHTML = "Pause";
  }, 150);
};

init();
