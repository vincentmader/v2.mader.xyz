// VARIABLE DEFINITIONS
// ============================================================================

// numerical parameters
const min_pheromone_drop_amount = 0.1;
const ant_speed = 1;
const sensor_radius = 5; // minimum 5 for ant to form streets
var phA_evaporation_rate; // = 0.99; // 1 - 1 / world_size[0]**2;
var phB_evaporation_rate; // = 0.99; // 1 - 1 / world_size[0]**2;
var colony_size; // = 750;
const probability_for_random_ant_turn = 0.7;
const max_ant_random_turn_angle = Math.PI / 4;
const ant_fov = (7 / 6) * Math.PI; // 2*Math.PI
// world parameters
const colony_radius = 3;
const colony_pos = [1.1 * colony_radius, 1.1 * colony_radius]; // [world_size[0] / 6, world_size[1] / 6];
const world_size = [60, 60];
// button presets
var paused = false;
var periodic_bounds = false;
var placement_select = "food";
// draw settings
const ant_drawing_radius = 0.08;
const food_drawing_radius = 1;
const pheromone_drawing_radius = 2;
var bool_draw_pheromones = false;
var bool_draw_registered_pheromones = false;
var bool_draw_ant_state_colors = true;
var canvas, ctx, W, H;
// world & ants
var world;
var ants = [];
const ant_eating_radius = 3;
const pheromone_drop_amount = 1; // amount of pheromone distributed by ant each turn
var ants_1, ants_2, ants_3, ants_4;
// stats
var time_step = 0;
var delivered_food = 0;
var fps;
var fps_values = [];
var time_01 = []; // TODO: remove
var time_02 = [];
var time_03 = [];
// constants
const TAU = 2 * Math.PI;

// CLASS DEFINITIONS

