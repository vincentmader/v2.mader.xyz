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
var DT = 2; // TODO: make changeable
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
var paused = false; // TODO: add restart button
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
    ctx.strokeStyle = "white";
    ctx.fillStyle = "white";
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
const add_event_listeners = () => {
  // BUTTONS
  document
    .getElementById("button_toggle_random_phi_distribution")
    .addEventListener("click", function () {
      belt_random_phi_distribution = !belt_random_phi_distribution;
      // console.log(
      //   "toggled drawing of random phi distribution,",
      //   belt_random_phi_distribution
      // );
      init();
    });
  document
    .getElementById("button_toggle_pause")
    .addEventListener("click", function () {
      paused = !paused;
      console.log(paused);
      if (paused) {
        document.getElementById("button_toggle_pause").innerHTML = "unpause";
      } else {
        document.getElementById("button_toggle_pause").innerHTML = "pause";
      }
    });
  // SLIDERS
  document
    .getElementById("slider_belt_min_r")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_belt_min_r").value;
      belt_min_r = Number(value);
      console.log("new belt min r: ", value);
      init();
    });
  document
    .getElementById("slider_belt_width")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_belt_width").value;
      belt_width = Number(value);
      console.log("new belt width: ", value);
      init();
    });
  document
    .getElementById("slider_belt_exp_factor")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_belt_exp_factor").value;
      belt_exp_factor = Number(value);
      console.log("new belt exp factor: ", value);
      init();
    });
};

// ANIMATION LOOP
// ============================================================================

async function animate() {
  setInterval(function () {
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
  }, 1000 / 60);
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

  document.getElementById("slider_belt_width").value = belt_width;
  document.getElementById("slider_belt_min_r").value = belt_min_r;
  document.getElementById("slider_belt_exp_factor").value = belt_exp_factor;

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

  // reset time
  reset_time();
  animate();
};

init();
// add_event_listeners
add_event_listeners();
