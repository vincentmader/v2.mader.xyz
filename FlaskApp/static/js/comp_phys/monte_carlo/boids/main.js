// Boids
// Vincent C. Mader

// IMPORTS
// ============================================================================

import { Vector2D } from "../../../utils/math_utils.js";
import { Point } from "../../../utils/math_utils.js";
import { Rectangle } from "../../../utils/math_utils.js";

// VARIABLE DEFINITIONS
// ============================================================================

// numerical parameters
var flock_size = 700; // nr of boids in system
var boid_collision_radius = 4; // TODO: make changeable
var use_quad_tree = true;
var quad_tree_capacity = 10;
// sensor radii
var avoidance_radius = 20;
var attraction_radius = 300;
var cohesion_radius = 200;
var evasion_radius = 500;
// forces
var avoidance_force = 0.5;
var attraction_force = 0.05;
var cohesion_force = 1;
//
var DT = 1; // TODO: make changeable
var initial_boid_speed = 5; // TODO: make changeable
var initial_predator_speed = 5; // TODO: make changeable
var probability_for_random_boid_turn = 1; // TODO: make changeable
var max_random_turn_angle = Math.PI / 10; // TODO: make changeable

// world parameters
const world_size = [1200, 1200];

const nr_of_predators = 1;
var quadtree;

// button presets
var bool_draw_avoidance_radius = false;
var bool_draw_attraction_radius = false;
var bool_draw_cohesion_radius = false;
var bool_show_trajectories = false;
var bool_show_quad_tree_grid = false;
// var bool_draw_boid_sensor_radius = false;
// var bool_draw_boid_collision_radius = false;
var bool_draw_boid_velocity_vectors = false;
var paused = false;
var periodic_bounds = true;

// draw settings
var boid_drawing_radius = 2;
var canvas, ctx, W, H;

// world & boids
var world, flock, predators;

// stats
var time_step, delivered_food, fps;
//var fps_values = [];

// constants
const TAU = 2 * Math.PI;

// CLASS DEFINITIONS

class QuadTree {
  constructor(boundary, n) {
    this.boundary = boundary;
    this.capacity = n;
    this.points = [];
    this.divided = false;
  }
  subdivide() {
    let x = this.boundary.x;
    let y = this.boundary.y;
    let w = this.boundary.w;
    let h = this.boundary.h;
    // create rectangle objects representing children
    let ne = new Rectangle(x + w / 2, y - h / 2, w / 2, h / 2);
    let nw = new Rectangle(x - w / 2, y - h / 2, w / 2, h / 2);
    let se = new Rectangle(x + w / 2, y + h / 2, w / 2, h / 2);
    let sw = new Rectangle(x - w / 2, y + h / 2, w / 2, h / 2);
    // create quad tree objects
    this.northeast = new QuadTree(ne, this.capacity);
    this.northwest = new QuadTree(nw, this.capacity);
    this.southeast = new QuadTree(se, this.capacity);
    this.southwest = new QuadTree(sw, this.capacity);
    this.divided = true;
    // insert points into children
    for (let p of this.points) {
      this.northeast.insert(p);
      this.northwest.insert(p);
      this.southeast.insert(p);
      this.southwest.insert(p);
    }
  }
  insert(point) {
    if (!this.boundary.contains(point)) return;
    if (this.points.length < this.capacity) {
      this.points.push(point);
    } else {
      if (!this.divided) {
        this.subdivide();
      }
      this.northeast.insert(point);
      this.northwest.insert(point);
      this.southeast.insert(point);
      this.southwest.insert(point);
    }
  }
  show() {
    ctx.lineWidth = 1;
    ctx.strokeStyle = "gray";
    ctx.beginPath();
    let ctx_coords = get_ctx_coords([
      this.boundary.x - this.boundary.w,
      this.boundary.y - this.boundary.h,
    ]);
    let ctx_w = get_ctx_radius(2 * this.boundary.w);
    let ctx_h = get_ctx_radius(2 * this.boundary.h);
    ctx.rect(ctx_coords[0], ctx_coords[1], ctx_w, ctx_h);
    ctx.stroke();

    if (this.divided) {
      this.northeast.show();
      this.northwest.show();
      this.southeast.show();
      this.southwest.show();
    }
    // for (let p of this.points) {
    //   ctx.strokeStyle = "white";
    //   ctx.fillStyle = "white";
    //   ctx.beginPath();
    //   ctx.arc(p.x, p.y, 1, 0, TAU);
    //   ctx.stroke();
    //   ctx.fill();
    // }
  }
  query(range, found) {
    if (!this.boundary.intersects(range)) {
      return;
    } else {
      for (let p of this.points) {
        if (range.contains(p) && !found.includes(p)) {
          found.push(p);
        }
      }
      if (this.divided) {
        this.northwest.query(range, found);
        this.northeast.query(range, found);
        this.southwest.query(range, found);
        this.southeast.query(range, found);
      }
    }
  }
}

