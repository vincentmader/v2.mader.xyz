// Asteroids
// Vincent C. Mader

// IMPORTS
// ============================================================================

import { Vector2D } from "../../utils/math_utils.js";
// import { get_random_color } from "../../utils/color_utils.js";

// VARIABLE DEFINITIONS
// ============================================================================

// numerical parameters

//var use_quad_tree = true; // else: direct summation, O(N^2)
//var quad_tree_capacity = 1;
//var max_opening_angle = Math.PI / 20;
var DT = 3; // TODO: make changeable
var EPSILON = 80;
var nr_of_asteroids = 2500;
var min_r_for_merger = 2;
var belt_random_phi_distribution = true;
var belt_min_r = 250;
var belt_width = 200;
var belt_exp_factor = 2;
//var use_leapfrog = false; // else: Euler

// world parameters
const world_size = [1000, 1000];
//var quad_tree;

// button presets
var bool_show_trajectories = false;
//var bool_show_quad_tree_grid = !use_quad_tree;
var paused = false;
//// var periodic_bounds = false;

// draw settings
var canvas, ctx, W, H;
var asteroid_drawing_radius = 0.4; // TODO: make changeable?

// world
var bodies, asteroids;
var world;

// stats
var time_step;
//// var fps;
//// var fps_values = [];

// constants
const TAU = 2 * Math.PI;
const G = 1;

// CLASS DEFINITIONS

class Particle {
  constructor(mass, pos_0, vel_0) {
    this.mass = mass;
    this.position = pos_0;
    this.velocity = vel_0;
  }
  draw() {
    ctx.strokeStyle = "gray";
    ctx.fillStyle = "gray";
    let ctx_coords = get_ctx_coords([this.position.x, this.position.y]);
    let r = get_ctx_radius(asteroid_drawing_radius);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], 3 * r, 0, TAU);
    ctx.stroke();
    ctx.fill();
  }
  update_velocity() {
    for (let b of bodies) {
      let delta_pos = b.position.sub(this.position);
      let r = delta_pos.norm_l2(); // distance
      let unit_vec = delta_pos.mult(1 / r);
      let acc = unit_vec.mult((G * b.mass) / (r ** 2 + EPSILON ** 2));
      this.velocity = this.velocity.add(acc.mult(DT));
    }
  }
  update_position() {
    this.position = this.position.add(this.velocity.mult(DT));
  }
  update() {
    this.update_velocity();
    this.update_position();
  }
}

class Body {
  constructor(mass, pos_0, vel_0) {
    this.mass = mass;
    this.position = pos_0;
    this.velocity = vel_0;
  }
  draw() {
    // ctx.strokeStyle = "rgba(255,255,255,0.3)";
    ctx.strokeStyle = "orange";
    ctx.fillStyle = "orange";
    let ctx_coords = get_ctx_coords([this.position.x, this.position.y]);
    let r = get_ctx_radius(3);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], 3 * r, 0, TAU);
    ctx.stroke();
    ctx.fill();
  }
  update_velocity() {
    for (let b of bodies) {
      if (b === this) continue;
      let delta_pos = b.position.sub(this.position);
      let r = delta_pos.norm_l2(); // distance
      let unit_vec = delta_pos.mult(1 / r);
      let acc = unit_vec.mult((G * b.mass) / (r ** 2 + EPSILON ** 2));
      this.velocity = this.velocity.add(acc.mult(DT));
    }
  }
  update_position() {
    this.position = this.position.add(this.velocity.mult(DT));
  }
  update() {
    this.update_velocity();
    this.update_position();
    for (let b of bodies) {
      if (this === b) continue;
      if (this.position.sub(b.position).norm_l2() < min_r_for_merger) {
        this.mass += b.mass;
        this.velocity = new Vector2D(0, 0);
        bodies.splice(bodies.indexOf(b), 1);
        // this.velocity = this.velocity
        //   .mult(this.mass)
        //   .add(b.velocity.mult(b.mass))
        //   .mult(1 / this.mass);
      }
    }
  }
}

class World {
  constructor(world_size) {
    this.width = world_size[0];
    this.height = world_size[1];
    this.origin = new Vector2D(world_size[0] / 2, world_size[1] / 2);
  }
}