class Ant {
  constructor(spawn_position) {
    // set state variables
    this.is_carrying_food = false;
    this.has_detected_food = false;
    this.has_detected_pheromones = false;
    // spawn at colony
    this.x = spawn_position[0];
    this.y = spawn_position[1];
    // set orientation & velocity vector
    this.theta = TAU * Math.random(); // TODO: keep as class attribute?
  }
  assert_position_in_world() {
    if (this.x < 0) this.x = 0;
    else if (this.x > world_size[0] - 1) this.x = world_size[0] - 1;
    if (this.y < 0) this.y = 0;
    else if (this.y > world_size[1] - 1) this.y = world_size[1] - 1;
  }
  update_position_values() {
    this.x += this.u;
    this.y += this.v;
  }
  update_velocity_values() {
    this.u = ant_speed * Math.cos(this.theta);
    this.v = ant_speed * Math.sin(this.theta);
  }
  turn_randomly() {
    const rand = 2 * (Math.random() - 0.5);
    this.theta += max_ant_random_turn_angle * rand;
  }
  distribute_pheromone() {
    const ph_idx = { true: 1, false: 0 }[this.is_carrying_food];
    const idx = [Math.ceil, Math.floor][Math.floor(2 * Math.random())](this.x);
    const jdx = [Math.ceil, Math.floor][Math.floor(2 * Math.random())](this.y);
    world.pheromone_strengths[ph_idx][jdx][idx] += pheromone_drop_amount;
    world.active_grid_cells.push([jdx, idx]);
  }
  detect_pheromones() {
    // decide which pheromone to track based on ant's state
    const ph_idx = { true: 0, false: 1 }[this.is_carrying_food];
    // get broad search bounds for looping over cells in sensor radius
    const y_min = Math.max(0, Math.floor(this.y - sensor_radius));
    const y_max = Math.min(world_size[1], Math.floor(this.y + sensor_radius));
    const x_min = Math.max(0, Math.floor(this.x - sensor_radius));
    const x_max = Math.min(world_size[0], Math.floor(this.x + sensor_radius));

    this.has_detected_pheromones = false;
    // loop over all cells in broad search (square grid around ant)
    var pheromone_strength;
    const weights = [0, 0, 0];
    for (let y = y_min; y < y_max; y++) {
      for (let x = x_min; x < x_max; x++) {
        // get view angle of marker from ant's fov ("-"=left, "+"=right)
        let delta_x = x - this.x;
        let delta_y = y - this.y;
        let phi = Math.atan2(delta_y, delta_x) - this.theta;
        // make sure phi is in [0, 2*pi]
        while (Math.abs(phi) > Math.PI) phi -= Math.sign(phi) * TAU;

        // skip cell if it is outside of ant's FOV
        if (Math.abs(phi >= ant_fov / 2)) continue;
        // skip cell if it is (almost empty), get pheromone strength
        pheromone_strength = world.pheromone_strengths[ph_idx][y][x];
        if (pheromone_strength <= min_pheromone_drop_amount) continue;
        // skip cell if it is outside of sensor radius
        let r = Math.sqrt(delta_y ** 2 + delta_x ** 2);
        // if (r > sensor_radius) continue;

        let direction_idx = Math.floor(3 * (phi / ant_fov + 0.5));
        weights[direction_idx] += pheromone_strength;
        this.has_detected_pheromones = true;

        // draw registered pheromones
        if (bool_draw_registered_pheromones) draw_registered_pheromones(x, y);
      }
    }
    // if pheromones detected: turn towards the highest density region
    if (this.has_detected_pheromones) {
      const direction_idx = argmax([weights[0], weights[1], weights[2]]) - 1;
      this.theta += (ant_fov / 3) * direction_idx;
    }
  }
  detect_colony() {
    var colony_found = false;
    var distance_to_cell, distance_to_colony, colony_x, colony_y;
    // get broad search bounds for looping over cells in sensor radius
    const y_min = Math.max(0, Math.floor(this.y - sensor_radius));
    const y_max = Math.min(world_size[1], Math.floor(this.y + sensor_radius));
    const x_min = Math.max(0, Math.floor(this.x - sensor_radius));
    const x_max = Math.min(world_size[0], Math.floor(this.x + sensor_radius));
    // gather list of food locations within sensor radius
    for (let y = y_min; y < y_max; y++) {
      for (let x = x_min; x < x_max; x++) {
        distance_to_cell = Math.sqrt((x - this.x) ** 2 + (y - this.y) ** 2);
        if (distance_to_cell > sensor_radius) continue;
        let delta_x = x - colony_pos[0];
        let delta_y = y - colony_pos[1];
        distance_to_colony = Math.sqrt(delta_x ** 2 + delta_y ** 2);
        if (distance_to_colony <= colony_radius) {
          colony_found = true;
          colony_x = x;
          colony_y = y;
        }
        if (colony_found) break;
      }
      if (colony_found) break;
    }
    // change orientation so as to face next food target (first from list)
    if (colony_found) {
      const delta_y = colony_y - Math.floor(this.y);
      const delta_x = colony_x - Math.floor(this.x);
      const theta = Math.atan2(delta_y, delta_x);
      this.theta = theta;
    }
  }
  detect_food() {
    var distance_to_food, x, y;
    // get broad search bounds for looping over cells in sensor radius
    const y_min = Math.max(0, Math.floor(this.y - sensor_radius));
    const y_max = Math.min(world_size[1], Math.floor(this.y + sensor_radius));
    const x_min = Math.max(0, Math.floor(this.x - sensor_radius));
    const x_max = Math.min(world_size[0], Math.floor(this.x + sensor_radius));
    // gather list of food locations within sensor radius
    const food_locations = [];
    for (let y = y_min; y < y_max; y++) {
      for (let x = x_min; x < x_max; x++) {
        distance_to_food = Math.sqrt((x - this.x) ** 2 + (y - this.y) ** 2);
        if (distance_to_food <= sensor_radius && world.food_sources[y][x] > 0) {
          food_locations.push([x, y]); // TODO: go for largest src?
        }
      }
    }
    // loop over food locations, check if food can be picked up
    for (let food_location of food_locations) {
      x = food_location[0];
      y = food_location[1];
      distance_to_food = Math.sqrt((x - this.x) ** 2 + (y - this.y) ** 2);
      if (distance_to_food <= ant_eating_radius) {
        this.pickup_food(x, y);
        return;
      }
    }
    // change orientation so as to face next food target
    // (first from list for now, TODO: change?)
    if (food_locations.length > 0) {
      x = food_locations[0][0];
      y = food_locations[0][1];
      let delta_y = y - this.y;
      let delta_x = x - this.x;
      let theta = Math.atan2(delta_y, delta_x);
      this.theta = theta;
    }
  }
  pickup_food(x, y) {
    let floored_x = Math.floor(x);
    let floored_y = Math.floor(y);
    // change world food source list
    world.food_sources[floored_y][floored_x] -= 1;
    // change state
    this.is_carrying_food = true;
    // turn around to walk back // TODO: toggle?
    this.theta += Math.PI / 2;
  }
  deliver_food() {
    if (this.is_carrying_food) {
      let delta_x = colony_pos[0] - this.x;
      let delta_y = colony_pos[1] - this.y;
      let distance_to_colony = Math.sqrt(delta_x ** 2 + delta_y ** 2);
      // check if food can be delivered
      if (distance_to_colony <= colony_radius) {
        this.is_carrying_food = false;
        delivered_food += 1;
        this.theta += Math.PI; // turn around
        // } else if (distance_to_colony <= sensor_radius) {
        //   this.theta = Math.atan2(delta_y, delta_x); // TODO: fix sign?
      }
    }
  }
  detect_walls() {
    let new_x = this.x + 2 * this.u;
    let new_y = this.y + 2 * this.v;
    // METHOD 1: remove ants from ants list when leaving world
    // if (
    //   new_x > world_size[0] - 1 ||
    //   new_x < 0 ||
    //   new_y > world_size[1] - 1 ||
    //   new_y < 0
    // ) {
    //   ants = ants.filter((val, idx, arr) => val !== this);
    // }
    // METHOD 2: apply periodic bounds
    // if (periodic_bounds === true) {
    //   if (new_x > world_size[0] - 1) {
    //     this.x = 0;
    //   } else if (new_x < 0) {
    //     this.x = world_size[0] - 1;
    //   }
    //   if (new_y > world_size[1] - 1) {
    //     this.y = 0;
    //   } else if (new_y < 0) {
    //     this.y = world_size[1] - 1;
    //   }
    // } else {
    // METHOD 3: update velocities on 'wall' hit
    // if (world.has_wall_at_position([new_x, new_y])) {
    //   // this.theta += Math.PI * (Math.random() - 0.5) // TODO: improve?
    //   this.theta = TAU * Math.random();
    //   // this.theta -= Math.PI + (Math.random() - 0.5) * Math.PI/4;
    //   this.update_velocity_values();
    //   if (!this.detect_walls()) {
    //     return false;
    //   } else return true;
    // }
    // METHOD 3.1: different velocity update (move away from wall)
    if (new_x > world_size[0] - 1) {
      this.theta = Math.PI;
    } else if (new_x < 0) {
      this.theta = 0;
    }
    if (new_y > world_size[1] - 1) {
      this.theta = -Math.PI / 2;
    } else if (new_y < 0) {
      this.theta = Math.PI / 2;
    }
    // }

    this.update_velocity_values();
  }
  draw() {
    const ctx_coords = get_ctx_coords([this.x, this.y]);
    // var color;
    // if (bool_draw_ant_state_colors) {
    // color = { true: "green", false: "white" }[this.is_carrying_food];
    // } else color = "white";
    ctx.fillStyle = { true: "green", false: "white" }[this.is_carrying_food];
    ctx.strokeStyle = "white";
    const ctx_radius = get_ctx_radius(ant_drawing_radius);
    // ctx.beginPath();
    // ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
    // ctx.stroke();
    ctx.fillRect(ctx_coords[0], ctx_coords[1], ctx_radius, ctx_radius);
    ctx.fill();
  }
  draw_sensor_radius() {
    const ctx_coords = get_ctx_coords([this.x, this.y]);
    ctx.strokeStyle = "white";
    const ctx_radius = get_ctx_radius(sensor_radius);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
    ctx.stroke();
  }
  draw_velocity_vector() {
    const c = 15; // factor increase vector arrow size
    ctx.strokeStyle = "white";
    ctx.beginPath();
    const ctx_coords_i = get_ctx_coords([this.x, this.y]);
    ctx.moveTo(ctx_coords_i[0], ctx_coords_i[1]);
    const ctx_coords_f = get_ctx_coords([
      this.x + c * this.u,
      this.y + c * this.v,
    ]);
    ctx.lineTo(ctx_coords_f[0], ctx_coords_f[1]);
    ctx.stroke();
  }
  update() {
    this.assert_position_in_world();
    if (this.is_carrying_food) {
      this.detect_colony();
      this.deliver_food();
      this.detect_pheromones();
    } else {
      this.detect_food();
      if (this.has_detected_food) {
        //
      } else {
        this.detect_pheromones();
      }
    }
    // if (!this.has_detected_pheromones) {
    if (Math.random() < probability_for_random_ant_turn) {
      this.turn_randomly();
    }
    // }
    this.detect_walls(); // needs to be called last (update_vel_val())

    if (time_step % 2 == 0) this.distribute_pheromone(); // TODO: after move?
    this.update_position_values();
  }
}

