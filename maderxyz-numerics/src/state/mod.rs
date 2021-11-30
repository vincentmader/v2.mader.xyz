
use rand::{Rng};

mod field;
mod object_family;
pub use field::Field;
pub use field::FieldType;
pub use object_family::ObjectFamily;
pub use object_family::ObjectType;
pub use object_family::ObjectAttribute;
use crate::interactions::ObjectInteraction;
use crate::interactions::FieldInteraction;
use crate::integrators::Integrator;
use crate::integrators::FieldIntegrator;


#[derive(Clone)]
pub struct State {

    pub iteration_idx: usize,
    pub object_families: Vec<ObjectFamily>,
    // pub cellular_automata: Vec<CellularAutomaton>,
    pub fields: Vec<Field>,
    // pub spins: Vec<bool>,

}
impl State {

    pub fn new(page_id: &str) -> Self {

        let iteration_idx = 0;
        let object_families = initialize_object_families(page_id);
        let fields = initialize_fields(page_id);

        State { iteration_idx, object_families, fields }
    }
}

fn initialize_fields(page_id: &str) -> Vec<Field> {
    let mut rng = rand::thread_rng();

    let mut fields: Vec<Field> = Vec::new();
    match page_id {
        "diffusion" => {
            let field_type = FieldType::Fluid; // TODO rename, fluid density?
            let interactions = Vec::from([FieldInteraction::Diffusion]);
            let integrator = FieldIntegrator::Diffusion; // TODO

            let mut cells: Vec<Vec<f64>> = Vec::new() ;
            const GRID_SIZE: usize = 42;
            for row_idx in 0..GRID_SIZE {
                for col_idx in 0..GRID_SIZE {

                    let mut density: f64 = 0.;
                    
                    let r = (
                        (row_idx as f64-GRID_SIZE as f64/2.).powf(2.) + 
                        (col_idx as f64-GRID_SIZE as f64/2.).powf(2.)
                    ).sqrt();
                    // if r < GRID_SIZE as f64 / 10. {
                    //     density = 0.;
                    // } else {
                        density = rng.gen();
                    // }
                    cells.push(Vec::from([density]))
                }
            }
            let dimensions = (GRID_SIZE, GRID_SIZE);
            let density_field = Field::new(
                0,
                field_type, 
                interactions, 
                dimensions,
                cells,
                integrator,
            );
            fields.push(density_field);
        }, "ising-model" => {
            let field_type = FieldType::Spin; // TODO rename, fluid density?
            let interactions = Vec::from([
                // FieldInteraction::Ising
            ]);
            let integrator = FieldIntegrator::Ising; // TODO

            let mut cells: Vec<Vec<f64>> = Vec::new() ;
            const GRID_SIZE: usize = 100;
            for row_idx in 0..GRID_SIZE {
                for col_idx in 0..GRID_SIZE {
                    let spin_up: bool = rng.gen();
                    let spin = match spin_up {
                        true => 1.,
                        false => -1.
                    };
                    
                    // let r = (
                    //     (row_idx as f64-GRID_SIZE as f64/2.).powf(2.) + 
                    //     (col_idx as f64-GRID_SIZE as f64/2.).powf(2.)
                    // ).sqrt();
                    // if r < GRID_SIZE as f64 / 10. {
                    //     density = 0.;
                    // } else {
                        // density = rng.gen();
                    // }
                    cells.push(Vec::from([spin]))
                }
            }
            let dimensions = (GRID_SIZE, GRID_SIZE);
            let density_field = Field::new(
                0,
                field_type, 
                interactions, 
                dimensions,
                cells,
                integrator,
            );
            fields.push(density_field);
        },
        _ => {}
    }

    fields
}