// UTILITY FUNCTIONS
// ============================================================================

const reset_time = () => {
  time_step = 0;
  // delivered_food = 0;
};
const get_map_coords = (ctx_coords) => {
  const map_coord_x = (ctx_coords[0] / W) * world.width;
  const map_coord_y = (ctx_coords[1] / H) * world.height;
  return [map_coord_x, map_coord_y];
};
const get_ctx_coords = (map_coords) => {
  const ctx_coord_x = (map_coords[0] / world.width) * W;
  const ctx_coord_y = (map_coords[1] / world.height) * H;
  return [ctx_coord_x, ctx_coord_y];
};
const get_ctx_radius = (radius) => {
  return (radius / world.width) * W; // TODO: only for H=W
};
//// const getCursorPosition = (canvas, event) => {
////   const rect = canvas.getBoundingClientRect();
////   const x = event.clientX - rect.left;
////   const y = event.clientY - rect.top;
////   return [x, y];
//// };
//// //const argmax = (arr) => {
//// //  if (arr.length === 0) {
//// //    return -1;
//// //  }

//// //  var max = arr[0];
//// //  var maxIndex = 0;

//// //  for (var i = 1; i < arr.length; i++) {
//// //    if (arr[i] > max) {
//// //      maxIndex = i;
//// //      max = arr[i];
//// //    }
//// //  }