class Boid {
  constructor(spawn_position, initial_speed, initial_rotation) {
    this.position = spawn_position;
    this.speed = initial_speed;

    let u = initial_speed * Math.cos(initial_rotation);
    let v = initial_speed * Math.sin(initial_rotation);
    this.velocity = new Vector2D(u, v);

    this.collision_radius = boid_collision_radius;
    this.avoidance_force = avoidance_force;
  }
  apply_avoidance(possible_neighbors) {
    // find list idx of closest boid
    let distance_to_closest_boid = 10000; // TODO
    let idx_of_closest_boid = -1;
    for (let idx = 0; idx < possible_neighbors.length; idx++) {
      let boid = flock.boids[idx];
      if (boid === this) continue;
      let distance = boid.position.sub(this.position).norm_l2();
      if (distance > avoidance_radius) continue;
      let angle =
        boid.position.sub(this.position).angle() - this.velocity.angle();
      if (angle < -Math.PI / 2 || angle > Math.PI / 2) continue;
      if (distance < distance_to_closest_boid) {
        distance_to_closest_boid = distance;
        idx_of_closest_boid = idx;
      }
    }
    // do nothing if no other boids are present within sensor radius
    if (idx_of_closest_boid == -1) return;
    // check for collision with closest boid
    let closest_boid = flock.boids[idx_of_closest_boid];
    let collision_detected = false;
    var ahead;
    // TODO: implement faster way of checking for line-circle intersection
    let speed = this.velocity.norm_l2();
    for (let lambda of [0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1]) {
      ahead = this.position.add(this.velocity.mult(speed * lambda));
      let distance_from_ahead = ahead.sub(closest_boid.position).norm_l2();
      // console.log(distance_from_ahead);
      if (
        distance_from_ahead <
        closest_boid.collision_radius + this.collision_radius
      ) {
        collision_detected = true;
        break;
      }
    }
    // continue if there are no collisions on present trajectory
    if (!collision_detected) return;
    // TODO: choose random point, check for collision, repeat until empty spot found
    // apply repulsion force to velocity
    let force = ahead.sub(closest_boid.position);
    let force_norm = force.norm_l2();
    force = force.mult(this.avoidance_force / force_norm);
    this.velocity = this.velocity.add(force);
    // renormalize
    this.velocity = this.velocity.mult(this.speed / this.velocity.norm_l2());
  }
  apply_attraction(possible_neighbors) {
    var center_of_mass = 0;
    var boids_in_local_flock = 0;
    for (let idx = 0; idx < possible_neighbors.length; idx++) {
      let boid = flock.boids[idx];
      if (boid === this) continue;
      let distance = boid.position.sub(this.position).norm_l2();
      if (distance > attraction_radius) continue;
      let angle =
        boid.position.sub(this.position).angle() - this.velocity.angle();
      if (angle < -Math.PI / 2 || angle > Math.PI / 2) continue;
      // determine center of mass of local boid flock (inside sensor radius)
      if (center_of_mass === 0) {
        center_of_mass = boid.position;
        boids_in_local_flock = 1;
      } else {
        center_of_mass = center_of_mass.add(boid.position);
        boids_in_local_flock += 1;
      }
    }
    if (boids_in_local_flock === 0) return;
    center_of_mass = center_of_mass.mult(1 / boids_in_local_flock);

    let force = center_of_mass.sub(this.position);
    let force_norm = force.norm_l2();
    this.velocity = this.velocity.add(
      force.mult((1 / force_norm) * attraction_force)
    );
    // renormalize
    this.velocity = this.velocity.mult(this.speed / this.velocity.norm_l2());
  }
  apply_cohesion(possible_neighbors) {
    var average_velocity = 0;
    var boids_in_local_flock = 0;
    for (let idx = 0; idx < possible_neighbors.length; idx++) {
      let boid = flock.boids[idx];
      if (boid === this) continue;
      let distance = boid.position.sub(this.position).norm_l2();
      if (distance > cohesion_radius) continue;
      let angle =
        boid.position.sub(this.position).angle() - this.velocity.angle();
      if (angle < -Math.PI / 2 || angle > Math.PI / 2) continue;
      // determine center of mass of local boid flock (inside sensor radius)
      if (average_velocity === 0) {
        average_velocity = boid.velocity;
        boids_in_local_flock = 1;
      } else {
        average_velocity = average_velocity.add(boid.velocity);
        boids_in_local_flock += 1;
      }
    }
    if (boids_in_local_flock === 0) return;
    average_velocity = average_velocity.mult(1 / boids_in_local_flock);

    let force = average_velocity;
    if (!force) console.log(force);
    let force_norm = force.norm_l2();
    this.velocity = this.velocity.add(
      force.mult((1 / force_norm) * cohesion_force)
    );
    // renormalize
    this.velocity = this.velocity.mult(this.speed / this.velocity.norm_l2());
  }
  apply_evasion() {
    for (let p of predators) {
      // TODO: what radius?
      if (p.position.sub(this.position).norm_l2() < evasion_radius) {
        let force = p.position.sub(this.position);
        let force_norm = force.norm_l2();
        this.velocity = this.velocity.sub(
          force.mult((1 / force_norm) * avoidance_force)
        );
        this.velocity = this.velocity.mult(
          this.speed / this.velocity.norm_l2()
        );
      }
    }
  }
  apply_random_turns() {
    // TODO: do this every %N ? bottlenecks?
    if (Math.random() < probability_for_random_boid_turn) {
      let turning_angle = (2 * Math.random() - 1) * max_random_turn_angle;
      this.velocity = this.velocity.rotate(turning_angle);
    }
  }
  update_position_values() {
    this.position = this.position.add(this.velocity.mult(DT));
    // apply periodic bounds
    if (periodic_bounds) {
      if (this.position.x > world.width) this.position.x = 0;
      if (this.position.y > world.height) this.position.y = 0;
      if (this.position.x < 0) this.position.x = world.width;
      if (this.position.y < 0) this.position.y = world.height;
    }
  }
  draw() {
    // get canvas coords of boid
    let ctx_coords = get_ctx_coords([this.position.x, this.position.y]);
    // get boid color
    let angle = this.velocity.angle() + Math.PI;
    let color = "hsl(" + ((angle - Math.PI / 2) / TAU) * 360 + ", 100%, 50%)";
    // draw boid
    let ctx_radius = get_ctx_radius(boid_drawing_radius);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
    ctx.fillStyle = color;
    ctx.fill();
    ctx.strokeStyle = color;
    ctx.stroke();

    // draw sensor radii
    if (bool_draw_avoidance_radius && this === flock.boids[0]) {
      let ctx_radius = get_ctx_radius(avoidance_radius);
      ctx.beginPath();
      ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
      ctx.strokeStyle = "red";
      ctx.lineWidth = 3;
      ctx.stroke();
    }
    if (bool_draw_attraction_radius && this === flock.boids[0]) {
      let ctx_radius = get_ctx_radius(attraction_radius);
      ctx.beginPath();
      ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
      ctx.strokeStyle = "green";
      ctx.lineWidth = 3;
      ctx.stroke();
    }
    if (bool_draw_cohesion_radius && this === flock.boids[0]) {
      let ctx_radius = get_ctx_radius(cohesion_radius);
      ctx.beginPath();
      ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
      ctx.strokeStyle = "blue";
      ctx.lineWidth = 3;
      ctx.stroke();
    }
    ctx.lineWidth = 1;

    // // draw collision radius
    // if (bool_draw_boid_collision_radius) {
    //   ctx.strokeStyle = "white";
    //   let ctx_radius = get_ctx_radius(boid_collision_radius);
    //   ctx.beginPath();
    //   ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
    //   ctx.stroke();
    // }
    // draw velocity vector
    // if (bool_draw_boid_velocity_vectors) {
    if (true) {
      ctx.strokeStyle = color;
      ctx.beginPath();
      let ctx_coords = get_ctx_coords([this.position.x, this.position.y]);
      ctx.moveTo(ctx_coords[0], ctx_coords[1]);
      // ctx_coords = get_ctx_coords([
      //   this.position.x +
      //     (this.velocity.x / this.velocity.norm_l2()) * this.sensor_radius,
      //   this.position.y +
      //     (this.velocity.y / this.velocity.norm_l2()) * this.sensor_radius,
      // ]);
      ctx_coords = get_ctx_coords([
        this.position.x + (this.velocity.x / this.velocity.norm_l2()) * 5,
        this.position.y + (this.velocity.y / this.velocity.norm_l2()) * 5,
      ]);
      ctx.lineTo(ctx_coords[0], ctx_coords[1]);
      ctx.stroke();
    }
  }
  update() {
    var possible_neighbors = [];
    if (use_quad_tree) {
      let x = this.position.x;
      let y = this.position.y;
      let w = Math.max(attraction_radius, cohesion_radius);
      let h = w;
      let range = new Rectangle(x, y, w, h);

      // ctx.strokeStyle = 'white'
      // ctx.beginPath();
      // let ctx_coords = get_ctx_coords([range.x, range.y])
      // let ctx_w = get_ctx_radius(range.w)
      // let ctx_h = get_ctx_radius(range.h)
      // ctx.rect(ctx_coords[0]-ctx_w/2, ctx_coords[1]-ctx_h/2, 2*ctx_w, 2*ctx_h)
      // ctx.stroke()

      let points = [];
      quadtree.query(range, points);
      for (let p of points) {
        possible_neighbors.push(flock.boids_from_loc[[p.x, p.y]]);
        // console.log(possible_neighbors.length)
      }
    } else {
      possible_neighbors = flock.boids;
    }

    this.apply_attraction(possible_neighbors);
    this.apply_cohesion(possible_neighbors);
    this.apply_random_turns();
    this.apply_avoidance(possible_neighbors);
    this.apply_evasion();
    this.update_position_values();
    this.draw();
  }
}

