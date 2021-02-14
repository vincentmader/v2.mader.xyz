import { get_boltzmann_probability } from "./physics_utils.js";
import { apply_periodic_bounds } from "./physics_utils.js";

const line_width = 2;
const J = 1;
// const T = 3;
var T;
var B;

var canvas, ctx;
var W, H;

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
      // get position and geometry of cell
      x = (W / N) * i;
      y = (H / N) * j;
      w = (W / N) * 0.7;
      h = (H / N) * 0.7;
      // get color for cell
      if (grid[i][j] == -1) {
        color = "black";
      } else if (grid[i][j] == +1) {
        color = "white";
      }
      ctx.fillStyle = color;
      // draw rect
      ctx.fillRect(x, y, w, h);
    }
  }
}

function get_flip_energy(grid, i, j) {
  const N = grid.length;
  const s_flip = grid[i][j];

  var i_neighbor, j_neighbor, s_neighbor;
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
  dE -= B * s_flip;
  dE += B * -s_flip;
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

function flip_random_spin(grid) {
  const N = grid.length;
  // choose random grid cell
  const i = Math.floor(N * Math.random());
  const j = Math.floor(N * Math.random());
  // flip if E_flip<0, else only with probability e^(-E_flip/kT)
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

  ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";

  const N = 100;
  const flips_before_draw = 500;
  var grid = initialize_spin_grid(N);
  var temperature_slider, Bfield_slider;

  setInterval(function () {
    temperature_slider = document.getElementById("temperature_slider");
    T = 10 ** Number(temperature_slider.value / 60);
    document.getElementById("temperature_display").innerHTML =
      // "$$T=" +
      "T = " + T.toFixed(3);
    // + "$$";
    // MathJax.Hub.Queue(["Typeset", MathJax.Hub, temperature_slider]);

    Bfield_slider = document.getElementById("Bfield_slider");
    B = Number(Bfield_slider.value);
    document.getElementById("Bfield_display").innerHTML =
      // "$$B=" +
      "B = " + B.toFixed(3);
    // + "$$";
    // MathJax.Hub.Queue(["Typeset", MathJax.Hub, "Bfield_slider"]);

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < flips_before_draw; i++) {
      grid = flip_random_spin(grid);
    }
    draw_grid(grid);
  }, 1);
};

init();