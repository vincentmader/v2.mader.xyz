const N = 20; // TODO: make changeable

var paused = false;
const fps_goal = 10;

var canvas, ctx, W, H;
var grid;

// initialization of grid
const initialize_grid = (N) => {
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
};

const initialize_example = (grid, N) => {
  let x = 2;
  let y = 2;
  grid[y][x + 1] = 1;
  grid[y - 1][x + 1] = 1;
  grid[y][x] = 1;
  grid[y + 1][x] = 1;
  grid[y - 1][x - 1] = 1;
  grid[N - 2][N - 5] = 1;
  grid[N - 3][N - 5] = 1;
  grid[N - 2][N - 6] = 1;
  grid[N - 3][N - 6] = 1;
  return grid;
};
// draw grid
const draw_grid = (grid) => {
  const N = grid.length;
  var x, y;
  var w = W / N;
  var h = H / N;
  var color;

  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      // get position and geometry of cell
      x = (W / N) * i;
      y = (H / N) * j;
      // get color for cell
      if (grid[i][j] == -1) {
        color = "black";
      } else if (grid[i][j] == +1) {
        color = "white";
      }
      // draw rect
      var z = 0.1; // used for drawing cell borders
      ctx.fillStyle = "#333333";
      ctx.fillRect(x - z * w, y - z * h, (1 + z) * w, (1 + z) * h);
      ctx.fillStyle = color;
      ctx.fillRect(x, y, w, h);
    }
  }
};

const get_next_grid_state = (N, grid) => {
  var new_grid, new_row;
  var entry, new_entry;
  var k_bc, l_bc;
  var nr_of_neighbors, neighbor;

  new_grid = [];
  for (let i = 0; i < N; i++) {
    new_row = [];
    for (let j = 0; j < N; j++) {
      entry = grid[i][j];

      nr_of_neighbors = 0;
      for (let k = i - 1; k <= i + 1; k++) {
        for (let l = j - 1; l <= j + 1; l++) {
          if (i == k && l == j) {
            continue;
          }
          // apply periodic boundaries
          if (k < 0) k_bc = k + N;
          else if (k >= N) k_bc = k - N;
          else k_bc = k;
          if (l < 0) l_bc = l + N;
          else if (l >= N) l_bc = l - N;
          else l_bc = l;
          // calculate number of neighbors
          neighbor = grid[k_bc][l_bc];
          if (neighbor == 1) {
            nr_of_neighbors += 1;
          }
        }
      }
      // decide fate of cell
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
      } else if (3 < nr_of_neighbors) {
        new_entry = -1;
      }
      // append to array and return
      new_row.push(new_entry);
    }
    new_grid.push(new_row);
  }
  return new_grid;
};

const flip_grid_entry = (N, grid, x, y) => {
  const i = Math.floor((x / W) * N);
  const j = Math.floor((y / H) * N);
  grid[i][j] *= -1;
  return grid;
};

const getCursorPosition = (canvas, event) => {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  return [x, y];
};

const add_event_listeners = () => {
  let button_toggle_pause = document.getElementById("button_toggle_pause");
  button_toggle_pause.addEventListener("click", function () {
    paused = !paused;
    if (paused) button_toggle_pause.innerHTML = "unpause";
    if (!paused) button_toggle_pause.innerHTML = "pause";
  });
  let button_reset = document.getElementById("button_reset");
  button_reset.addEventListener("click", function () {
    grid = initialize_grid(N);
    paused = true;
    if (paused) button_toggle_pause.innerHTML = "unpause";
    if (!paused) button_toggle_pause.innerHTML = "pause";
  });
  canvas.addEventListener("mousedown", function (e) {
    const pos = getCursorPosition(canvas, e);
    grid = flip_grid_entry(N, grid, pos[0], pos[1]);
  });
};

const animate = () => {
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw_grid(grid);
    if (!paused) {
      grid = get_next_grid_state(N, grid);
    }
  }, 1000 / fps_goal);
};

const init = () => {
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");
  ctx.lineWidth = 2;
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";

  add_event_listeners();
  grid = initialize_grid(N);
  grid = initialize_example(grid, N);
  animate();
};

init();