class Predator {
  constructor(spawn_position, initial_speed, initial_rotation) {
    this.position = spawn_position;
    this.speed = initial_speed;

    let u = initial_speed * Math.cos(initial_rotation);
    let v = initial_speed * Math.sin(initial_rotation);
    this.velocity = new Vector2D(u, v);
  }
  apply_random_turns() {
    // TODO: do this every %N ? bottlenecks?
    if (Math.random() < probability_for_random_boid_turn) {
      let turning_angle = (2 * Math.random() - 1) * max_random_turn_angle;
      this.velocity = this.velocity.rotate(turning_angle);
    }
  }
  update_position_values() {
    this.position = this.position.add(this.velocity.mult(DT));
    // apply periodic bounds
    if (periodic_bounds) {
      if (this.position.x > world.width) this.position.x = 0;
      if (this.position.y > world.height) this.position.y = 0;
      if (this.position.x < 0) this.position.x = world.width;
      if (this.position.y < 0) this.position.y = world.height;
    }
  }
  follow_mouse(mouse_pos) {
    let force = mouse_pos.sub(this.position);
    this.velocity = this.velocity.add(force);
    this.velocity = this.velocity.mult(
      initial_predator_speed / this.velocity.norm_l2()
    );
  }
  draw() {
    // get canvas coords of boid
    let ctx_coords = get_ctx_coords([this.position.x, this.position.y]);
    // get boid color
    // let angle = this.velocity.angle() + Math.PI;
    let color = "gray";
    // let color = "hsl(" + ((angle - Math.PI / 2) / TAU) * 360 + ", 100%, 50%)";
    // draw boid
    let ctx_radius = get_ctx_radius(10 * boid_drawing_radius);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], ctx_radius, 0, TAU);
    ctx.fillStyle = color;
    ctx.fill();
    ctx.strokeStyle = color;
    ctx.stroke();
    // ctx.lineWidth = 1;
    // draw velocity vector
    // if (bool_draw_boid_velocity_vectors) {
    // if (true) {
    //   ctx.strokeStyle = color;
    //   ctx.beginPath();
    //   let ctx_coords = get_ctx_coords([this.position.x, this.position.y]);
    //   ctx.moveTo(ctx_coords[0], ctx_coords[1]);
    //   // ctx_coords = get_ctx_coords([
    //   //   this.position.x +
    //   //     (this.velocity.x / this.velocity.norm_l2()) * this.sensor_radius,
    //   //   this.position.y +
    //   //     (this.velocity.y / this.velocity.norm_l2()) * this.sensor_radius,
    //   // ]);
    //   ctx_coords = get_ctx_coords([
    //     this.position.x + (this.velocity.x / this.velocity.norm_l2()) * 5,
    //     this.position.y + (this.velocity.y / this.velocity.norm_l2()) * 5,
    //   ]);
    //   ctx.lineTo(ctx_coords[0], ctx_coords[1]);
    //   ctx.stroke();
  }
  update() {
    this.update_position_values();
    this.apply_random_turns();
    this.draw();
  }
}