fn initialize_object_families(page_id: &str) -> Vec<ObjectFamily> {
    const TAU: f64 = 2.*3.14159;
    let mut rng = rand::thread_rng();

    let dt = 0.01;
    let epsilon = 0.0;

    let v_kepler = | m: f64, r: f64 | (m / r).sqrt();

    let mut object_families: Vec<ObjectFamily> = Vec::new();
    match page_id {
        "3body-moon" => {
            let mut objects: Vec<Vec<f64>> = Vec::new();

            let epsilon = 0.01;
            let (r, dr) = (0.9, 0.02);

            // add Sun
            let m_sun = 1.;
            let (x, y, u, v) = (0., 0., 0., 0.);
            objects.push(Vec::from([m_sun, x, y, u, v]));
            let tail_length = 0;
            let object_family = ObjectFamily::new(
                0, ObjectType::Static, objects, 
                Vec::from([ObjectInteraction::NewtonianGravity]),
                Integrator::EulerExplicit, dt, epsilon, tail_length
            );
            object_families.push(object_family);

            let mut objects: Vec<Vec<f64>> = Vec::new();
            // add Earth
            let m_earth = 1e-3;
            let v = 0.707*v_kepler(m_sun, r);  // TODO: add G*M vars?
            let (x, y, u, v) = (r, 0., 0., v);
            objects.push(Vec::from([m_earth, x, y, u, v]));
            let tail_length = 100;
            let object_family = ObjectFamily::new(
                1, ObjectType::Body, objects, 
                Vec::from([ObjectInteraction::NewtonianGravity]),
                Integrator::EulerExplicit, dt, epsilon, tail_length
            );
            object_families.push(object_family);

            let mut objects: Vec<Vec<f64>> = Vec::new();
            // add Moon
            let m = 1e-6;
            let v = 0.707*v_kepler(m_sun, r+dr) + 0.707*v_kepler(m_earth, dr);
            let (x, y, u, v) = (r+dr, 0., 0., v);
            objects.push(Vec::from([m, x, y, u, v]));
            let tail_length = 10;
            let object_family = ObjectFamily::new(
                2, ObjectType::Particle, objects, 
                Vec::from([ObjectInteraction::NewtonianGravity]),
                Integrator::EulerExplicit, dt, epsilon, tail_length
            );
            object_families.push(object_family);
            // TODO: fix Moon orbit
            //  - what happened?
            //  - move to mass-less particles for moons/rings?

        }, "3body-fig8" => {
            const M: f64 = 1.;
            let xs = [0.7775727187509279270, 0., -0.7775727187509279270];
            let ys = [0.6287930240184685937, 0., -0.6287930240184685937];
            let us = [-0.06507160612095737318, 0.1301432122419148851, -0.06507160612095737318];
            let vs = [0.6324957346748190101, -1.264991469349638020, 0.6324957346748190101];

            let mut bodies: Vec<Vec<f64>> = Vec::new();
            let mut particles: Vec<Vec<f64>> = Vec::new();
            for body_idx in 0..3 {

                // bodies
                let object_type = ObjectType::Body;
                let epsilon: f64 = 0.;
                let (x0, y0, u0, v0) = (xs[body_idx], ys[body_idx], us[body_idx], vs[body_idx]);
                bodies.push(Vec::from([M, x0, y0, u0, v0]));

                // particles
                let m = 1; // has to be non-zero, 1/0 inf, TODO remove M from particle vec
                let r = 0.05;
                let nr_of_particles = 40;
                for particle_idx in 0..nr_of_particles {
    
                    let phi = TAU * particle_idx as f64 / nr_of_particles as f64;
                    let x = x0 + r * phi.cos();
                    let y = y0 + r * phi.sin();
                    let u = u0 - v_kepler(M, r) * phi.sin();
                    let v = v0 + v_kepler(M, r) * phi.cos();
    
                    // particles.push(Vec::from([0.1, x, y, u, v])); // TODO why does m have to be non-zero? -> 1/0 inf
                }
            }
            let integrator = Integrator::EulerExplicit;
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let body_family = ObjectFamily::new(
                0, ObjectType::Body, bodies, interactions, integrator, dt, epsilon, 201
            );
            let integrator = Integrator::EulerExplicit;
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let particle_family = ObjectFamily::new(
                0, ObjectType::Particle, particles, interactions, integrator, dt, 0.05, 0
            );
            object_families.push(body_family);
            object_families.push(particle_family);

            // let object_type = ObjectType::Body;
            // let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            // let integrator = Integrator::EulerExplicit;
            // let epsilon: f64 = 0.;
            // const M: f64 = 1.;
            // let tail_length = 201;

            // let mut objects: Vec<Vec<f64>> = Vec::new();
            // // body 1 & 3
            // let (x, y, u, v) = (
            //    0.7775727187509279270, 0.6287930240184685937, 
            //    -0.06507160612095737318, 0.6324957346748190101
            // );
            // objects.push(Vec::from([M, x, y, u, v]));
            // objects.push(Vec::from([M, -x, -y, u, v]));
            // // body 2 (center)
            // let (x, y, u, v) = (
            //     0., 0., 
            //     0.1301432122419148851, -1.264991469349638020
            // );
            // objects.push(Vec::from([M, x, y, u, v]));

            // let object_family = ObjectFamily::new(
            //     0, object_type, objects, interactions, integrator, dt, epsilon, tail_length
            // );
            // object_families.push(object_family);

            // particles
            // let object_type = ObjectType::Particle;
            // let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            // let integrator = Integrator::EulerExplicit;
            // let epsilon: f64 = 0.05;
            // let tail_length = 0;
            // let mut objects: Vec<Vec<f64>> = Vec::new();
            // let (x0, y0, u0, v0) = (
            //     0., 0., 
            //     0.1301432122419148851, -1.264991469349638020
            // );
            // let r = 0.05;
            // let nr_of_particles = 40;
            // for particle_idx in 0..nr_of_particles {

            //     let phi = TAU * particle_idx as f64 / nr_of_particles as f64;
            //     let x = x0 + r * phi.cos();
            //     let y = y0 + r * phi.sin();
            //     let u = u0 - v_kepler(M, r) * phi.sin();
            //     let v = v0 + v_kepler(M, r) * phi.cos();

            //     objects.push(Vec::from([0.1, x, y, u, v])); // TODO why does m have to be non-zero? -> 1/0 inf
            // }
            // let object_family = ObjectFamily::new(
            //     1, object_type, objects, interactions, integrator, dt, epsilon, tail_length
 //           );
            // object_families.push(object_family);

        }, "3body-broucke" => {

            let object_type = ObjectType::Body;
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            const M: f64 = 1.;
            let tail_length = 30;
            let dt = 0.006;

            let mut objects: Vec<Vec<f64>> = Vec::new();
            let (x, y, u, v) = (-0.9892620043, 0., 0.,  1.9169244185);
            objects.push(Vec::from([M, x, y, u, v]));
            let (x, y, u, v) = (-1.2203557197, 0., 0., -2.1079512924);
            objects.push(Vec::from([M, x, y, u, v]));
            let (x, y, u, v) = ( 2.2096177241, 0., 0.,  0.1910268738);
            objects.push(Vec::from([M, x, y, u, v]));

            let object_family = ObjectFamily::new(
                0, object_type, objects, interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);

        }, "3body-freefall" => {

            // let object_type = ObjectType::Body;
            // let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            // let integrator = Integrator::EulerExplicit;
            // let epsilon: f64 = 0.;
            // const M: f64 = 1.;

            // let mut objects: Vec<Vec<f64>> = Vec::new();
            // let (x, y, u, v) = (0.5, 0., 0., 0.);
            // objects.push(Vec::from([M, x, y, u, v]));
            // objects.push(Vec::from([M, -x, y, u, v]));
            // let (x, y, u, v) = (0.0207067154, 0.3133550361, 0., 0.);
            // objects.push(Vec::from([M, x, y, u, v]));

            // let object_family = ObjectFamily::new(
            //     0, object_type, objects,
            //     interactions, integrator, dt, epsilon
            // );
            // object_families.push(object_family);

        }, "3body-liao" => {

            // let object_type = ObjectType::Body;
            // let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            // let integrator = Integrator::EulerExplicit;

            // let mut objects: Vec<Vec<f64>> = Vec::new();
            // let (m, x, y, u, v) = (
            //     1., 1., 0., 0.2374365149, 0.2536896353
            // );
            // objects.push(Vec::from([m, x, y, u, v]));
            // objects.push(Vec::from([m, -x, y, u, v]));
            // let (m, x, y, u, v) = (
            //     0.5, 0., 0., -0.9497460596, -1.0147585412
            // );
            // objects.push(Vec::from([m, x, y, u, v]));

            // let object_family = ObjectFamily::new(
            //     0, object_type, objects,
            //     interactions, integrator, dt, epsilon
            // );
            // object_families.push(object_family);

        }, "3body-moth" => {

            // let object_type = ObjectType::Body;
            // let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            // let integrator = Integrator::EulerExplicit;
            // const M: f64 = 1.;

            // let mut objects: Vec<Vec<f64>> = Vec::new();
            // let (x, y, u, v) = (1., 0., 0.383444, 0.377364);
            // objects.push(Vec::from([M, x, y, u, v]));
            // objects.push(Vec::from([M, -x, y, u, v]));
            // let (x, y, u, v) = (0., 0., -0.766888, -0.754728);
            // objects.push(Vec::from([M, x, y, u, v]));

            // let object_family = ObjectFamily::new(
            //     0, object_type, objects,
            //     interactions, integrator, dt, epsilon
            // );
            // object_families.push(object_family);

        }, "nbody-flowers" => {
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            // let dt: f64 = 0.025;
            let epsilon: f64 = 0.01;

            // add Sun
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let (m, x, y, u, v) = (1., 0., 0., 0., 0.);
            objects.push(Vec::from([m, x, y, u, v]));
            let tail_length = 0;
            let object_family = ObjectFamily::new(
                0, ObjectType::Static, objects, 
                interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);

            // add satellites
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            let m = 0.001;  // TODO: unphysical of course, but stable!
            let r = 0.9;
            let v0 = 0.65;
            let nr_of_satellites = 6; // TODO: make changeable
            // let mut object_colors: Vec<String> = Vec::new();
            for object_idx in 0..nr_of_satellites {
                let phi = object_idx as f64 / nr_of_satellites as f64 * TAU;
                let x = r * phi.cos();
                let y = r * phi.sin();
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                objects.push(Vec::from([m, x, y, u, v]));
            }
            let tail_length = 240;
            let mut object_family = ObjectFamily::new(
                1, ObjectType::Particle, objects,
                interactions, integrator, dt, epsilon, tail_length
            );
            // object_family.object_colors = object_colors;
            object_families.push(object_family);

        }, "nbody-binary" => { 
            const M1: f64 = 1.;
            const M2: f64 = 1.;
            let x = 0.7;
            let dt: f64 = 0.01;
            let tail_length = 1000;
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;

            // add Suns
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let v1 = 0.4*v_kepler(M2, x);
            objects.push(Vec::from([M1, -x, 0., 0., v1]));
            let v2 = 0.4*v_kepler(M1, x);
            objects.push(Vec::from([M2, x, 0., 0., -v2]));

            let object_family = ObjectFamily::new(
                0, ObjectType::Body, objects, 
                interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);



        }, "nbody-asteroids" => {  
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            let dt: f64 = 0.0035;
            // let epsilon: f64 = 0.01;

            // add Suns
            let mut objects: Vec<Vec<f64>> = Vec::new();

            const M1: f64 = 1.;
            const M2: f64 = 1.;
            let x = 0.1;

            let v1 = 0.4*v_kepler(M2, x);
            objects.push(Vec::from([M1, -x, 0., 0., v1]));

            let v2 = 0.4*v_kepler(M1, x);
            objects.push(Vec::from([M2, x, 0., 0., -v2]));

            let tail_length = 230;
            let object_family = ObjectFamily::new(
                0, ObjectType::Body, objects, 
                interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);

            const M: f64 = M1 + M2;

            // add satellites
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            let m = 0.001;  // TODO: unphysical of course, but stable!
            let N = 200; // TODO: make changeable
            for id in 0..N {

                // let mut rand: f64 = rng.gen();
                // let phi = rand * TAU;
                // let r = rand * 0.3 + 0.6;
                // let v0 = rand * v_kepler(M, r);

                // let mut rand: f64 = rng.gen();
                // let mut rand2: f64 = rng.gen();
                // let phi = rand * TAU;
                // let r = rand2 * 0.3 + 0.6;
                // let v0 = rand2 * v_kepler(M, r);

                let mut phi: f64 = rng.gen();
                phi *= TAU;
                let mut r: f64 = rng.gen();
                r *= 0.2;
                r += 0.5;
                // let mut v0: f64 = rng.gen();
                // v0 *= 0.4;
                let v0: f64 = v_kepler(M, r);

                // let phi = id as f64 / N as f64 * TAU;
                let x = r * phi.cos();
                let y = r * phi.sin();
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                objects.push(Vec::from([m, x, y, u, v]));
            }
            let tail_length = 0;
            let object_family = ObjectFamily::new(
                0, ObjectType::Particle, objects,
                interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);

            // add satellites (2)
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            let m = 0.001;  // TODO: unphysical of course, but stable!
            let N = 300; // TODO: make changeable
            for id in 0..N {

                // let mut rand: f64 = rng.gen();
                // let phi = rand * TAU;
                // let r = rand * 0.3 + 0.6;
                // let v0 = rand * v_kepler(M, r);

                // let mut rand: f64 = rng.gen();
                // let mut rand2: f64 = rng.gen();
                // let phi = rand * TAU;
                // let r = rand2 * 0.3 + 0.6;
                // let v0 = rand2 * v_kepler(M, r);

                // let mut phi: f64 = rng.gen();
                // phi *= TAU;
                let mut r: f64 = rng.gen();
                r *= 0.;
                r += 0.3;
                // let mut v0: f64 = rng.gen();
                // v0 *= 0.4;
                let v0: f64 = v_kepler(M, r);

                let phi = id as f64 / N as f64 * TAU;
                let x = r * phi.cos();
                let y = r * phi.sin();
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                objects.push(Vec::from([m, x, y, u, v]));
            }
            let tail_length = 0;
            let object_family = ObjectFamily::new(
                0, ObjectType::Particle, objects,
                interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);

        }, "nbody-solar" => {

            // Sun
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            const M: f64 = 1.;
            let (x, y, u, v) = (0., 0., 0., 0.);
            objects.push(Vec::from([M, x, y, u, v]));
            let tail_length = 0;
            let object_family = ObjectFamily::new(
                0, ObjectType::Static, objects,
                interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);

            // Earth
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            let integrator = Integrator::EulerExplicit;
            let m = 1.;
            let r = 0.9;
            let v = v_kepler(M, r) / f64::from(2).sqrt();  // TODO why sqrt(2) ?
            let (x, y, u, v) = (r, 0., 0., v);
            objects.push(Vec::from([M, x, y, u, v]));
            let tail_length = 100;  // TODO why not working?
            let object_family = ObjectFamily::new(
                0, ObjectType::Body, objects,
                interactions, integrator, dt, epsilon, tail_length
            );
            object_families.push(object_family);

        }, "charge-interaction" => {
            let nr_of_protons = 4;
            let nr_of_electrons = 40;

            let epsilon = 0.2;
            let dt = 0.001;

            let mut objects: Vec<Vec<f64>> = Vec::new();
            // add protons
            let m_p: f64 = 1.;
            for _ in 0..nr_of_protons {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();
                let (u, v, q) = (0., 0., 1.);
                objects.push(Vec::from([m_p, x, y, u, v, q]));
            }
            // add electrons
            let m_e = 1e-1;
            for _ in 0..nr_of_electrons {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();
                let (u, v, q) = (0., 0., -1.);
                objects.push(Vec::from([m_e, x, y, u, v, q]));
            }
            let interactions = Vec::from([
                ObjectInteraction::CoulombInteraction, 
                // ObjectInteraction::WallCollision,
            ]);
            let tail_length = 50;
            let mut object_family = ObjectFamily::new(
                0, ObjectType::Body, objects, interactions,
                Integrator::EulerExplicit, dt, epsilon, tail_length
            );
            object_family.attributes.push(ObjectAttribute::Charge);
            object_families.push(object_family);

        }, _ => {}
    }

    object_families
}


