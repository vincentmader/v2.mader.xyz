const line_width = 2;
const FPS_GOAL = 30;

var canvas, ctx;
var W, H;
var paused = false;
var currently_selected_color = 3;

function initialize_grid(N) {
  var grid, row, random_entry;
  grid = [];
  for (let i = 0; i < N; i++) {
    row = [];
    for (let j = 0; j < N; j++) {
      // if (Math.abs(j - N / 2) < 10) {
      //   if (Math.abs(i - N / 2) < 4) row.push(1);
      //   else row.push(4);
      // } else if (Math.abs(j - N / 2) < 30 * (Math.random() - 0.5)) {
      //   row.push(4);
      // } else row.push(1);
      row.push(1);
    }
    grid.push(row);
  }
  return grid;
}

function draw_grid(grid) {
  var x, y, w, h, color, cell_value;
  const N = grid.length;

  w = (W / N) * 0.9;
  h = (H / N) * 0.9;
  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      // get position and geometry of cell
      x = (W / N) * i;
      y = (H / N) * j;
      // get color for cell
      cell_value = grid[i][j];
      if (cell_value == 1) {
        color = "#004400";
      } else if (cell_value == 2) {
        color = "blue";
      } else if (cell_value == 3) {
        color = "red";
      } else if (cell_value == 4) {
        color = "black";
      }
      // draw rect
      var z = 0.2;
      ctx.fillStyle = "#333333";
      ctx.fillRect(x - z * w, y - z * h, (1 + z) * w, (1 + z) * h);
      ctx.fillStyle = color;
      ctx.fillRect(x, y, w, h);
    }
  }
}

function get_next_grid_state(N, grid) {
  var new_grid = [];
  var new_row = [];
  var entry, new_entry;
  var neighbor, k_bc, l_bc;

  for (let i = 0; i < N; i++) {
    new_row = [];
    for (let j = 0; j < N; j++) {
      entry = grid[i][j];

      new_entry = 1;
      // for (let k = i - 1; k <= i + 1; k++) {
      //   for (let l = j - 1; l <= j + 1; l++) {
      for (let k = Math.max(i - 1, 0); k <= Math.min(i + 1, N - 1); k++) {
        for (let l = Math.max(j - 1, 0); l <= Math.min(j + 1, N - 1); l++) {
          if (i == k && l == j) {
            continue;
          }
          // apply bounds
          // if (k < 0) k_bc = k + N;
          // else if (k >= N) k_bc = k - N;
          // else k_bc = k;
          // if (l < 0) l_bc = l + N;
          // else if (l >= N) l_bc = l - N;
          // else l_bc = l;

          // neighbor = grid[k_bc][l_bc];
          neighbor = grid[k][l];
          if (entry == 1) {
            if (neighbor == 3) {
              if (Math.random() < 0.5) new_entry = 3;
              break;
            }
          } else if (entry == 2) {
            new_entry = 2;
          } else if (entry == 3) {
            new_entry = 4;
          } else if (entry == 4) {
            new_entry = 4;
          }
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
  var draw_radius;
  if (currently_selected_color == 3) {
    draw_radius = 1;
  } else {
    draw_radius = 7;
  }
  for (let k = i - draw_radius; k < i + draw_radius; k++) {
    for (let l = j - draw_radius; l < j + draw_radius; l++) {
      if (Math.sqrt((k - i) ** 2 + (l - j) ** 2) > draw_radius) continue;
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
  document.getElementById("button_1").addEventListener("click", function () {
    currently_selected_color = 1;
  });
  document.getElementById("button_2").addEventListener("click", function () {
    currently_selected_color = 2;
  });
  document.getElementById("button_3").addEventListener("click", function () {
    currently_selected_color = 3;
  });
  canvas.addEventListener("mousedown", function (e) {
    const pos = getCursorPosition(canvas, e);
    grid = flip_grid_entry(N, grid, pos[0], pos[1]);
  });

  const N = 150;
  var grid = initialize_grid(N);

  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw_grid(grid);
    if (!paused) {
      grid = get_next_grid_state(N, grid);
    }
    if (paused) {
      document.getElementById("play/pause").innerHTML = "unpause";
    } else {
      document.getElementById("play/pause").innerHTML = "pause";
    }
  }, 1000 / FPS_GOAL);
};

init();