class World {
  constructor(world_size) {
    this.world_width = world_size[0];
    this.world_height = world_size[1];
    // grid arrays containing information about the world
    this.food_sources = [];
    for (let row_idx = 0; row_idx < world_size[0]; row_idx++) {
      this.food_sources[row_idx] = [];
      for (let col_idx = 0; col_idx < world_size[1]; col_idx++) {
        this.food_sources[row_idx][col_idx] = 0;
      }
    }
    this.pheromone_strengths = [[], []];
    for (let pheromone of [0, 1]) {
      for (let row_idx = 0; row_idx < world_size[0]; row_idx++) {
        this.pheromone_strengths[pheromone][row_idx] = [];
        for (let col_idx = 0; col_idx < world_size[1]; col_idx++) {
          this.pheromone_strengths[pheromone][row_idx][col_idx] = 0;
        }
      }
    }
    this.active_grid_cells = [];
  }
  draw_food_sources() {
    const ctx_radius = get_ctx_radius(food_drawing_radius);
    ctx.fillStyle = "green";
    var ctx_coords;
    for (let row_idx = 0; row_idx < world_size[0]; row_idx++) {
      for (let col_idx = 0; col_idx < world_size[1]; col_idx++) {
        if (this.food_sources[row_idx][col_idx] === 0) {
          continue;
        }
        ctx_coords = get_ctx_coords([col_idx, row_idx]);
        ctx.fillStyle = "green";
        ctx.strokeStyle = "green";
        ctx.beginPath();
        ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
        ctx.stroke();
        ctx.fill();
        // display position as text
        // if (colony_size < 10) {
        //   ctx.font = "20px Arial";
        //   ctx.fillStyle = "white"
        //   ctx.fillText("(" + col_idx + ", " + row_idx + ")", ctx_coords[0], ctx_coords[1]);
        // }
      }
    }
  }
  draw_ant_colonies() {
    // TODO: implement multiple colonies
    const ctx_coords = get_ctx_coords([colony_pos[0], colony_pos[1]]); // TODO: switch indices?
    ctx.fillStyle = "#181818";
    ctx.strokeStyle = "#181818";
    // ctx.fillRect(
    //   ctx_coords[0], ctx_coords[1], ant_drawing_radius, ant_drawing_radius
    // );
    const ctx_radius = get_ctx_radius(colony_radius);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
    ctx.stroke();
    ctx.fill();
  }
  draw_pheromones() {
    const ctx_radius = get_ctx_coords([
      pheromone_drawing_radius,
      pheromone_drawing_radius,
    ])[0];
    // determine max & min pheromone strength values, TODO: remove?
    // for (let i of [0, 1]) {
    //   max_pheromone_strengths[i] = max_from_2D_array(
    //     world.pheromone_strengths[i]
    //   );
    // }
    const max_pheromone_strengths = [1, 1];
    // loop over grid and draw
    var ctx_coords, strength_A, strength_B, x, y;
    // for (let y = 0; y < world_size[0]; y++) {
    // for (let x = 0; x < world_size[1]; x++) {
    for (let idx = 0; idx < this.active_grid_cells.length; idx++) {
      // if (idx % 2 == 0 || idx % 3 == 0) {continue}
      let cell = this.active_grid_cells[idx];
      // if (Math.random() > 0.8) {continue}

      x = cell[1];
      y = cell[0];
      // get marker strengths from cell, skip (quasi-)empty ones
      strength_A = this.pheromone_strengths[0][y][x];
      strength_B = this.pheromone_strengths[1][y][x];
      if (Math.max(strength_A, strength_B) < min_pheromone_drop_amount) {
        continue;
      }
      // draw either phA or phB (stronger one) with strength-dependent alpha
      if (strength_A <= strength_B && strength_B) {
        // Pheromone B
        var alpha = sigmoid(
          strength_B / Math.max(1, max_pheromone_strengths[1])
        );
        ctx.fillStyle = "rgba(255, 64, 0, " + alpha + ")";
      } else if (strength_A >= strength_B) {
        // Pheromone A
        var alpha = sigmoid(
          strength_A / Math.max(1, max_pheromone_strengths[0])
        );
        ctx.fillStyle = "rgba(128, 255, 255, " + alpha + ")";
      } else {
        continue;
      }
      // place marker
      ctx_coords = get_ctx_coords([x, y]);
      ctx.fillRect(ctx_coords[0], ctx_coords[1], ctx_radius, ctx_radius);
      // }
    }
  }
  evaporate_pheromones() {
    const nr_of_active_cells = this.active_grid_cells.length;
    // either loop over all N**2 cells...
    // for (let row_idx = 0; row_idx < world_size[0]; row_idx++) {
    //   for (let col_idx = 0; col_idx < world_size[1]; col_idx++) {
    // ... or use Monte Carlo techniques
    // const foo = Math.min(4*colony_size, nr_of_active_cells)
    // for (let _ = 0; _ < foo; _++) {
    //   var idx = Math.floor(Math.random() * nr_of_active_cells)
    // ... or loop over list of active cells
    for (let idx = 0; idx < nr_of_active_cells; idx++) {
      // if (idx % 2 == 0 || idx % 3 == 0) {continue}
      let cell = this.active_grid_cells[idx];
      let row_idx = cell[0];
      let col_idx = cell[1];
      if (
        world.pheromone_strengths[0][row_idx][col_idx] * phA_evaporation_rate <
        min_pheromone_drop_amount
      ) {
        world.pheromone_strengths[0][row_idx][col_idx] = 0;
        // remove_from_array(this.active_grid_cells, [row_idx, col_idx])
        // world.active_grid_cells.splice(idx_in_list, 1);
      } else {
        world.pheromone_strengths[0][row_idx][col_idx] *= phA_evaporation_rate;
      }
      if (
        world.pheromone_strengths[1][row_idx][col_idx] * phB_evaporation_rate <
        min_pheromone_drop_amount
      ) {
        world.pheromone_strengths[1][row_idx][col_idx] = 0;
        // remove_from_array(this.active_grid_cells, [row_idx, col_idx])
        // world.active_grid_cells.splice(idx_in_list, 1);
      } else {
        world.pheromone_strengths[1][row_idx][col_idx] *= phB_evaporation_rate;
      }
    }
    // world.active_grid_cells = world.active_grid_cells.slice(20, nr_of_active_cells)
  }
  has_wall_at_position(position) {
    // check for world edges in horizontal direction
    if (position[0] < 0 || position[0] > world_size[0]) {
      return true;
    }
    // check for world edges in vertical direction
    if (position[1] < 0 || position[1] > world_size[1]) {
      return true;
    }
    // check for walls on map
    // ...
    // else: return false
    return false;
  }
  update_ants() {
    // TODO: move function elsewhere?
    // ants.forEach((ant) => {
    //   let x = ant.x;
    //   let y = ant.y;
    // });
    ants.forEach((ant) => {
      ant.update();
      if (!bool_draw_pheromones) {
        // if (colony_size < 100) {
        //   ant.draw_sensor_radius();
        //   ant.draw_velocity_vector();
        // }
        ant.draw();
      }
    });
  }
  update() {
    this.evaporate_pheromones();
  }
}