// pub use crate::interactions::ObjectInteraction;


// pub fn initialize_particles(page_id: &str) -> Vec<ObjectFamily> {

//     let mut particle_families: Vec<ObjectFamily> = Vec::new();

//     let integrator = match page_id {
//         _ => Integrator::EulerExplicit
//     };
//     let interactions = match page_id {
//         _ => Vec::from([ObjectInteraction::NewtonianGravity])
//     };

//     match page_id {
//         "nbody-flowers" => {
//             const N: usize = 24;  // TODO: make changeable
//             const R: f64 = 0.8; // TODO: make changeable
//             const v0: f64 = 0.1;  // TODO: "", +v_kepler
//             const dphi: f64 = 2.*3.14159 / N as f64;
//             let mut particles: Vec<f64> = Vec::new();
//             for particle_idx in 0..N {
//                 let phi = particle_idx as f64 * dphi;
//                 let x = R * phi.cos();
//                 let y = R * phi.sin();
//                 let u = v0 * phi.sin();
//                 let v = v0 * phi.cos();
//                 particles.extend_from_slice(&[x, y, u, v])
//             }
//             let object_family = ObjectFamily::new(N, particles, interactions, integrator);
//             particle_families.push(object_family);
//         },  // TODO
//         _ => {}
//     }

