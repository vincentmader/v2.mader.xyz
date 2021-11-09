// Interstellar Gas Cloud
// Vincent C. Mader

// IMPORTS
// ============================================================================

import { Vector2D } from "../../utils/math_utils.js";
import { Rectangle } from "../../utils/math_utils.js";

// VARIABLE DEFINITIONS
// ============================================================================

// numerical parameters
var cloud_size = 100; // nr of particles
var use_quad_tree = true; // else: direct summation, O(N^2)
var quad_tree_capacity = 1;
var max_opening_angle = Math.PI / 20;
var DT = 1; // TODO: make changeable
var EPSILON = 10;
var max_mass = 1;
var use_leapfrog = true; // else: Euler

// // world parameters
const world_size = [600, 600];
var quad_tree;

// // button presets
var paused = false;

var bool_draw_trajectories = false;
var bool_draw_vel_vecs = false
var bool_draw_acc_vecs = false

var bool_draw_all_qt_nodes = false;
var bool_draw_active_qt_nodes = false;
var bool_draw_qt_branches = false;

// draw settings
var canvas, ctx, W, H;

// world & cloud
var world, cloud;

// stats
var time_step;
// var fps;
// var fps_values = [];

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
    if (this == cloud.particles[50]) {
      ctx.strokeStyle = "red";
      ctx.fillStyle = "red";
    } else {
      ctx.strokeStyle = "green";
      ctx.fillStyle = "green";
    }
    let ctx_coords = get_ctx_coords([this.position.x, this.position.y]);
    let r = get_ctx_radius(this.mass);
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], 3 * r, 0, TAU);
    ctx.stroke();
    ctx.fill();

    if (bool_draw_vel_vecs) {
      let pos = this.position
      let ctx_coords = get_ctx_coords([pos.x, pos.y]);
      let new_pos = pos.add(this.velocity.mult(DT * 100))
      let new_ctx_coords = get_ctx_coords([new_pos.x, new_pos.y]);
      ctx.strokeStyle = 'red'
      ctx.beginPath()
      ctx.moveTo(ctx_coords[0], ctx_coords[1])
      ctx.lineTo(new_ctx_coords[0], new_ctx_coords[1])
      ctx.stroke()
    }
  }
  update() {
    // 1. Create list of neighboring particles/clouds
    let neighbors;
    if (use_quad_tree) {
      neighbors = [];
      neighbors = this.create_macro_particles(quad_tree, neighbors);
    } else {
      neighbors = cloud.particles;
    }
    // 2. Integrate (leapfrog or Euler)
    if (use_leapfrog) {
      // apply Leapfrog integration to get new position & velocity
      // get current position, velocity & acceleration
      // j = i + 1/2, k = i + 1
      let x_i = this.position;
      let v_i = this.velocity;
      let a_i = new Vector2D(0, 0);
      for (let p of neighbors) {
        if (this === p) continue;
        let delta_pos = p.position.sub(this.position);
        let distance = delta_pos.norm_l2();
        let unit_vec = delta_pos.mult(1 / distance);
        let acc = unit_vec.mult((G * p.mass) / (distance ** 2 + EPSILON ** 2));
        a_i = a_i.add(acc);

        if (this === cloud.particles[50]) {
          let pos = this.position
          let ctx_coords = get_ctx_coords([pos.x, pos.y]);
          let new_pos = p.position
          let new_ctx_coords = get_ctx_coords([new_pos.x, new_pos.y]);
          ctx.strokeStyle = 'red'
          ctx.beginPath()
          ctx.moveTo(ctx_coords[0], ctx_coords[1])
          ctx.lineTo(new_ctx_coords[0], new_ctx_coords[1])
          ctx.stroke()
        }
      }
      // get next position
      let v_j = v_i.add(a_i.mult(DT / 2));
      let x_k = x_i.add(v_j.mult(DT));
      this.position = x_k;
      // get next velocity
      let a_k = new Vector2D(0, 0); // TODO: try also with a_i (worse?)
      // for (let p of neighbors) {
      //   if (this === p) continue;
      //   let delta_pos = p.position.sub(this.position);
      //   let distance = delta_pos.norm_l2();
      //   let unit_vec = delta_pos.mult(1 / distance);
      //   let acc = unit_vec.mult((G * p.mass) / (distance ** 2 + EPSILON ** 2));
      //   a_k = a_k.add(acc);
      // }
      let v_k = v_j.add(a_i.mult(DT / 2)); // should be done with a_k
      this.velocity = v_k;
      // console.log(a_i.dot(x_i))

      if (bool_draw_acc_vecs) {
        let pos = this.position
        let ctx_coords = get_ctx_coords([pos.x, pos.y]);
        let new_pos = pos.add(a_i.mult(DT * 10000))
        let new_ctx_coords = get_ctx_coords([new_pos.x, new_pos.y]);
        ctx.strokeStyle = 'yellow'
        ctx.beginPath()
        ctx.moveTo(ctx_coords[0], ctx_coords[1])
        ctx.lineTo(new_ctx_coords[0], new_ctx_coords[1])
        ctx.stroke()
      }
    } else {
      let x_i = this.position;
      let v_i = this.velocity;
      let a_i = new Vector2D(0, 0);
      for (let p of neighbors) {
        if (this === p) continue;
        let delta_pos = p.position.sub(this.position);
        let distance = delta_pos.norm_l2();
        let unit_vec = delta_pos.mult(1 / distance);
        let acc = unit_vec.mult((G * p.mass) / (distance ** 2 + EPSILON ** 2));
        a_i = a_i.add(acc);
      }
      let v_k = v_i.add(a_i.mult(DT));
      let x_k = x_i.add(v_i.mult(DT));
      this.velocity = v_k;
      this.position = x_k;
    }
  }
  create_macro_particles(qt, macro_particles) {
    if (!qt.northwest) {
      let macro_particle = cloud.get_macro_particle(qt, 1000, this); // TODO: better way?
      if (macro_particle) macro_particles.push(macro_particle);
      return macro_particles;
    } else {
      let divs = [qt.northwest, qt.northeast, qt.southwest, qt.southeast];
      for (let node of divs) {
        let x = node.boundary.x;
        let y = node.boundary.y;
        let w = node.boundary.w;
        let h = node.boundary.h;
        // calculate opening angle of node as seen from particle
        let dist = Math.sqrt(
          (x - this.position.x) ** 2 + (y - this.position.y) ** 2
        );
        let theta = Math.asin(w / dist);
        while (theta < 0) theta += TAU;
        while (theta > TAU) theta -= TAU;
        if (theta > max_opening_angle || !theta) {
          // if opening angle is too big: recurse!
          macro_particles = this.create_macro_particles(node, macro_particles);
        } else {
          // otherwise create a macro particle from the node
          let macro_particle = cloud.get_macro_particle(node, theta, this);
          if (macro_particle) macro_particles.push(macro_particle);
        }
      }
    }
    // draw macro-particles
    // console.log(macro_particles.length)
    if (this === cloud.particles[50]) {
      for (let p of macro_particles) {
        let ctx_coords = get_ctx_coords([p.position.x, p.position.y])
        ctx.strokeStyle = 'red'
        ctx.fillStyle = 'red'
        ctx.beginPath()
        ctx.arc(ctx_coords[0], ctx_coords[1], 1, 0, TAU)
        ctx.stroke()
        ctx.fill()
      }
    }
    return macro_particles;
  }
}