// UTILITY FUNCTIONS
// ============================================================================

const reset_time = () => {
  time_step = 0;
  delivered_food = 0;
};
const get_map_coords = (ctx_coords) => {
  const map_coord_x = (ctx_coords[0] / W) * world_size[0];
  const map_coord_y = (ctx_coords[1] / H) * world_size[1];
  return [map_coord_x, map_coord_y];
};
const get_ctx_coords = (map_coords) => {
  const ctx_coord_x = (map_coords[0] / world_size[0]) * W;
  const ctx_coord_y = (map_coords[1] / world_size[1]) * H;
  return [ctx_coord_x, ctx_coord_y];
};
const get_ctx_radius = (radius) => {
  return (radius / world_size[0]) * W; // TODO: only for H=W
};
const getCursorPosition = (canvas, event) => {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  return [x, y];
};
const argmax = (arr) => {
  if (arr.length === 0) {
    return -1;
  }

  var max = arr[0];
  var maxIndex = 0;

  for (var i = 1; i < arr.length; i++) {
    if (arr[i] > max) {
      maxIndex = i;
      max = arr[i];
    }
  }

  return maxIndex;
};
const min_from_1D_array = (arr) => {
  return arr.reduce(function (a, b) {
    return Math.min(a, b);
  });
};
const max_from_1D_array = (arr) => {
  return arr.reduce(function (a, b) {
    return Math.max(a, b);
  });
};
const min_from_2D_array = (arr) => {
  var values = arr.map(function (elt) {
    return elt[1];
  });
  return Math.min.apply(null, values);
};
const max_from_2D_array = (arr) => {
  var values = arr.map(function (elt) {
    return elt[1];
  });
  return Math.max.apply(null, values);
};
const sigmoid = (x) => {
  return Math.exp(x) / (Math.exp(x) + 1);
};
const round = (num, acc) => {
  return Math.round((num + Number.EPSILON) * 10 ** acc) / 10 ** acc;
};
const add_info_text = () => {
  ctx.font = "30px Arial";
  ctx.fillText("t = " + time_step, 10, 50);
};
const sleep = (ms) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};
const sleepFor = (ms) => {
  var now = new Date().getTime();
  while (new Date().getTime() < now + ms) {
    /* do nothing */
  }
};
const remove_from_array = (arr, item) => {
  const index = arr.indexOf(item);
  if (index > -1) {
    arr.splice(index, 1);
  }
};
const mean = (arr) => {
  var sum = 0;
  for (let i of arr) {
    sum += i;
  }
  return sum / arr.length;
};
function updateAnts(arr) {
  // TODO: move function elsewhere?
  arr.forEach((ant) => {
    ant.update();
    if (!bool_draw_pheromones) {
      // if (colony_size < 100) {
      //   ant.draw_sensor_radius();
      //   ant.draw_velocity_vector();
      // }
      ant.draw();
    }
  });
}
const add_event_listeners = () => {
  canvas.addEventListener("mousedown", function (e) {
    const ctx_coords = getCursorPosition(canvas, e);
    const map_coords = get_map_coords(ctx_coords);
    const col_idx = Math.floor(map_coords[0]);
    const row_idx = Math.floor(map_coords[1]);
    if (placement_select === "food") {
      world.food_sources[row_idx][col_idx] += 2000;
    } else if (placement_select === "phA") {
      world.pheromone_strengths[0][row_idx][col_idx] += 100;
      world.active_grid_cells.push([row_idx, col_idx]);
    } else if (placement_select === "phB") {
      world.pheromone_strengths[1][row_idx][col_idx] += 100;
      world.active_grid_cells.push([row_idx, col_idx]);
    }
    // console.log(ctx_coords, [col_idx, row_idx]);
  });
  document
    .getElementById("button_play/pause")
    .addEventListener("click", function () {
      if (paused) {
        paused = false;
        // TODO: change button text
      } else {
        paused = true;
        // TODO: change button text
      }
    });
  document
    .getElementById("button_play/pause")
    .addEventListener("click", function () {
      console.log("ayyy");
    });
  document
    .getElementById("slider_colony_size")
    .addEventListener("click", function () {
      colony_size = document.getElementById("slider_colony_size").value;
      // console.log("aaaaaaaa");
      console.log(colony_size);
      init();
    });
  document
    .getElementById("button_reset")
    .addEventListener("click", function () {
      // animate()
      init();
    });
  document
    .getElementById("button_place_food")
    .addEventListener("click", function () {
      placement_select = "food";
    });
  document
    .getElementById("button_place_walls")
    .addEventListener("click", function () {
      placement_select = "walls";
    });
  document
    .getElementById("button_place_phA")
    .addEventListener("click", function () {
      placement_select = "phA";
    });
  document
    .getElementById("button_place_phB")
    .addEventListener("click", function () {
      placement_select = "phB";
    });
  document
    .getElementById("button_draw_pheromones")
    .addEventListener("click", function () {
      bool_draw_pheromones = !bool_draw_pheromones;
    });
  document
    .getElementById("button_periodic_bounds")
    .addEventListener("click", function () {
      periodic_bounds = !periodic_bounds;
    });
};
const draw_registered_pheromones = (x, y) => {
  if (colony_size < 100) {
    ctx.fillStyle = "white";
    ctx.strokeStyle = "white";
    const ctx_coords = get_ctx_coords([x, y]);
    const ctx_r = get_ctx_radius(1);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], 5 * ctx_r, 0, TAU);
    ctx.stroke();
    ctx.fill();
  }
};

