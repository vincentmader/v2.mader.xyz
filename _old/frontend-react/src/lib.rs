// NOTES:
// - assume time step: dt = 1

// TODOS:
// - implement sensor radii  (or grid-based broad search?)
// - improve performance -> remove .clone() calls?
//   * clone needed bc: 
//     copy not implemented for Boid (bc not impl. for Vec)

// IMPORTS
// =============================================================================

use rand::Rng;

use wasm_bindgen::prelude::*;
use web_sys::console;

// #[macro_use]  // ?
// extern crate stdweb;

// mod canvas;
// use canvas::Canvas;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// BOID
// =============================================================================

#[derive(Debug, Clone)]
struct Boid {
    pos: Vec<f64>,
    vel: Vec<f64>,
    speed: f64
}

impl Boid {
    fn new() -> Boid {
        const PI: f64 = 3.14159265358;
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen_range(0.0..2.*PI);
        let speed = 0.1;

        let vel_x: f64 = speed * theta.cos();
        let vel_y: f64 = speed * theta.sin();       
        let pos_x: f64 = rng.gen();       
        let pos_y: f64 = rng.gen();       

        let pos: Vec<f64> = vec![pos_x, pos_y];
        let vel: Vec<f64> = vec![vel_x, vel_y];

        return Boid {
            pos, vel, speed
        };
    }

    fn get_distance(&self, neighbor: &Boid) -> f64 {
        let delta_x: f64 = neighbor.pos[0] - self.pos[0];
        let delta_y: f64 = neighbor.pos[1] - self.pos[1];
        return (delta_x.powf(2.) + delta_y.powf(2.)).sqrt();
    }

    fn apply_cohesion(
        &mut self, neighbors: &Vec<Boid>, 
        sensor_radius: f64,
        force_strength: f64,
    ) {
        let mut pos_avg_x: f64 = self.pos[0];
        let mut pos_avg_y: f64 = self.pos[1];
        let mut neighbor_count: u32 = 0;
        // loop over neighbors & calculate average position (center of mass)
        for neighbor_idx in 0..neighbors.len() {
            let neighbor: &Boid = &neighbors[neighbor_idx];
            let distance: f64 = self.get_distance(&neighbor);
            if distance < sensor_radius {
                pos_avg_x += neighbor.pos[0];
                pos_avg_y += neighbor.pos[1];
                neighbor_count += 1;
            } else {
                continue
            }
        }
        pos_avg_x /= f64::from(neighbor_count);
        pos_avg_y /= f64::from(neighbor_count);
        let pos_avg: Vec<f64> = vec![pos_avg_x, pos_avg_y];
        // steer towards center of mass
        let vel_goal: Vec<f64> = vec![
            self.vel[0] - pos_avg[0],
            self.vel[1] - pos_avg[1],
        ];
        self.vel[0] += force_strength * (vel_goal[0] - self.vel[0]);
        self.vel[1] += force_strength * (vel_goal[1] - self.vel[1]);
    }

    fn apply_alignment(
        &mut self, neighbors: &Vec<Boid>, 
        sensor_radius: f64,
        force_strength: f64,
    ) {
        let mut vel_avg_x: f64 = self.vel[0];
        let mut vel_avg_y: f64 = self.vel[1];
        let mut neighbor_count: u32 = 0;
        // loop over neighbors & calculate average velocity
        for neighbor_idx in 0..neighbors.len() {
            let neighbor: &Boid = &neighbors[neighbor_idx];
            let distance: f64 = self.get_distance(&neighbor);
            if distance < sensor_radius {
                vel_avg_x += neighbor.vel[0];
                vel_avg_y += neighbor.vel[1];
                neighbor_count += 1;
            } else {
                continue
            }
        }
        vel_avg_x /= f64::from(neighbor_count);
        vel_avg_y /= f64::from(neighbor_count);
        let vel_avg: Vec<f64> = vec![vel_avg_x, vel_avg_y];
        // steer towards avg. vel. vector
        self.vel[0] += force_strength * (vel_avg[0] - self.vel[0]);
        self.vel[1] += force_strength * (vel_avg[1] - self.vel[1]);
    }