class Cloud {
  constructor(nr_of_particles) {
    this.nr_of_particles = nr_of_particles;

    let origin = new Vector2D(world.width / 2, world.height / 2);
    this.particles = [];
    for (let idx = 0; idx < nr_of_particles; idx++) {
      let m = Math.max(1, Math.floor(max_mass * Math.random()));
      let pos_0 = new Vector2D(
        // world.width * Math.random(),
        // world.height * Math.random()
        // world.width / 4, //+ (2 * Math.random() - 1) * (world.width / 8),
        world.width / 3,
        // (world.width / 2) * Math.random(),
        // TAU * Math.random(),
        (TAU * idx) / nr_of_particles,
        "pol"
      ).add(origin);
      let vel_0 = new Vector2D(
        // 2 * Math.random() - 1,
        // 2 * Math.random() - 1
        Math.sqrt((G * nr_of_particles) / (world.width / 1)),
        pos_0.sub(origin).angle() - TAU / 4, // Math.random() * TAU
        "pol"
      );
      let p = new Particle(m, pos_0, vel_0);
      this.particles.push(p);
    }
  }
  update() {
    this.particles_from_loc = {};
    // define quad tree
    let n = quad_tree_capacity;
    let boundary = new Rectangle(
      world.width / 2,
      world.height / 2,
      world.width / 2,
      world.height / 2
    );
    quad_tree = new QuadTree(boundary, n);
    let particles = this.particles;
    for (let p of particles) {
      let pos = p.position;
      this.particles_from_loc[[pos.x, pos.y]] = p; // TODO: better idea?
      quad_tree.insert(pos);
    }
    // loop over particles & update
    for (let p of this.particles) {
      p.update();
    }
  }
  get_macro_particle(node, theta, p) {
    // search for particles in node
    let found = [];
    node.query(node.boundary, found); // TODO: node.boundary
    // calculate center of mass
    // let com = new Vector2D(node.boundary.x, node.boundary.y);
    let com = 0 //new Vector2D(-0.0000001, -0.0000001); // center of mass // TODO: wtf?
    let M = 0; // total mass of node
    for (let v of found) {
      let p = cloud.particles_from_loc[[v.x, v.y]];
      let m = p.mass;
      if (com === 0) {
        com = p.position.mult(m)
      } else {
        com = com.add(p.position.mult(m));
        M += m;
      }
    }
    // only return macro particle for non-empty nodes
    if (M === 0) {
      return null;
    } else {
      com = com.mult(1 / M);
      // define macro particle describing cloud/cell
      let macro_particle = new Particle(M, com, new Vector2D(0, 0));
      // draw & return
      if (theta < max_opening_angle && p == this.particles[50]) {
        cloud.draw_macro_particle(node, macro_particle, p);
        for (let v of found) {
          let p = cloud.particles_from_loc[[v.x, v.y]];
          let ctx_coords = get_ctx_coords([p.position.x, p.position.y]);
          let ctx_coords2 = get_ctx_coords([
            node.boundary.x, // com.x,
            node.boundary.y, // com.y,
          ]);
          ctx.strokeStyle = "gray";
          ctx.beginPath();
          ctx.moveTo(ctx_coords[0], ctx_coords[1]);
          ctx.lineTo(ctx_coords2[0], ctx_coords2[1]);
          ctx.stroke();

          // draw active nodes
          if (bool_draw_active_qt_nodes) {
            let x = node.boundary.x;
            let y = node.boundary.y;
            let w = node.boundary.w;
            let h = node.boundary.h;
            ctx.beginPath();
            ctx_coords = get_ctx_coords([x + w, y + h]);
            ctx.moveTo(ctx_coords[0], ctx_coords[1]);
            ctx_coords = get_ctx_coords([x - w, y + h]);
            ctx.lineTo(ctx_coords[0], ctx_coords[1]);
            ctx.moveTo(ctx_coords[0], ctx_coords[1]);
            ctx_coords = get_ctx_coords([x - w, y - h]);
            ctx.lineTo(ctx_coords[0], ctx_coords[1]);
            ctx.moveTo(ctx_coords[0], ctx_coords[1]);
            ctx_coords = get_ctx_coords([x + w, y - h]);
            ctx.lineTo(ctx_coords[0], ctx_coords[1]);
            ctx.moveTo(ctx_coords[0], ctx_coords[1]);
            ctx_coords = get_ctx_coords([x + w, y + h]);
            ctx.lineTo(ctx_coords[0], ctx_coords[1]);
            ctx.moveTo(ctx_coords[0], ctx_coords[1]);
            ctx.stroke();
          }
        }
      }
      return macro_particle;
    }
  }

