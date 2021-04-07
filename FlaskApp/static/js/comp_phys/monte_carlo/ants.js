const line_width = 3;

const ants = [];
const colony_size = 100; // nr of ants
const ant_step_size = 10;
const map = [];
const map_size = [1000, 1000]
// const ant_sensor_radius = 20

var canvas, ctx;
var W, H;
var time_step;
const colony_pos = [0, 0];

class Ant {
  constructor(colony_pos) {
    // spawn at colony
    this.x = colony_pos[0];
    this.y = colony_pos[1];
    // set orientation & velocity
    const theta = 2 * Math.PI * Math.random();
    this.u = Math.cos(theta);
    this.v = Math.sin(theta);
  }
  pick_next_direction() {
    // TODO: if there is a food source inside the ants' sensor radius, get it!
    //
    // avoid leaving canvas area
    if (this.x + 1 > map_size[0] / 2) this.u *= -1;
    else if (this.x - 1 < -map_size[0] / 2) this.u *= -1;
    if (this.y + 1 > map_size[1] / 2) this.v *= -1;
    else if (this.y - 1 < -map_size[1] / 2) this.v *= -1;
    // randomly turn right/left of go straight based on scents
    if (Math.random() > 0.8) this.get_random_direction()
  }
  get_random_direction() {
    const rand = Math.random();
    var theta;
    if (0 < rand <= 1/3) {
      theta = Math.PI / 4 // go left
    } else if (1/3 < rand <= 2/3) {
      theta = 0 // go forward
    } else {
      theta = -Math.PI / 4 // go right
    }
    this.u = this.u * Math.cos(theta) - this.v * Math.sin(theta)
    this.v = this.u * Math.sin(theta) + this.v * Math.cos(theta)
    // console.log(this.u**2 + this.v**2)
  }
  move() {
    this.x += this.u * ant_step_size
    this.y += this.v * ant_step_size
  }
  update() {
    this.pick_next_direction();
    this.move();
  }
  draw() {
    var ctx_coords, x, y
    ctx.fillStyle = "white";
    ctx_coords = get_ctx_coords([this.x, this.y]);
    x = ctx_coords[0];
    y = ctx_coords[1];
    ctx.fillRect(x, y, 1, 1);
  }
}

const get_ctx_coords = (map_coords) => {
  const map_coord_x = map_coords[0];
  const map_coord_y = map_coords[1];
  const ctx_coord_x = (map_coord_x / map_size[0] + 0.5) * W;
  const ctx_coord_y = (map_coord_y / map_size[1] + 0.5) * H;
  return [ctx_coord_x, ctx_coord_y];
}

const initialize_map = (map_size) => {
  var row;
  for (let row_idx = 0; row_idx < map_size[0]; row_idx++) {
    row = [];
    for (let col_idx = 0; col_idx < map_size[1]; col_idx++) {
      row.push(0); // no food, no scent
    }
    map.push();
  }
  return map;
};

const draw_map = () => {};

const reset_time = () => {
  time_step = 0;
};

const initialize_ants = (colony_size) => {
  var ant;
  for (let idx = 0; idx < colony_size; idx++) {
    ant = new Ant(colony_pos);
    ants.push(ant);
  }
};

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

  reset_time();
  initialize_ants(colony_size);
  initialize_map(map_size);
};

// Animation Loop
const animate = () => {
  requestAnimationFrame(animate); // Create an animation loop
  ctx.clearRect(0, 0, canvas.width, canvas.height); // Erase whole canvas
  ants.forEach((ant) => {
    ant.update();
    ant.draw();
  });
  draw_map();
  time_step += 1;
  // let text = String(get_pressure_from_energy(circles)) + "   ";
  // text += String(get_pressure_from_wall_collisions());
  // document.getElementById("infoBox").textContent = text;
  // let ay = document.getElementById("infoBox");
  // ay.innerHTML = text;
};

init();
animate();