//     particle_families
// }

// pub fn initialize_bodies(page_id: &str) -> Vec<ObjectFamily> {

//     let mut body_families: Vec<ObjectFamily> = Vec::new();
//     // TODO: INSTEAD use tuple/vec of family lengths (?)

//     let integrator = match page_id {
//         _ => Integrator::EulerExplicit
//     };
//     let interactions = match page_id {
//         _ => Vec::from([ObjectInteraction::NewtonianGravity])
//     };

//     match page_id {
//         "3body-fig8" => {
//             const FAMILY_SIZE: usize = 3;
//             let mut bodies: Vec<f64> = Vec::new();
//             // body 1 & 3  (left & right)
//             let (x, y, u, v) = ( 0.777572, 0.628793, -0.065071, 0.632495 );
//             bodies.extend_from_slice(&[1., x, y, u, v]);
//             bodies.extend_from_slice(&[1., -x, -y, u, v]);
//             // body 2  (center)
//             let (x, y, u, v) = ( 0., 0., 0.130143, -1.264991 );
//             bodies.extend_from_slice(&[1., x, y, u, v]);
//             let object_family = ObjectFamily::new(FAMILY_SIZE, bodies, interactions, integrator);
//             body_families.push(object_family);
//         },  // TODO
//         _ => {}
//     }  