  draw() {
    for (let p of this.particles) {
      p.draw();
    }
  }
  draw_macro_particle(node, macro_particle, p) {
    ctx.strokeStyle = "green";
    ctx.fillStyle = "green";
    ctx.lineWidth = 2;
    let ctx_coords = get_ctx_coords([
      macro_particle.position.x,
      macro_particle.position.y,
      // node.boundary.x,
      // node.boundary.y,
    ]);
    let r = macro_particle.mass;
    ctx.beginPath();
    ctx.arc(ctx_coords[0], ctx_coords[1], r, 0, TAU);
    ctx.stroke();
    //
    let ctx_coords_0 = get_ctx_coords([p.position.x, p.position.y]);
    let ctx_coords_1 = get_ctx_coords([node.boundary.x, node.boundary.y]);
    ctx.lineWidth = 1;
    ctx.strokeStyle = "gray";
    ctx.beginPath();
    ctx.moveTo(ctx_coords_0[0], ctx_coords_0[1]);
    ctx.lineTo(ctx_coords_1[0], ctx_coords_1[1]);
    ctx.stroke();
  }
}

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

class World {
  constructor(world_size) {
    this.width = world_size[0];
    this.height = world_size[1];
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
  //   // canvas.addEventListener("keypress", function(e) {
  //   //   var keynum;
  //   //   if (window.event) { // I.E.
  //   //     keynum = e.keyCode;
  //   //   } else if (e.which) { // Netscape/Firefox/Opera
  //   //     keynum = e.which;
  //   //   }
  //   //   alert(String.fromCharCode(keynum))
  //   // })
  //    canvas.addEventListener("mousemove", function (e) {
  //      const ctx_coords = getCursorPosition(canvas, e);
  //      const map_coords = get_map_coords(ctx_coords);
  //      const x = map_coords[0]
  //      const y = map_coords[1]
  //      if (x > 0 && x < world.width && y > 0 && y < world.height) {
  //        let mouse_pos = new Vector2D(x, y)
  //        predators[0].follow_mouse(mouse_pos)
  //      }
  //    })
  //    // canvas.addEventListener("mousedown", function (e) {
  //   //    const ctx_coords = getCursorPosition(canvas, e);
  //   //    const map_coords = get_map_coords(ctx_coords);
  //   //    const col_idx = Math.floor(map_coords[0]);
  //   //    const row_idx = Math.floor(map_coords[1]);
  //   //    if (placement_select === "food") {
  //   //      world.food_sources[row_idx][col_idx] += food_placement_amount;
  //   //    } else if (placement_select === "phA") {
  //   //      world.pheromone_strengths[0][row_idx][col_idx] += 100;
  //   //      world.active_grid_cells.push([row_idx, col_idx]);
  //   //    } else if (placement_select === "phB") {
  //   //      world.pheromone_strengths[1][row_idx][col_idx] += 100;
  //   //      world.active_grid_cells.push([row_idx, col_idx]);
  //   //    } else if (placement_select === "walls") {
  //   //      for (let i = row_idx - 1; i <= row_idx + 1; i++) {
  //   //        for (let j = col_idx - 1; j <= col_idx + 1; j++) {
  //   //          try {
  //   //            world.walls[i][j] = 1
  //   //          } finally {}
  //   //        }
  //   //      }
  //   //    } else if (placement_select === "remove_walls") {
  //   //      for (let i = row_idx - 1; i <= row_idx + 1; i++) {
  //   //        for (let j = col_idx - 1; j <= col_idx + 1; j++) {
  //   //          try {
  //   //            world.walls[i][j] = 0
  //   //          } finally {}
  //   //        }
  //   //      }
  //   //    }
  //   //    // console.log(ctx_coords, [col_idx, row_idx]);
  //   //  });
    // BUTTONS
    document
      .getElementById("button_toggle_pause")
      .addEventListener("click", function () {
        paused = !paused;
        console.log("toggled pause");
      });
    document
      .getElementById("button_reset")
      .addEventListener("click", function () {
        init()
        console.log("reset");
      });

    document
      .getElementById("button_toggle_draw_qt_branches")
      .addEventListener("click", function () {
        bool_draw_qt_branches = !bool_draw_qt_branches;
        console.log("toggled showing of quad tree branches");
      });
    document
      .getElementById("button_toggle_draw_all_qt_nodes")
      .addEventListener("click", function () {
        bool_draw_all_qt_nodes = !bool_draw_all_qt_nodes;
        console.log("toggled showing of all quad tree nodes");
      });
    document
      .getElementById("button_toggle_draw_active_qt_nodes")
      .addEventListener("click", function () {
        bool_draw_active_qt_nodes = !bool_draw_active_qt_nodes;
        console.log("toggled showing of active quad tree nodes");
      });

    document
      .getElementById("button_toggle_draw_trajectories")
      .addEventListener("click", function () {
        bool_draw_trajectories = !bool_draw_trajectories;
        console.log("toggled showing of trajectories");
      });
    document
      .getElementById("button_toggle_draw_vel_vecs")
      .addEventListener("click", function () {
        bool_draw_vel_vecs = !bool_draw_vel_vecs;
        console.log("toggled showing of velocity vectors");
      });
    document
      .getElementById("button_toggle_draw_acc_vecs")
      .addEventListener("click", function () {
        bool_draw_acc_vecs = !bool_draw_acc_vecs;
        console.log("toggled showing of acceleration vectors");
      });
  //   // buttons for displaying force radii
  //   document
  //     .getElementById("button_display_avoidance_radius")
  //     .addEventListener("click", function () {
  //       bool_draw_avoidance_radius = !bool_draw_avoidance_radius;
  //       console.log("toggled drawing of avoidance radius");
  //     });
  //   document
  //     .getElementById("button_display_attraction_radius")
  //     .addEventListener("click", function () {
  //       bool_draw_attraction_radius = !bool_draw_attraction_radius;
  //       console.log("toggled drawing of attraction radius");
  //     });
  //   document
  //     .getElementById("button_display_cohesion_radius")
  //     .addEventListener("click", function () {
  //       bool_draw_cohesion_radius = !bool_draw_cohesion_radius;
  //       console.log("toggled drawing of cohesion radius");
  //     });
  //   document
  //     .getElementById("button_toggle_pause")
  //     .addEventListener("click", function () {
  //       paused = !paused;
  //       console.log("toggled pause");
  //     });
  //   document
  //     .getElementById("button_toggle_use_quad_tree")
  //     .addEventListener("click", function () {
  //       use_quad_tree = !use_quad_tree;
  //       console.log("toggled usage of quad tree");
  //     });
  //   // document
  //   //   .getElementById("button_toggle_periodic_bounds")
  //   //   .addEventListener("click", function () {
  //   //     periodic_bounds = !periodic_bounds;
  //   //     console.log("toggled periodic bounds");
  //   //   });
  //   // document
  //   //   .getElementById("button_toggle_display_sensor_radius")
  //   //   .addEventListener("click", function () {
  //   //     bool_draw_boid_sensor_radius = !bool_draw_boid_sensor_radius;
  //   //     console.log("toggled drawing of boid sensor radius");
  //   //   });
  //   // document
  //   //   .getElementById("button_toggle_display_collision_radius")
  //   //   .addEventListener("click", function () {
  //   //     bool_draw_boid_collision_radius = !bool_draw_boid_collision_radius;
  //   //     console.log("toggled drawing of boid collision radius");
  //   //   });
  //   // document
  //   //   .getElementById("button_toggle_display_velocity_vector")
  //   //   .addEventListener("click", function () {
  //   //     bool_draw_boid_velocity_vectors = !bool_draw_boid_velocity_vectors;
  //   //     console.log("toggled drawing of boid velocity vectors");
  //   //   });
  //   // SLIDERS
  //   // sliders for sensor radii
  //   document
  //     .getElementById("slider_avoidance_radius")
  //     .addEventListener("click", function () {
  //       let value = document.getElementById("slider_avoidance_radius").value;
  //       avoidance_radius = (value / 1000) * world.width; // TODO: only for W=H
  //       console.log("new boid avoidance radius: ", avoidance_radius);
  //     });
  //   document
  //     .getElementById("slider_attraction_radius")
  //     .addEventListener("click", function () {
  //       let value = document.getElementById("slider_attraction_radius").value;
  //       attraction_radius = (value / 1000) * world.width; // TODO: only for W=H
  //       console.log("new boid attraction radius: ", attraction_radius);
  //     });
  //   document
  //     .getElementById("slider_cohesion_radius")
  //     .addEventListener("click", function () {
  //       let value = document.getElementById("slider_cohesion_radius").value;
  //       cohesion_radius = (value / 1000) * world.width; // TODO: only for W=H
  //       console.log("new boid cohesion radius: ", cohesion_radius);
  //     });
  //   // sliders for force strengths
  //   document
  //     .getElementById("slider_avoidance_strength")
  //     .addEventListener("click", function () {
  //       let value = document.getElementById("slider_avoidance_strength").value;
  //       avoidance_force = value / 100;
  //       console.log("new boid avoidance force: ", avoidance_force);
  //     });
  //   document
  //     .getElementById("slider_attraction_strength")
  //     .addEventListener("click", function () {
  //       let value = document.getElementById("slider_attraction_strength").value;
  //       attraction_force = value / 100;
  //       console.log("new boid attraction strength: ", attraction_force);
  //     });
  //   document
  //     .getElementById("slider_cohesion_strength")
  //     .addEventListener("click", function () {
  //       let value = document.getElementById("slider_cohesion_strength").value;
  //       cohesion_force = value / 100;
  //       console.log("new boid cohesion strength: ", cohesion_force);
  //     });
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
  cloud = new Cloud(cloud_size);

  // set values of sliders for sensor radii
  // document.getElementById("slider_avoidance_radius").value = avoidance_radius;
  // document.getElementById("slider_attraction_radius").value = attraction_radius;
  // document.getElementById("slider_cohesion_radius").value = cohesion_radius;
  // // set values of sliders for force strengths
  // document.getElementById("slider_avoidance_strength").value =
  //   avoidance_force * 100;
  // document.getElementById("slider_attraction_strength").value =
  //   attraction_force * 100;
  // document.getElementById("slider_cohesion_strength").value =
  //   cohesion_force * 100;

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
  if (!bool_draw_trajectories) ctx.clearRect(0, 0, canvas.width, canvas.height);
  // update cloud of particles
  cloud.update();
  // show quad tree grid
  if (bool_draw_all_qt_nodes) quad_tree.show();
  // draw particles
  cloud.draw();
  // increment time
  time_step += 1;
}

init();
animate();