class Flock {
  constructor(flock_size) {
    this.boids = [];
    for (let idx = 0; idx < flock_size; idx++) {
      let spawn_position = new Vector2D(
        world.width * Math.random(),
        world.height * Math.random()
      );
      let initial_speed = initial_boid_speed; // Math.random()
      let initial_rotation = TAU * Math.random();
      let boid = new Boid(spawn_position, initial_speed, initial_rotation);
      this.boids.push(boid);
    }
  }
  update() {
    this.boids_from_loc = {};
    // define quadtree if necessary
    let n = quad_tree_capacity;
    let boundary = new Rectangle(
      world.width / 2,
      world.height / 2,
      world.width / 2,
      world.height / 2
    );
    quadtree = new QuadTree(boundary, n);
    for (let boid of this.boids) {
      let pos = boid.position;
      this.boids_from_loc[[pos.x, pos.y]] = boid; // TODO: better idea?
      quadtree.insert(pos);
    }
    // loop over boids & update
    for (let boid of this.boids) {
      boid.update();
    }
  }
}

class World {
  constructor(world_size) {
    this.width = world_size[0];
    this.height = world_size[1];
  }
  //  draw_walls() {
  //    for (let idx = 0; idx < this.height; idx++) {
  //      for (let jdx = 0; jdx < this.width; jdx++) {
  //        if (this.walls[idx][jdx] == 1) {
  //          let ctx_coords = get_ctx_coords([jdx, idx])
  //          let ctx_radius = get_ctx_radius(1)
  //          ctx.strokeStyle = "white"
  //          ctx.strokeRect(ctx_coords[0], ctx_coords[1], ctx_radius, ctx_radius)
  //        }
  //      }
  //    }
  //  has_wall_at_position(position) {
  //    // check for world edges in horizontal direction
  //    if (position[0] < 0 || position[0] > this.width) {
  //      return true;
  //    }
  //    // check for world edges in vertical direction
  //    if (position[1] < 0 || position[1] > this.height) {
  //      return true;
  //    }
  //    // check for walls on map
  //    // ...
  //    // else: return false
  //    return false;
  //  }
}