// ANIMATION LOOP
// ============================================================================

var iteration_start_time = new Date();
async function animate() {
  // get fps
  const new_start_time = new Date();
  const fps = 1000 / (new_start_time - iteration_start_time);
  iteration_start_time = new_start_time;
  if (fps_values.length >= 100) fps_values.shift();
  fps_values.push(fps);

  // create animation loop
  requestAnimationFrame(animate);
  if (paused) {
    return;
  }

  // get slider values (TODO: add onchange event listener?)
  const slider_A = document.getElementById("slider_evaporation_factor_A").value;
  const slider_B = document.getElementById("slider_evaporation_factor_B").value;
  phA_evaporation_rate = Number(slider_A) / 1000;
  phB_evaporation_rate = Number(slider_B) / 1000;

  // update world every 10 time steps
  if (time_step % 10 == 0) {
    world.update();
  }

  var foo = new Date();
  // time_01.push(foo - iteration_start_time);
  // if (time_01.length > 20) {
  //   time_01.shift();
  // }

  // Erase whole canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // if (time_step % 2 == 0) {
  //   // draw pheromones
  if (bool_draw_pheromones) world.draw_pheromones();
  // }

  var foo2 = new Date();
  time_02.push(foo2 - foo);
  if (time_02.length > 20) {
    time_02.shift();
  }

  // Update ants
  // function updateAnts(arr) {
  // ants.forEach((ant) => {
  //   ant.update();
  //   if (!bool_draw_pheromones) {
  //     // if (colony_size < 100) {
  //     //   ant.draw_sensor_radius();
  //     //   ant.draw_velocity_vector();
  //     // }
  //     ant.draw();
  //   }
  // });
  // }
  // let ants_1 = ants.slice(0, ants.length / 4);
  // let ants_2 = ants.slice(ants.length / 4, ants.length / 2);
  // let ants_3 = ants.slice(ants.length / 2, (3 * ants.length) / 4);
  // let ants_4 = ants.slice((3 * ants.length) / 4, ants.length);

  // let promise_1 = new Promise((res) => updateAnts(ants_1));
  // let promise_2 = new Promise((res) => updateAnts(ants_2));
  // let promise_3 = new Promise((res) => updateAnts(ants_3));
  // let promise_4 = new Promise((res) => updateAnts(ants_4));
  // Promise.all([promise_1, promise_2, promise_3, promise_4]);
  world.update_ants();
  // updateAnts(ants);

  var foo3 = new Date();
  time_03.push(foo3 - foo2);
  if (time_03.length > 20) {
    time_03.shift();
  }

  // var new_date = new Date();
  // console.log("ants: " + (new_date - old_date))
  // var old_date = new_date

  world.draw_ant_colonies();
  world.draw_food_sources();

  // var foo4 = new Date();
  // var time_04 = foo4 - foo3

  // var new_date = new Date();
  // console.log("col/food: " + (new_date - old_date))
  // var old_date = new_date
  // console.log("===========")

  // add_info_text() // TODO: create canvas class/object
  // if (time_step % 200 === 0) {
  //   console.log("delivered food: " + delivered_food);
  // }

  // TODO: push new antzz
  // for (let i = 0; i < colony_size - ants.length; i++) {
  //   var phi = TAU * Math.random();
  //   var ant_spawn_pos = [
  //     colony_pos[0] + colony_radius * Math.cos(phi),
  //     colony_pos[1] + colony_radius * Math.sin(phi),
  //   ];
  //   ants.push(new Ant(ant_spawn_pos));
  // }

  // Increment time
  time_step += 1;
  // sleepFor(40);

  ctx.font = "18px Arial";
  ctx.fillStyle = "white";
  // var fps = mean(fps_values)
  ctx.fillText("fps = " + round(mean(fps_values), 0), 0.8 * W, 0.99 * H);
  // ctx.fillText("world: " + round(mean(time_01), 1), 0.79 * W, 0.94 * H);
  // ctx.fillText("draw: " + round(mean(time_02), 1), 0.79 * W, 0.89 * H);
  // ctx.fillText("ants: " + round(mean(time_03), 1), 0.79 * W, 0.84 * H);
  ctx.fillText("ants: " + round(mean(time_03), 1), 0.79 * W, 0.94 * H);

  // console.log(min_from_1D_array(foo0), max_from_1D_array(foo0))
}

// INITIALIZATION
// ============================================================================

const init = () => {
  // setup canvas
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");

  // setup world
  world = new World(world_size);

  // setup ants
  colony_size = Number(document.getElementById("slider_colony_size").value);
  ants = [];
  var phi, ant_spawn_pos;
  for (let idx = 0; idx < colony_size; idx++) {
    phi = TAU * Math.random();
    ant_spawn_pos = [
      colony_pos[0] + colony_radius * Math.cos(phi),
      colony_pos[1] + colony_radius * Math.sin(phi),
    ];
    ants.push(new Ant(ant_spawn_pos));
  }
  // ants_1 = ants.slice(0, ants.length / 4);
  // ants_2 = ants.slice(ants.length / 4, ants.length / 2);
  // ants_3 = ants.slice(ants.length / 2, (3 * ants.length) / 4);
  // ants_4 = ants.slice((3 * ants.length) / 4, ants.length);

  // reset time
  reset_time();

  // add_event_listeners
  add_event_listeners();
};

init();
animate(new Date().getTime());