    fn apply_evasion(
        &mut self, predators: &Vec<Predator>, 
        sensor_radius: f64,
        force_strength: f64,
    ) {
        // TODO: find closest predator
        let mut found_predator_in_sensor_radius: bool = false;
        let mut closest_distance: f64 = 100000.;
        let mut closest_predator = Predator { pos: vec![0., 0.], vel: vec![0., 0.]};
        for predator_idx in 0..predators.len() {
            let predator: &Predator = &predators[predator_idx];
            let distance: f64 = ((predator.pos[0] - self.pos[0]).powf(2.) + (predator.pos[0] - self.pos[0]).powf(2.)).sqrt();
            if distance > sensor_radius {
                continue
            }
            found_predator_in_sensor_radius = true;
            if distance < closest_distance || closest_distance > 1. {
                let closest_predator = predator;
                closest_distance = distance;
            }
        }
        if found_predator_in_sensor_radius {
            // TODO: steer away
            let vel_goal: Vec<f64> = vec![
                self.pos[0] - closest_predator.pos[0],
                self.pos[1] - closest_predator.pos[1],
            ];
            self.vel[0] += force_strength * (vel_goal[0] - self.vel[0]);
            self.vel[1] += force_strength * (vel_goal[1] - self.vel[1]);
        }
    }

    fn apply_separation(
        &mut self, neighbors: &Vec<Boid>, 
        sensor_radius: f64,
        force_strength: f64,
    ) {
        // TODO: find nearest neighbor
        let mut found_boid_in_sensor_radius: bool = false;
        let mut closest_distance: f64 = 100000.;
        let mut closest_boid_idx: i32 = -100;
        for neighbor_idx in 0..neighbors.len() {
            let neighbor: &Boid = &neighbors[neighbor_idx];
            let distance: f64 = self.get_distance(&neighbor);
            if distance < sensor_radius {

                // println!("{}", distance)
            } else {
                continue
            }
        }

        // let closest_boid = neighbors[closest_boid_idx];
        // TODO: steer away
    }

    fn update_position(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
    }

    fn apply_boundaries(&mut self) {
        if self.pos[0] < 0. {self.pos[0] += 1.}
        if self.pos[1] < 0. {self.pos[1] += 1.}
        if self.pos[0] > 1. {self.pos[0] -= 1.}
        if self.pos[1] > 1. {self.pos[1] -= 1.}
    }

    fn normalize_velocity(&mut self, friction: f64) {
        let vec_length: f64 = (self.vel[0].powf(2.) + self.vel[1].powf(2.)).sqrt();
        self.vel[0] *= self.speed / vec_length * (1. - friction);
        self.vel[1] *= self.speed / vec_length * (1. - friction);
    }

    fn update(
        &mut self, 
        boids: &Vec<Boid>, 
        predators: &Vec<Predator>, 
        sensor_radius: &SensorRadius,
        force_strength: &ForceStrength,
        friction: f64,
    ) {
        // TODO: choose boid neighbors from neighboring cells (broad search)
        let neighbors: &Vec<Boid> = boids;
        // 
        // apply forces
        // TODO: pre-calculate forces, then call self.update_velocities() ?
        self.apply_cohesion(neighbors, sensor_radius.cohesion, force_strength.cohesion);
        self.apply_alignment(neighbors, sensor_radius.alignment, force_strength.alignment);
        self.apply_evasion(predators, sensor_radius.evasion, force_strength.evasion);
        self.apply_separation(neighbors, sensor_radius.separation, force_strength.separation);
        // update position
        self.normalize_velocity(friction);
        self.update_position();
        self.apply_boundaries();
    }
}

// FLOCK
// =============================================================================

#[derive(Debug, Clone)]
struct Flock {
    boids: Vec<Boid>
}

impl Flock {
    fn new(nr_of_boids: &usize) -> Flock {
        let mut boids = Vec::with_capacity(*nr_of_boids);
        for _ in 0..*nr_of_boids {
            boids.push(Boid::new());
        }
        return Flock {
            boids
        };
    }