// UTILITY FUNCTIONS
// ============================================================================

const reset_time = () => {
  time_step = 0;
  delivered_food = 0;
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
const getCursorPosition = (canvas, event) => {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  return [x, y];
};
//const argmax = (arr) => {
//  if (arr.length === 0) {
//    return -1;
//  }

//  var max = arr[0];
//  var maxIndex = 0;

//  for (var i = 1; i < arr.length; i++) {
//    if (arr[i] > max) {
//      maxIndex = i;
//      max = arr[i];
//    }
//  }

//  return maxIndex;
//};
//const min_from_1D_array = (arr) => {
//  return arr.reduce(function (a, b) {
//    return Math.min(a, b);
//  });
//};
//const max_from_1D_array = (arr) => {
//  return arr.reduce(function (a, b) {
//    return Math.max(a, b);
//  });
//};
//const min_from_2D_array = (arr) => {
//  var values = arr.map(function (elt) {
//    return elt[1];
//  });
//  return Math.min.apply(null, values);
//};
//const max_from_2D_array = (arr) => {
//  var values = arr.map(function (elt) {
//    return elt[1];
//  });
//  return Math.max.apply(null, values);
//};
//const sigmoid = (x) => {
//  return Math.exp(x) / (Math.exp(x) + 1);
//};
//const round = (num, acc) => {
//  return Math.round((num + Number.EPSILON) * 10 ** acc) / 10 ** acc;
//};
//const add_info_text = () => {
//  ctx.font = "30px Arial";
//  ctx.fillText("t = " + time_step, 10, 50);
//};
//const sleep = (ms) => {
//  return new Promise((resolve) => setTimeout(resolve, ms));
//};
//const sleepFor = (ms) => {
//  var now = new Date().getTime();
//  while (new Date().getTime() < now + ms) {
//    /* do nothing */
//  }
//};
//const remove_from_array = (arr, item) => {
//  const index = arr.indexOf(item);
//  if (index > -1) {
//    arr.splice(index, 1);
//  }
//};
//const mean = (arr) => {
//  var sum = 0;
//  for (let i of arr) {
//    sum += i;
//  }
//  return sum / arr.length;
//};
const add_event_listeners = () => {
  // canvas.addEventListener("keypress", function(e) {
  //   var keynum;
  //   if (window.event) { // I.E.
  //     keynum = e.keyCode;
  //   } else if (e.which) { // Netscape/Firefox/Opera
  //     keynum = e.which;
  //   }
  //   alert(String.fromCharCode(keynum))
  // })
  canvas.addEventListener("mousemove", function (e) {
    const ctx_coords = getCursorPosition(canvas, e);
    const map_coords = get_map_coords(ctx_coords);
    const x = map_coords[0];
    const y = map_coords[1];
    if (x > 0 && x < world.width && y > 0 && y < world.height) {
      let mouse_pos = new Vector2D(x, y);
      predators[0].follow_mouse(mouse_pos);
    }
  });
  // canvas.addEventListener("mousedown", function (e) {
  //    const ctx_coords = getCursorPosition(canvas, e);
  //    const map_coords = get_map_coords(ctx_coords);
  //    const col_idx = Math.floor(map_coords[0]);
  //    const row_idx = Math.floor(map_coords[1]);
  //    if (placement_select === "food") {
  //      world.food_sources[row_idx][col_idx] += food_placement_amount;
  //    } else if (placement_select === "phA") {
  //      world.pheromone_strengths[0][row_idx][col_idx] += 100;
  //      world.active_grid_cells.push([row_idx, col_idx]);
  //    } else if (placement_select === "phB") {
  //      world.pheromone_strengths[1][row_idx][col_idx] += 100;
  //      world.active_grid_cells.push([row_idx, col_idx]);
  //    } else if (placement_select === "walls") {
  //      for (let i = row_idx - 1; i <= row_idx + 1; i++) {
  //        for (let j = col_idx - 1; j <= col_idx + 1; j++) {
  //          try {
  //            world.walls[i][j] = 1
  //          } finally {}
  //        }
  //      }
  //    } else if (placement_select === "remove_walls") {
  //      for (let i = row_idx - 1; i <= row_idx + 1; i++) {
  //        for (let j = col_idx - 1; j <= col_idx + 1; j++) {
  //          try {
  //            world.walls[i][j] = 0
  //          } finally {}
  //        }
  //      }
  //    }
  //    // console.log(ctx_coords, [col_idx, row_idx]);
  //  });

  // BUTTONS
  // buttons for displaying force radii
  document
    .getElementById("button_display_avoidance_radius")
    .addEventListener("click", function () {
      bool_draw_avoidance_radius = !bool_draw_avoidance_radius;
      console.log("toggled drawing of avoidance radius");
    });
  document
    .getElementById("button_display_attraction_radius")
    .addEventListener("click", function () {
      bool_draw_attraction_radius = !bool_draw_attraction_radius;
      console.log("toggled drawing of attraction radius");
    });
  document
    .getElementById("button_display_cohesion_radius")
    .addEventListener("click", function () {
      bool_draw_cohesion_radius = !bool_draw_cohesion_radius;
      console.log("toggled drawing of cohesion radius");
    });
  document
    .getElementById("button_toggle_pause")
    .addEventListener("click", function () {
      paused = !paused;
      console.log("toggled pause");
    });
  document
    .getElementById("button_toggle_show_trajectories")
    .addEventListener("click", function () {
      bool_show_trajectories = !bool_show_trajectories;
      console.log("toggled showing of trajectories");
    });
  document
    .getElementById("button_toggle_use_quad_tree")
    .addEventListener("click", function () {
      use_quad_tree = !use_quad_tree;
      console.log("toggled usage of quad tree");
    });
  document
    .getElementById("button_toggle_show_quad_tree_grid")
    .addEventListener("click", function () {
      bool_show_quad_tree_grid = !bool_show_quad_tree_grid;
      console.log("toggled showing of quad tree grid");
    });
  // document
  //   .getElementById("button_toggle_periodic_bounds")
  //   .addEventListener("click", function () {
  //     periodic_bounds = !periodic_bounds;
  //     console.log("toggled periodic bounds");
  //   });

  // document
  //   .getElementById("button_toggle_display_sensor_radius")
  //   .addEventListener("click", function () {
  //     bool_draw_boid_sensor_radius = !bool_draw_boid_sensor_radius;
  //     console.log("toggled drawing of boid sensor radius");
  //   });
  // document
  //   .getElementById("button_toggle_display_collision_radius")
  //   .addEventListener("click", function () {
  //     bool_draw_boid_collision_radius = !bool_draw_boid_collision_radius;
  //     console.log("toggled drawing of boid collision radius");
  //   });
  // document
  //   .getElementById("button_toggle_display_velocity_vector")
  //   .addEventListener("click", function () {
  //     bool_draw_boid_velocity_vectors = !bool_draw_boid_velocity_vectors;
  //     console.log("toggled drawing of boid velocity vectors");
  //   });

  // SLIDERS
  // sliders for sensor radii
  document
    .getElementById("slider_avoidance_radius")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_avoidance_radius").value;
      avoidance_radius = (value / 1000) * world.width; // TODO: only for W=H
      console.log("new boid avoidance radius: ", avoidance_radius);
    });
  document
    .getElementById("slider_attraction_radius")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_attraction_radius").value;
      attraction_radius = (value / 1000) * world.width; // TODO: only for W=H
      console.log("new boid attraction radius: ", attraction_radius);
    });
  document
    .getElementById("slider_cohesion_radius")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_cohesion_radius").value;
      cohesion_radius = (value / 1000) * world.width; // TODO: only for W=H
      console.log("new boid cohesion radius: ", cohesion_radius);
    });
  // sliders for force strengths
  document
    .getElementById("slider_avoidance_strength")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_avoidance_strength").value;
      avoidance_force = value / 100;
      console.log("new boid avoidance force: ", avoidance_force);
    });
  document
    .getElementById("slider_attraction_strength")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_attraction_strength").value;
      attraction_force = value / 100;
      console.log("new boid attraction strength: ", attraction_force);
    });
  document
    .getElementById("slider_cohesion_strength")
    .addEventListener("click", function () {
      let value = document.getElementById("slider_cohesion_strength").value;
      cohesion_force = value / 100;
      console.log("new boid cohesion strength: ", cohesion_force);
    });
  // other sliders
  // document
  //   .getElementById("slider_flock_size")
  //   .addEventListener("click", function () {
  //     flock_size = document.getElementById("slider_flock_size").value;
  //     console.log("new flock size: ", flock_size);
  //     init();
  //  });
  // document
  //   .getElementById("slider_collision_radius")
  //   .addEventListener("click", function () {
  //     let value = document.getElementById("slider_collision_radius").value;
  //     boid_collision_radius = (value / 5000) * world.width; // TODO: only for W=H
  //     console.log("new boid collision radius: ", boid_collision_radius);
  //     init();
  //   });
  //  document
  //    .getElementById("button_reset")
  //    .addEventListener("click", function () {
  //      // animate()
  //      init();
  //    });
  //  document
  //    .getElementById("button_place_food")
  //    .addEventListener("click", function () {
  //      placement_select = "food";
  //    });
  //  document
  //    .getElementById("button_place_walls")
  //    .addEventListener("click", function () {
  //      placement_select = "walls";
  //    });
  //  document
  //    .getElementById("button_remove_walls")
  //    .addEventListener("click", function () {
  //      placement_select = "remove_walls";
  //    });
  //  document
  //    .getElementById("button_place_phA")
  //    .addEventListener("click", function () {
  //      placement_select = "phA";
  //    });
  //  document
  //    .getElementById("button_place_phB")
  //    .addEventListener("click", function () {
  //      placement_select = "phB";
  //    });
  //  document
  //    .getElementById("button_draw_pheromones")
  //    .addEventListener("click", function () {
  //      bool_draw_pheromones = !bool_draw_pheromones;
  //    });
  //  document
  //    .getElementById("button_periodic_bounds")
  //    .addEventListener("click", function () {
  //      periodic_bounds = !periodic_bounds;
  //    });
};

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
  flock = new Flock(flock_size);
  predators = [];
  for (let idx = 0; idx < nr_of_predators; idx++) {
    let spawn_pos = new Vector2D(
      Math.random() * world.width,
      Math.random() * world.height
    );
    let v0 = initial_predator_speed; // TODO: larger v? variable?
    let phi0 = Math.random() * TAU;
    predators.push(new Predator(spawn_pos, v0, phi0));
  }

  // set values of sliders for sensor radii
  document.getElementById("slider_avoidance_radius").value = avoidance_radius;
  document.getElementById("slider_attraction_radius").value = attraction_radius;
  document.getElementById("slider_cohesion_radius").value = cohesion_radius;
  // set values of sliders for force strengths
  document.getElementById("slider_avoidance_strength").value =
    avoidance_force * 100;
  document.getElementById("slider_attraction_strength").value =
    attraction_force * 100;
  document.getElementById("slider_cohesion_strength").value =
    cohesion_force * 100;

  // reset time
  reset_time();
  // add_event_listeners
  add_event_listeners();
};

// ANIMATION LOOP
// ============================================================================

//var iteration_start_time = new Date();
async function animate() {
  // create animation loop
  requestAnimationFrame(animate);
  if (paused) {
    return;
  }
  // Erase whole canvas
  if (!bool_show_trajectories) ctx.clearRect(0, 0, canvas.width, canvas.height);
  // update predator(s)
  for (let p of predators) {
    p.update();
  }
  // update flock
  flock.update();
  // show quad tree grid
  if (bool_show_quad_tree_grid) quadtree.show();
  // increment time
  time_step += 1;
}

init();
animate();