//     body_families
// }

// pub fn initialize_statics(page_id: &str) -> Vec<ObjectFamily> {

//     let mut static_families: Vec<ObjectFamily> = Vec::new();

//     let integrator = match page_id {
//         _ => Integrator::EulerExplicit
//     };
//     let interactions = match page_id {
//         _ => Vec::from([ObjectInteraction::NewtonianGravity])
//     };

//     match page_id {
//         "3body-flowers" => {
//             const M: f64 = 1.;
//             let statics = Vec::from([M, 0., 0., 0., 0.]);
//             let object_family = ObjectFamily::new(1, statics, interactions, integrator);
//             static_families.push(object_family);
//         },  // TODO
//         _ => {}
//     }

//     static_families
// }


// pub struct VectorField {
//     grid_dim: (usize, usize),
//     field_x: Vec<f64>,
//     field_y: Vec<f64>,
// }
// impl VectorField {
//     pub fn new(
//         grid_dim: (usize, usize), 
//         field_x: Vec<f64>, 
//         field_y: Vec<f64>
//     ) -> Self {
//         VectorField { grid_dim, field_x, field_y }
//     }
// }

// pub struct SpinGrid {
//     grid_dim: (usize, usize),
//     spins: Vec<bool>,
// }
// impl SpinGrid {
//     pub fn new(
//         grid_dim: (usize, usize), spins: Vec<bool>
//     ) -> Self {
//         SpinGrid { grid_dim, spins }
//     }
// }

// pub struct FloatGrid {  // TODO: choose different name?
//     grid_dim: (usize, usize),
//     cells: Vec<f64>,
// }
// impl FloatGrid {
//     pub fn new(
//         grid_dim: (usize, usize), cells: Vec<f64>
//     ) -> Self {
//         FloatGrid { grid_dim, cells }
//     }
// }








// pub fn initialize_fields(page_id: &str) -> Vec<VectorField> {

//     let mut fields: Vec<VectorField> = Vec::new();

//     match page_id {
//         _ => {}
//     }  // TODO

//     fields
// }

// pub fn initialize_cells(page_id: &str) -> Vec<FloatGrid> {

//     let mut cells: Vec<FloatGrid> = Vec::new();

//     match page_id {
//         _ => {}
//     }  // TODO

//     cells

// }

// pub fn initialize_spins(page_id: &str) -> Vec<SpinGrid> {

//     let mut spins: Vec<SpinGrid> = Vec::new();

//     match page_id {
//         _ => {}
//     }  // TODO

//     spins
// }

