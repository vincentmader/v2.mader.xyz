// declare variables
var canvas, ctx, W, H;
var paused = true;

// initialization of grid
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

// draw grid
function draw_grid(grid) {
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
}

function get_next_grid_state(N, grid) {
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
      // for (let k = Math.max(i - 1, 0); k <= Math.min(i + 1, N - 1); k++) {
      //   for (let l = Math.max(j - 1, 0); l <= Math.min(j + 1, N - 1); l++) {
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
  ctx.lineWidth = 2;
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";

  document.getElementById("play/pause").addEventListener("click", function () {
    paused = !paused;
    if (paused) document.getElementById("play/pause").innerHTML = "Unpause";
    if (!paused) document.getElementById("play/pause").innerHTML = "Pause";
  });
  canvas.addEventListener("mousedown", function (e) {
    const pos = getCursorPosition(canvas, e);
    grid = flip_grid_entry(N, grid, pos[0], pos[1]);
  });

  const N = 20;
  var grid = initialize_grid(N);

  var frame_idx = 0;
  setInterval(function () {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    draw_grid(grid);
    if (!paused) {
      grid = get_next_grid_state(N, grid);
      frame_idx += 1;
    }
  }, 150);
};

init();