    fn update(
        &mut self, 
        predators: &Vec<Predator>, 
        sensor_radius: &SensorRadius, 
        force_strength: &ForceStrength,
        friction: f64,
    ) {
        let boids = self.boids.clone();
        for boid_idx in 0..self.boids.len() {
            self.boids[boid_idx].update(
                &boids,
                predators, 
                sensor_radius, 
                force_strength,
                friction,
            );
        }
    }
}

// PREDATOR
// =============================================================================

#[derive(Debug, Clone)]
struct Predator {
    pos: Vec<f64>,
    vel: Vec<f64>,
}

impl Predator {
    fn new() -> Predator {
        const PI: f64 = 3.14159265358;
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen_range(0.0..2.*PI);
        let speed = 0.1;

        let pos_x: f64 = rng.gen();       
        let pos_y: f64 = rng.gen();       
        let vel_x: f64 = speed * theta.cos();
        let vel_y: f64 = speed * theta.sin();       

        let pos: Vec<f64> = vec![pos_x, pos_y];
        let vel: Vec<f64> = vec![vel_x, vel_y];

        return Predator {
            pos, vel
        };
    }

    fn follow_boids(&mut self, boids: &Vec<Boid>) {
        // TODO: loop over boids in neighboring cells, calc. center of mass
        // TODO: steer towards local flock CoM
    }

    fn update_position(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
    }

    fn apply_boundaries(&mut self) {
        if self.pos[0] < 0. {self.pos[0] += 1.}
        if self.pos[1] < 0. {self.pos[1] += 1.}
        if self.pos[0] > 1. {self.pos[0] -= 1.}
        if self.pos[1] > 1. {self.pos[1] -= 1.}
    }

    fn update(&mut self, boids: &Vec<Boid>) {
        self.follow_boids(&boids);
        self.update_position();
        self.apply_boundaries();
    }
}

// OTHER STRUCTS
// =============================================================================

struct SensorRadius {
    separation: f64,
    cohesion: f64,
    alignment: f64,
    evasion: f64
}

struct ForceStrength {
    separation: f64,
    cohesion: f64,
    alignment: f64,
    evasion: f64
}

// MAIN
// =============================================================================

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hellooo world!"));


    // stdweb::initialize();

    // let canvas = Canvas::new("#canvas", 100, 100);
    // canvas.draw(5, 5, "red");

    // stdweb::event_loop();

    // constants
    // TODO: global constants? (e.g. pi)
    const SENSOR_RADIUS: SensorRadius = SensorRadius {
        separation: 0.1, cohesion: 0.1, alignment: 0.1, evasion: 0.1
    };
    const FORCE_STRENGTH: ForceStrength = ForceStrength {
        separation: 0.1, cohesion: 0.1, alignment: 0.1, evasion: 0.1
    };
    const FRICTION: f64 = 0.;
    // parameters
    let nr_of_boids: usize = 2000;
    let nr_of_predators: usize = 2;
    // world setup
    let mut flock: Flock = Flock::new(&nr_of_boids);
    let mut predators = Vec::with_capacity(nr_of_predators);
    for _ in 0..nr_of_predators {
        predators.push(Predator::new());
    }
    // info
    let mut iteration_step_idx: usize = 0;

    // main loop
    loop {
        // update boid flock
        flock.update(
            &predators, &SENSOR_RADIUS, &FORCE_STRENGTH, FRICTION
        );
        // update predators (TODO: move to separate func?)
        for predator_idx in 0..predators.len() {
            predators[predator_idx].update(&flock.boids);
        }

        // TODO: draw live on (html) canvas?

        // print info
        if iteration_step_idx % 60 == 0 {

            // console::log_1(&JsValue::from_str("Hellooo world!"));
            console::log_1(&JsValue::from_f64(flock.boids[0].pos[0]));

            println!(
                "{:?}\n\n{:?}\n\n==============\n\nstep {}\nnr of boids: {}", 
                flock.boids, predators, iteration_step_idx, nr_of_boids
            );
        }
        // println!(
        //     "step {}", iteration_step_idx
        // );
        iteration_step_idx += 1;
        break;
    }

    Ok(())
}