//// //  return maxIndex;
//// //};
//// //const min_from_1D_array = (arr) => {
//// //  return arr.reduce(function (a, b) {
//// //    return Math.min(a, b);
//// //  });
//// //};
//// //const max_from_1D_array = (arr) => {
//// //  return arr.reduce(function (a, b) {
//// //    return Math.max(a, b);
//// //  });
//// //};
//// //const min_from_2D_array = (arr) => {
//// //  var values = arr.map(function (elt) {
//// //    return elt[1];
//// //  });
//// //  return Math.min.apply(null, values);
//// //};
//// //const max_from_2D_array = (arr) => {
//// //  var values = arr.map(function (elt) {
//// //    return elt[1];
//// //  });
//// //  return Math.max.apply(null, values);
//// //};
//// //const sigmoid = (x) => {
//// //  return Math.exp(x) / (Math.exp(x) + 1);
//// //};
//// //const round = (num, acc) => {
//// //  return Math.round((num + Number.EPSILON) * 10 ** acc) / 10 ** acc;
//// //};
//// //const add_info_text = () => {
//// //  ctx.font = "30px Arial";
//// //  ctx.fillText("t = " + time_step, 10, 50);
//// //};
//// //const sleep = (ms) => {
//// //  return new Promise((resolve) => setTimeout(resolve, ms));
//// //};
//// //const sleepFor = (ms) => {
//// //  var now = new Date().getTime();
//// //  while (new Date().getTime() < now + ms) {
//// //    /* do nothing */
//// //  }
//// //};
//// //const remove_from_array = (arr, item) => {
//// //  const index = arr.indexOf(item);
//// //  if (index > -1) {
//// //    arr.splice(index, 1);
//// //  }
//// //};
//// //const mean = (arr) => {
//// //  var sum = 0;
//// //  for (let i of arr) {
//// //    sum += i;
//// //  }
//// //  return sum / arr.length;
//// //};
const add_event_listeners = () => {
  //  //   // canvas.addEventListener("keypress", function(e) {
  //  //   //   var keynum;
  //  //   //   if (window.event) { // I.E.
  //  //   //     keynum = e.keyCode;
  //  //   //   } else if (e.which) { // Netscape/Firefox/Opera
  //  //   //     keynum = e.which;
  //  //   //   }
  //  //   //   alert(String.fromCharCode(keynum))
  //  //   // })
  //  //    canvas.addEventListener("mousemove", function (e) {
  //  //      const ctx_coords = getCursorPosition(canvas, e);
  //  //      const map_coords = get_map_coords(ctx_coords);
  //  //      const x = map_coords[0]
  //  //      const y = map_coords[1]
  //  //      if (x > 0 && x < world.width && y > 0 && y < world.height) {
  //  //        let mouse_pos = new Vector2D(x, y)
  //  //        predators[0].follow_mouse(mouse_pos)
  //  //      }
  //  //    })
  //  //    // canvas.addEventListener("mousedown", function (e) {
  //  //   //    const ctx_coords = getCursorPosition(canvas, e);
  //  //   //    const map_coords = get_map_coords(ctx_coords);
  //  //   //    const col_idx = Math.floor(map_coords[0]);
  //  //   //    const row_idx = Math.floor(map_coords[1]);
  //  //   //    if (placement_select === "food") {
  //  //   //      world.food_sources[row_idx][col_idx] += food_placement_amount;
  //  //   //    } else if (placement_select === "phA") {
  //  //   //      world.pheromone_strengths[0][row_idx][col_idx] += 100;
  //  //   //      world.active_grid_cells.push([row_idx, col_idx]);
  //  //   //    } else if (placement_select === "phB") {
  //  //   //      world.pheromone_strengths[1][row_idx][col_idx] += 100;
  //  //   //      world.active_grid_cells.push([row_idx, col_idx]);
  //  //   //    } else if (placement_select === "walls") {
  //  //   //      for (let i = row_idx - 1; i <= row_idx + 1; i++) {
  //  //   //        for (let j = col_idx - 1; j <= col_idx + 1; j++) {
  //  //   //          try {
  //  //   //            world.walls[i][j] = 1
  //  //   //          } finally {}
  //  //   //        }
  //  //   //      }
  //  //   //    } else if (placement_select === "remove_walls") {
  //  //   //      for (let i = row_idx - 1; i <= row_idx + 1; i++) {
  //  //   //        for (let j = col_idx - 1; j <= col_idx + 1; j++) {
  //  //   //          try {
  //  //   //            world.walls[i][j] = 0
  //  //   //          } finally {}
  //  //   //        }
  //  //   //      }
  //  //   //    }
  //  //   //    // console.log(ctx_coords, [col_idx, row_idx]);
  //  //   //  });
  // BUTTONS
  document
    .getElementById("button_toggle_random_phi_distribution")
    .addEventListener("click", function () {
      belt_random_phi_distribution = !belt_random_phi_distribution;
      console.log(
        "toggled drawing of random phi distribution,",
        belt_random_phi_distribution
      );
      init();
    });
  //  //   document
  //  //     .getElementById("button_display_attraction_radius")
  //  //     .addEventListener("click", function () {
  //  //       bool_draw_attraction_radius = !bool_draw_attraction_radius;
  //  //       console.log("toggled drawing of attraction radius");
  //  //     });
  //  //   document
  //  //     .getElementById("button_display_cohesion_radius")
  //  //     .addEventListener("click", function () {
  //  //       bool_draw_cohesion_radius = !bool_draw_cohesion_radius;
  //  //       console.log("toggled drawing of cohesion radius");
  //  //     });
  //  //   document
  //  //     .getElementById("button_toggle_pause")
  //  //     .addEventListener("click", function () {
  //  //       paused = !paused;
  //  //       console.log("toggled pause");
  //  //     });
  //  //   document
  //  //     .getElementById("button_toggle_show_trajectories")
  //  //     .addEventListener("click", function () {
  //  //       bool_show_trajectories = !bool_show_trajectories;
  //  //       console.log("toggled showing of trajectories");
  //  //     });
  //  //   document
  //  //     .getElementById("button_toggle_use_quad_tree")
  //  //     .addEventListener("click", function () {
  //  //       use_quad_tree = !use_quad_tree;
  //  //       console.log("toggled usage of quad tree");
  //  //     });
  //  //   document
  //  //     .getElementById("button_toggle_show_quad_tree_grid")
  //  //     .addEventListener("click", function () {
  //  //       bool_show_quad_tree_grid = !bool_show_quad_tree_grid;
  //  //       console.log("toggled showing of quad tree grid");
  //  //     });
  //  //   // document
  //  //   //   .getElementById("button_toggle_periodic_bounds")
  //  //   //   .addEventListener("click", function () {
  //  //   //     periodic_bounds = !periodic_bounds;
  //  //   //     console.log("toggled periodic bounds");
  //  //   //   });
  //  //   // document
  //  //   //   .getElementById("button_toggle_display_sensor_radius")
  //  //   //   .addEventListener("click", function () {
  //  //   //     bool_draw_boid_sensor_radius = !bool_draw_boid_sensor_radius;
  //  //   //     console.log("toggled drawing of boid sensor radius");
  //  //   //   });
  //  //   // document
  //  //   //   .getElementById("button_toggle_display_collision_radius")
  //  //   //   .addEventListener("click", function () {
  //  //   //     bool_draw_boid_collision_radius = !bool_draw_boid_collision_radius;
  //  //   //     console.log("toggled drawing of boid collision radius");
  //  //   //   });
  //  //   // document
  //  //   //   .getElementById("button_toggle_display_velocity_vector")
  //  //   //   .addEventListener("click", function () {
  //  //   //     bool_draw_boid_velocity_vectors = !bool_draw_boid_velocity_vectors;
  //  //   //     console.log("toggled drawing of boid velocity vectors");
  //  //   //   });
  // SLIDERS
  document
    .getElementById("slider_belt_min_r")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_belt_min_r").value;
      belt_min_r = value;
      console.log("new belt min r: ", value);
      init();
    });
  document
    .getElementById("slider_belt_width")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_belt_width").value;
      belt_width = value;
      console.log("new belt width: ", value);
      init();
    });
  document
    .getElementById("slider_belt_exp_factor")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_belt_exp_factor").value;
      belt_exp_factor = value;
      console.log("new belt exp factor: ", value);
      init();
    });
  //  //   document
  //  //     .getElementById("slider_attraction_radius")
  //  //     .addEventListener("click", function () {
  //  //       let value = document.getElementById("slider_attraction_radius").value;
  //  //       attraction_radius = (value / 1000) * world.width; // TODO: only for W=H
  //  //       console.log("new boid attraction radius: ", attraction_radius);
  //  //     });
  //  //   document
  //  //     .getElementById("slider_cohesion_radius")
  //  //     .addEventListener("click", function () {
  //  //       let value = document.getElementById("slider_cohesion_radius").value;
  //  //       cohesion_radius = (value / 1000) * world.width; // TODO: only for W=H
  //  //       console.log("new boid cohesion radius: ", cohesion_radius);
  //  //     });
  //  //   // sliders for force strengths
  //  //   document
  //  //     .getElementById("slider_avoidance_strength")
  //  //     .addEventListener("click", function () {
  //  //       let value = document.getElementById("slider_avoidance_strength").value;
  //  //       avoidance_force = value / 100;
  //  //       console.log("new boid avoidance force: ", avoidance_force);
  //  //     });
  //  //   document
  //  //     .getElementById("slider_attraction_strength")
  //  //     .addEventListener("click", function () {
  //  //       let value = document.getElementById("slider_attraction_strength").value;
  //  //       attraction_force = value / 100;
  //  //       console.log("new boid attraction strength: ", attraction_force);
  //  //     });
  //  //   document
  //  //     .getElementById("slider_cohesion_strength")
  //  //     .addEventListener("click", function () {
  //  //       let value = document.getElementById("slider_cohesion_strength").value;
  //  //       cohesion_force = value / 100;
  //  //       console.log("new boid cohesion strength: ", cohesion_force);
  //  //     });
  //  //   // other sliders
  //  //   // document
  //  //   //   .getElementById("slider_flock_size")
  //  //   //   .addEventListener("click", function () {
  //  //   //     flock_size = document.getElementById("slider_flock_size").value;
  //  //   //     console.log("new flock size: ", flock_size);
  //  //   //     init();
  //  //   //  });
  //  //   // document
  //  //   //   .getElementById("slider_collision_radius")
  //  //   //   .addEventListener("click", function () {
  //  //   //     let value = document.getElementById("slider_collision_radius").value;
  //  //   //     boid_collision_radius = (value / 5000) * world.width; // TODO: only for W=H
  //  //   //     console.log("new boid collision radius: ", boid_collision_radius);
  //  //   //     init();
  //  //   //   });
  //  //   //  document
  //  //   //    .getElementById("button_reset")
  //  //   //    .addEventListener("click", function () {
  //  //   //      // animate()
  //  //   //      init();
  //  //   //    });
  //  //   //  document
  //  //   //    .getElementById("button_place_food")
  //  //   //    .addEventListener("click", function () {
  //  //   //      placement_select = "food";
  //  //   //    });
  //  //   //  document
  //  //   //    .getElementById("button_place_walls")
  //  //   //    .addEventListener("click", function () {
  //  //   //      placement_select = "walls";
  //  //   //    });
  //  //   //  document
  //  //   //    .getElementById("button_remove_walls")
  //  //   //    .addEventListener("click", function () {
  //  //   //      placement_select = "remove_walls";
  //  //   //    });
  //  //   //  document
  //  //   //    .getElementById("button_place_phA")
  //  //   //    .addEventListener("click", function () {
  //  //   //      placement_select = "phA";
  //  //   //    });
  //  //   //  document
  //  //   //    .getElementById("button_place_phB")
  //  //   //    .addEventListener("click", function () {
  //  //   //      placement_select = "phB";
  //  //   //    });
  //  //   //  document
  //  //   //    .getElementById("button_draw_pheromones")
  //  //   //    .addEventListener("click", function () {
  //  //   //      bool_draw_pheromones = !bool_draw_pheromones;
  //  //   //    });
  //  //   //  document
  //  //   //    .getElementById("button_periodic_bounds")
  //  //   //    .addEventListener("click", function () {
  //  //   //      periodic_bounds = !periodic_bounds;
  //  //   //    });
};

