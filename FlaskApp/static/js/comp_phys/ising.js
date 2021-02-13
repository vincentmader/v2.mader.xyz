import { draw_line } from "../utils/drawing_utils.js";

const π = Math.PI;
const line_width = 2;
const J = 1,
  k_B = 1,
  T = 0.5;

var canvas, ctx;
var W, H, o_x, o_y;
var frame_idx;
var ys = [];
var L, r;

function initialize_spin_grid(N) {
  var grid, row, random_spin;

  grid = [];
  for (let i = 0; i < N; i++) {
    row = [];
    for (let j = 0; j < N; j++) {
      // random choice: -1 or +1
      random_spin = [-1, +1][Math.round(Math.random())];
      // add spin to grid
      row.push(random_spin);
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
      if (grid[i][j] == -1) {
        color = "blue";
      } else if (grid[i][j] == +1) {
        color = "red";
      }

      x = (W / N) * i;
      y = (H / N) * j;
      w = (W / N) * 0.8;
      h = (H / N) * 0.8;

      ctx.fillStyle = color;
      ctx.fillRect(x, y, w, h);
    }
  }
}

function apply_periodic_bounds(cell_idx, N) {
  if (cell_idx >= N) {
    cell_idx -= N;
  } else if (cell_idx < 0) {
    cell_idx += N;
  }
  return cell_idx;
}

function get_flip_energy(grid, i, j) {
  var i_neighbor, j_neighbor, s_neighbor;
  const N = grid.length;
  const s_flip = grid[i][j];
  var dE = 0;
  for (const di of [-1, 0, +1]) {
    for (const dj of [-1, 0, +1]) {
      if ((di == dj) == 0) {
        // get row/col index for neighbor
        i_neighbor = apply_periodic_bounds(i + di, N);
        j_neighbor = apply_periodic_bounds(j + dj, N);
        // get spin of neighbor
        s_neighbor = grid[i_neighbor][j_neighbor];
        // subtract current state's energy, add new state's energy
        dE -= -J * s_neighbor * s_flip;
        dE += -J * s_neighbor * -s_flip;
      }
    }
  }
  return dE;
}

function flip_spin(grid, i, j) {
  const spin = grid[i][j];
  if (spin == -1) {
    grid[i][j] = +1;
  } else if (spin == +1) {
    grid[i][j] = -1;
  }
  return grid;
}

function get_boltzmann_probability(dE, T) {
  const beta = 1 / (k_B * T);
  return Math.exp(-beta * dE);
}

function flip_random_spin(grid) {
  const N = grid.length;

  const i = Math.floor(N * Math.random());
  const j = Math.floor(N * Math.random());

  const spin = grid[i][j];

  const dE = get_flip_energy(grid, i, j);
  if (dE <= 0) {
    grid = flip_spin(grid, i, j);
  } else if (Math.random() < get_boltzmann_probability(dE, T)) {
    grid = flip_spin(grid, i, j);
  }

  return grid;
}

const init = () => {
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;

  o_x = W / 2;
  o_y = H / 2;

  ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";

  L = W / 4 - 10;
  r = W / 100;

  const N = 100;
  var grid = initialize_spin_grid(N);

  frame_idx = 0;
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < 500; i++) {
      grid = flip_random_spin(grid);
    }
    draw_grid(grid);
    frame_idx += 1;
  }, 1);
};

init();
