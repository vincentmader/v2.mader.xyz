const N = 100;
const threshold = 3;
const fps_goal = 30;

var paused = false; // changeable via button
var bool_apply_periodic_bounds = false; // changeable via button
var currently_selected_color = 1; // changeable via button

var canvas, ctx;
var W, H;
const colors = ["#333333", "white", "red"];
const line_width = 2;

function initialize_grid(N) {
  var grid, row;
  grid = [];
  for (let i = 0; i < N; i++) {
    row = [];
    for (let j = 0; j < N; j++) {
      row.push(-1);
    }
    grid.push(row);
  }
  return grid;
}

function draw_grid(grid) {
  var x, y, w, h, color, cell_value;
  const N = grid.length;

  w = (W / N) * 1.2;
  h = (H / N) * 1.2;
  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      // get position and geometry of cell
      x = (W / N) * i;
      y = (H / N) * j;
      // get color for cell
      cell_value = grid[i][j];
      if (cell_value == -1) {
        color = "black";
      } else color = colors[cell_value];
      // draw rect
      ctx.fillStyle = color;
      ctx.fillRect(x, y, w, h);
    }
  }
}

function get_next_grid_state(N, grid) {
  var new_grid = [];
  var new_row = [];
  var entry, new_entry;
  var neighbors, neighbor, k_bc, l_bc;
  var counts, add_neighbor;

  for (let i = 0; i < N; i++) {
    new_row = [];
    for (let j = 0; j < N; j++) {
      entry = grid[i][j];

      neighbors = [];
      for (let k = i - 1; k <= i + 1; k++) {
        for (let l = j - 1; l <= j + 1; l++) {
          if (i == k && l == j) continue; // no self-interaction
          // apply periodic bounds
          if (bool_apply_periodic_bounds) {
            if (k < 0) k_bc = k + N;
            else if (k >= N) k_bc = k - N;
            else k_bc = k;
            if (l < 0) l_bc = l + N;
            else if (l >= N) l_bc = l - N;
            else l_bc = l;
            neighbors.push(grid[k_bc][l_bc]);
          } else {
            if (k < 0 || k >= N || l < 0 || l >= N) continue;
            neighbors.push(grid[k][l]);
          }
        }
      }

      counts = {};
      for (const n of neighbors) {
        if (n == -1) continue;
        if (n in counts) {
          counts[n] += 1;
        } else counts[n] = 1;
      }

      new_entry = entry;
      for (const c in counts) {
        if (c == entry) continue;
        if (counts[c] < threshold) continue;
        if (c > entry && !(entry == 0 && c == 2)) {
          new_entry = c;
        } else if (c == 0 && entry == 2) {
          new_entry = c;
        }
      }
      new_row.push(new_entry);
    }
    new_grid.push(new_row);
  }
  return new_grid;
}

function flip_grid_entry(N, grid, x, y) {
  const i = Math.floor((x / W) * N);
  const j = Math.floor((y / H) * N);
  var draw_radius = 2;
  for (let k = i - draw_radius; k < i + draw_radius; k++) {
    for (let l = j - draw_radius; l < j + draw_radius; l++) {
      grid[k][l] = currently_selected_color;
    }
  }
  return grid;
}

function getCursorPosition(canvas, event) {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  return [x, y];
}

const init = () => {
  // canvas setup
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");
  ctx.lineWidth = line_width;
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";

  // buttons & events
  document.getElementById("play/pause").addEventListener("click", function () {
    paused = !paused;
  });
  document.getElementById("button_0").addEventListener("click", function () {
    currently_selected_color = 0;
  });
  document.getElementById("button_1").addEventListener("click", function () {
    currently_selected_color = 1;
  });
  document.getElementById("button_2").addEventListener("click", function () {
    currently_selected_color = 2;
  });
  canvas.addEventListener("mousedown", function (e) {
    const pos = getCursorPosition(canvas, e);
    grid = flip_grid_entry(N, grid, pos[0], pos[1]);
  });
  document
    .getElementById("button_toggle_periodic_bounds")
    .addEventListener("click", function () {
      bool_apply_periodic_bounds = !bool_apply_periodic_bounds;
    });

  var grid = initialize_grid(N);

  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw_grid(grid);
    if (!paused) {
      grid = get_next_grid_state(N, grid);
    }
    if (paused) document.getElementById("play/pause").innerHTML = "unpause";
    if (!paused) document.getElementById("play/pause").innerHTML = "pause";
  }, 1000 / fps_goal);
};

init();