// INITIALIZATION
// ============================================================================

const init = () => {
  console.log("initializing...");
  // setup canvas
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");

  // setup world
  world = new World(world_size);

  // setup bodies (big boys)
  bodies = [];
  for (let idx = 0; idx < 2; idx++) {
    let m = 1000;
    let pos_0 = new Vector2D(50, idx * Math.PI, "pol");
    pos_0 = pos_0.add(world.origin);
    let speed = Math.sqrt((G * m) / 200);
    let vel_0 = new Vector2D(speed, idx * Math.PI + Math.PI / 2, "pol");
    let body = new Body(m, pos_0, vel_0);
    bodies.push(body);
  }

  // setup asteroids
  asteroids = [];
  for (let idx = 0; idx < nr_of_asteroids; idx++) {
    let m = 1;
    let phi;
    if (belt_random_phi_distribution) {
      phi = TAU * Math.random();
    } else {
      phi = (TAU * idx) / nr_of_asteroids;
    }
    let pos_0 = new Vector2D(
      belt_min_r + belt_width * Math.exp(-belt_exp_factor * Math.random()),
      phi,
      "pol"
    );
    let r = pos_0.norm_l2();
    let M = bodies[0].mass + bodies[1].mass; // TODO: generalize?
    let speed = Math.sqrt((G * M) / r); //* (2 * Math.random() - 1);
    let vel_0 = pos_0.rotate(TAU / 4).mult(speed / pos_0.norm_l2());
    let asteroid = new Particle(m, pos_0.add(world.origin), vel_0);
    asteroids.push(asteroid);
  }

  document.getElementById("slider_belt_width").value = belt_width;
  document.getElementById("slider_belt_min_r").value = belt_min_r;
  document.getElementById("slider_belt_exp_factor").value = belt_exp_factor;

  // reset time
  reset_time();
  // add_event_listeners
  add_event_listeners();
};

// ANIMATION LOOP
// ============================================================================

async function animate() {
  // create animation loop
  requestAnimationFrame(animate);
  if (paused) {
    return;
  }
  // Erase whole canvas
  if (!bool_show_trajectories) ctx.clearRect(0, 0, W, H);
  // update particles
  for (let b of bodies) {
    b.update();
    b.draw();
  }
  for (let a of asteroids) {
    a.update();
    a.draw();
  }
  // console.log(asteroids[0].position);
  // show quad tree grid
  // if (bool_show_quad_tree_grid) quad_tree.show();
  // increment time
  time_step += 1;
}

init();
animate();
