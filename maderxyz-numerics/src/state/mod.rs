
use rand::{Rng};

use crate::interactions::object::Interaction as ObjectInteraction;
use crate::interactions::object::InteractionVariant as ObjectInteractionVariant;
use crate::interactions::field::Interaction as FieldInteraction;
use crate::interactions::field::InteractionVariant as FieldInteractionVariant;
use crate::integrators::object::Integrator as ObjectIntegrator;
use crate::integrators::field::Integrator as FieldIntegrator;
pub mod field;
pub use field::Field;
pub use field::FieldVariant;
pub mod object_family;
pub use object_family::ObjectFamily;
pub use object_family::ObjectType;
pub use object_family::ObjectAttribute;


#[derive(Clone)]
pub struct State {

    pub iteration_idx: usize,
    pub object_families: Vec<ObjectFamily>,
    pub fields: Vec<Field>, // TODO cell-aut + spins, field-type?

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
            let field_variant = FieldVariant::Fluid; // TODO rename, fluid density?
            let interactions = Vec::from([
                FieldInteraction::new(
                    FieldInteractionVariant::Diffusion,
                    FieldIntegrator::Entire,
                )
            ]);

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
                field_variant, 
                interactions, 
                dimensions,
                cells,
            );
            fields.push(density_field);
        }, "ising-model" => {
            let field_variant = FieldVariant::Spin; // TODO rename, fluid density?
            let interactions = Vec::from([
                FieldInteraction::new(
                    FieldInteractionVariant::SpinSpin,
                    FieldIntegrator::BatchWise,
                )
            ]);

            let mut cells: Vec<Vec<f64>> = Vec::new() ;
            const GRID_SIZE: usize = 150;
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
                field_variant, 
                interactions, 
                dimensions,
                cells,
            );
            fields.push(density_field);
        }, "game-of-life" => {
            let field_variant = FieldVariant::Spin; // TODO rename, fluid density?
            let interactions = Vec::from([
                FieldInteraction::new(
                    FieldInteractionVariant::GameOfLife,
                    FieldIntegrator::Entire,
                )
            ]);
            const GRID_SIZE: usize = 30;

            let (x, y) = (6, 2);
            let living_cells = Vec::from([
                (GRID_SIZE - 4, GRID_SIZE - 4),
                (GRID_SIZE - 4, GRID_SIZE - 5),
                (GRID_SIZE - 5, GRID_SIZE - 4),
                (GRID_SIZE - 5, GRID_SIZE - 5),
                (x+1, y),
                (x+1, y-1),
                (x,y),
                (x,y+1),
                (x-1,y-1),
            ]);

            let mut cells: Vec<Vec<f64>> = Vec::new() ;
            for row_idx in 0..GRID_SIZE {
                for col_idx in 0..GRID_SIZE {
                    // let spin_up: bool = rng.gen();
                    // let spin = match spin_up {
                    //     true => 1.,
                    //     false => -1.
                    // };

                    // let r = (
                    //     (row_idx as f64-GRID_SIZE as f64/2.).powf(2.) + 
                    //     (col_idx as f64-GRID_SIZE as f64/2.).powf(2.)
                    // ).sqrt();
                    // if r < GRID_SIZE as f64 / 10. {
                    //     density = 0.;
                    // } else {
                        // density = rng.gen();
                    // }

                    let mut spin = -1.;
                    if living_cells.contains(&(col_idx, row_idx)) {
                        spin = 1.;
                    } 
                    cells.push(Vec::from([spin]));
                }
            }
            let dimensions = (GRID_SIZE, GRID_SIZE);
            let density_field = Field::new(
                0,
                field_variant, 
                interactions, 
                dimensions,
                cells,
            );
            fields.push(density_field);
        },
        _ => {}
    }

    fields
}

fn initialize_object_families(page_id: &str) -> Vec<ObjectFamily> {

    let mut rng = rand::thread_rng();
    const TAU: f64 = 2.*3.14159;
    const DT: f64 = 0.01;
    const M: f64 = 1.;

    let v_kepler = | m: f64, r: f64 | (m / r).sqrt();

    let mut object_families: Vec<ObjectFamily> = Vec::new();
    match page_id {
        "2body-kepler" => {
        //     let epsilon = 0.01;
            let r = 0.9;
        //     let tail_length = 0;
            
            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::NewtonianGravity,
                    ObjectIntegrator::EulerExplicit,
                ),
            ]);

            let mut objects: Vec<Vec<f64>> = Vec::new();

            let (x, y, u, v) = (0., 0., 0., 0.);
            objects.push(Vec::from([M, x, y, u, v]));
            let object_family = ObjectFamily::new(
                0, ObjectType::Static, objects, interactions.clone(), DT, 
            );
            object_families.push(object_family);

            let mut objects: Vec<Vec<f64>> = Vec::new();
            // add Earth
            let m_earth = 1e-3;
            let v = 0.707*v_kepler(M, r);  // TODO: add G*M vars?
            let (x, y, u, v) = (r, 0., 0., v);
            objects.push(Vec::from([m_earth, x, y, u, v]));

        //     let tail_length = 100;
            let object_family = ObjectFamily::new(
                1, ObjectType::Body, objects, interactions, DT
            );
            object_families.push(object_family);

        }, "3body-moon" => {

            let epsilon = 0.01;
            let (r, dr) = (0.7, 0.1);
            let m_sun = M;
            let m_earth = M;
            let earth_tail_length = 100;
            let moon_tail_length = 10;
            let dt = DT; // TODO

            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::NewtonianGravity,
                    ObjectIntegrator::EulerExplicit,
                ),
            ]);

            // add Sun
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let (x, y, u, v) = (0., 0., 0., 0.);
            objects.push(Vec::from([m_sun, x, y, u, v]));
            let object_family = ObjectFamily::new(
                0, ObjectType::Static, objects, interactions.clone(), dt
            );
            object_families.push(object_family);

            // add Earth
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let v = 0.707*v_kepler(m_sun, r);  // TODO: add G*M vars?
            let (x, y, u, v) = (r, 0., 0., v);
            objects.push(Vec::from([m_earth, x, y, u, v]));
            let mut object_family = ObjectFamily::new(
                1, ObjectType::Body, objects, interactions.clone(), dt
            );
            object_family.tail_length = earth_tail_length;
            object_families.push(object_family);

            // add Moon
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let m = 1e-6;
            let v = 0.707*v_kepler(m_sun, r+dr) + 0.707*v_kepler(m_earth, dr);
            let (x, y, u, v) = (r+dr, 0., 0., v);
            objects.push(Vec::from([m, x, y, u, v]));
            let mut object_family = ObjectFamily::new(
                2, ObjectType::Particle, objects, interactions, dt
            );
            object_family.tail_length = moon_tail_length;
            object_families.push(object_family);

        }, "3body-fig8" => {

            let particle_epsilon = 0.05;
            let body_tail_length = 201;

            let xs = [0.7775727187509279270, 0., -0.7775727187509279270];
            let ys = [0.6287930240184685937, 0., -0.6287930240184685937];
            let us = [-0.06507160612095737318, 0.1301432122419148851, -0.06507160612095737318];
            let vs = [0.6324957346748190101, -1.264991469349638020, 0.6324957346748190101];

            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::NewtonianGravity,
                    ObjectIntegrator::EulerExplicit,
                ),
            ]);

            let mut bodies: Vec<Vec<f64>> = Vec::new();
            let mut particles: Vec<Vec<f64>> = Vec::new();
            for body_idx in 0..3 {

                // bodies
                let (x0, y0, u0, v0) = (xs[body_idx], ys[body_idx], us[body_idx], vs[body_idx]);
                bodies.push(Vec::from([M, x0, y0, u0, v0]));

                // particles
                let m = 0.1; // has to be non-zero, 1/0 inf, TODO remove M from particle vec
                let r = 0.05;
                let nr_of_particles = 0;
                for particle_idx in 0..nr_of_particles {
                    let phi = TAU * particle_idx as f64 / nr_of_particles as f64;
                    let x = x0 + r * phi.cos();
                    let y = y0 + r * phi.sin();
                    let u = u0 - v_kepler(M, r) * phi.sin();
                    let v = v0 + v_kepler(M, r) * phi.cos();
                    particles.push(Vec::from([m, x, y, u, v])); // TODO why does m have to be non-zero? -> 1/0 inf
                }
            }

            let mut body_family = ObjectFamily::new(
                0, ObjectType::Body, bodies, interactions.clone(), DT
            );
            body_family.tail_length = body_tail_length;
            object_families.push(body_family);
            let mut particle_family = ObjectFamily::new(
                1, ObjectType::Particle, particles, interactions, DT
            );
            particle_family.epsilon = particle_epsilon;
            object_families.push(particle_family);

            // let object_type = ObjectType::Body;
            // let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            // let integrator = ObjectIntegrator::EulerExplicit;
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
            // let integrator = ObjectIntegrator::EulerExplicit;
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

            const DT: f64 = 0.006;
            const TAIL_LENGTH: usize = 30;

            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::NewtonianGravity,
                    ObjectIntegrator::EulerExplicit,
                ),
            ]);

            let object_type = ObjectType::Body;
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let (x, y, u, v) = (-0.9892620043, 0., 0.,  1.9169244185);
            objects.push(Vec::from([M, x, y, u, v]));
            let (x, y, u, v) = (-1.2203557197, 0., 0., -2.1079512924);
            objects.push(Vec::from([M, x, y, u, v]));
            let (x, y, u, v) = ( 2.2096177241, 0., 0.,  0.1910268738);
            objects.push(Vec::from([M, x, y, u, v]));

            let mut object_family = ObjectFamily::new(
                0, object_type, objects, interactions, DT
            );
            object_family.dt = DT;
            object_family.tail_length = TAIL_LENGTH;
            object_families.push(object_family);

        }, "3body-freefall" => {

            // let object_type = ObjectType::Body;
            // let interactions = Vec::from([ObjectInteraction::NewtonianGravity]);
            // let integrator = ObjectIntegrator::EulerExplicit;
            // let epsilon: f64 = 0.;

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
            // let integrator = ObjectIntegrator::EulerExplicit;

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
            // let integrator = ObjectIntegrator::EulerExplicit;

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

            let particle_tail_length = 60;
            let epsilon: f64 = 0.01;

            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::NewtonianGravity,
                    ObjectIntegrator::EulerExplicit,
                ),
            ]);

            // add Sun
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let (m, x, y, u, v) = (1., 0., 0., 0., 0.);
            objects.push(Vec::from([m, x, y, u, v]));
            let tail_length = 0;
            let mut object_family = ObjectFamily::new(
                0, ObjectType::Static, objects, interactions.clone(), DT
            );
            object_families.push(object_family);

            // add satellites
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let m = 0.001;  // TODO: unphysical of course, but stable!
            let r = 0.9;
            let v0 = 0.65;
            let nr_of_satellites = 16; // TODO: make changeable
            // let mut object_colors: Vec<String> = Vec::new();
            for object_idx in 0..nr_of_satellites {
                let phi = object_idx as f64 / nr_of_satellites as f64 * TAU;
                let x = r * phi.cos();
                let y = r * phi.sin();
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                objects.push(Vec::from([m, x, y, u, v]));
            }
            let mut object_family = ObjectFamily::new(
                1, ObjectType::Particle, objects, interactions, DT
            );
            object_family.tail_length = particle_tail_length;
            object_family.epsilon = epsilon;
            object_families.push(object_family);

        }, "nbody-binary" => { 

            let x = 0.7;
            let z = 0.4;
            const DT: f64 = 0.1;
            const TAIL_LENGTH: usize = 1000;
            const M1: f64 = M;
            const M2: f64 = M;

            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::NewtonianGravity,
                    ObjectIntegrator::EulerExplicit,
                ),
            ]);

            // add Suns
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let v1 = z*v_kepler(M2, x);
            objects.push(Vec::from([M1, -x, 0., 0., v1]));
            let v2 = z*v_kepler(M1, x);
            objects.push(Vec::from([M2, x, 0., 0., -v2]));

            let mut object_family = ObjectFamily::new(
                0, ObjectType::Body, objects, interactions, DT
            );
            object_family.tail_length = TAIL_LENGTH;
            object_families.push(object_family);

        }, "nbody-asteroids" => {  

            let dt: f64 = 0.0035;
            let tail_length = 230;

            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::NewtonianGravity,
                    ObjectIntegrator::EulerExplicit,
                ),
            ]);

            // add Suns
            const M1: f64 = 1.;
            const M2: f64 = 1.;
            const M: f64 = M1 + M2;
            let x = 0.1;

            let mut objects: Vec<Vec<f64>> = Vec::new();
            let v1 = 0.4*v_kepler(M2, x);
            objects.push(Vec::from([M1, -x, 0., 0., v1]));
            let v2 = 0.4*v_kepler(M1, x);
            objects.push(Vec::from([M2, x, 0., 0., -v2]));

            let mut object_family = ObjectFamily::new(
                0, ObjectType::Body, objects, interactions.clone(), dt
            );
            object_family.tail_length = tail_length;
            object_family.tail_length = tail_length;
            object_families.push(object_family);

            // add satellites
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let m = 0.001;  // TODO: unphysical of course, but stable!
            let N = 300; // TODO: make changeable
            for id in 0..N {

                // let mut rand: f64 = rng.gen();
                // let phi = rand * TAU;
                // let r = rand * 0.3 + 0.6;

                let phi = id as f64 / N as f64 * TAU;
                let r = 0.3;
                let x = r * phi.cos();
                let y = r * phi.sin();
                let v0: f64 = v_kepler(M, r);
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                objects.push(Vec::from([m, x, y, u, v]));
            }
            let object_family = ObjectFamily::new(
                1, ObjectType::Particle, objects, interactions.clone(), dt
            );
            object_families.push(object_family);

            // add satellites (2)
            let mut objects: Vec<Vec<f64>> = Vec::new();
            let m = 0.001;  // TODO: unphysical of course, but stable!
            let N = 300; // TODO: make changeable
            for id in 0..N {
                let mut rand: f64 = rng.gen();
                let mut r = 0.1 * rand + 0.5;
                let phi = id as f64 / N as f64 * TAU;
                let x = r * phi.cos();
                let y = r * phi.sin();
                let v0: f64 = v_kepler(M, r);
                let u = -v0 * phi.sin();
                let v = v0 * phi.cos();
                objects.push(Vec::from([m, x, y, u, v]));
            }
            let object_family = ObjectFamily::new(
                2, ObjectType::Particle, objects, interactions, dt, 
            );
            object_families.push(object_family);

        }, "nbody-solar" => {

            // let m = 1.;
            // let r = 0.9;
            // let tail_length = 100;  // TODO why not working?

            // let interactions = Vec::from([
            //     ObjectInteraction::new(
            //         ObjectInteractionVariant::NewtonianGravity,
            //         ObjectIntegrator::EulerExplicit,
            //     ),
            // ]);

            // // Sun
            // let mut objects: Vec<Vec<f64>> = Vec::new();
            // let (x, y, u, v) = (0., 0., 0., 0.);
            // objects.push(Vec::from([M, x, y, u, v]));
            // let tail_length = 0;
            // let object_family = ObjectFamily::new(
            //     0, ObjectType::Static, objects, interactions.clone(), 
            //     dt, epsilon, tail_length
            // );
            // object_families.push(object_family);

            // // Earth
            // let mut objects: Vec<Vec<f64>> = Vec::new();
            // let v = v_kepler(M, r) / f64::from(2).sqrt();  // TODO why sqrt(2) ?
            // let (x, y, u, v) = (r, 0., 0., v);
            // objects.push(Vec::from([M, x, y, u, v]));
            // let object_family = ObjectFamily::new(
            //     0, ObjectType::Body, objects,
            //     interactions, dt, epsilon, tail_length
            // );
            // object_families.push(object_family);

        }, "charge-interaction" => {

            let epsilon = 0.2;
            let dt = 0.001;
            let nr_of_protons = 4;
            let nr_of_electrons = 40;
            let tail_length = 50;

            let interactions = Vec::from([
                ObjectInteraction::new(
                    ObjectInteractionVariant::CoulombInteraction,
                    ObjectIntegrator::EulerExplicit,
                ),
                // TODO ObjectInteraction::WallCollision,
            ]);

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
            let mut object_family = ObjectFamily::new(
                0, ObjectType::Body, objects, interactions, dt, 
            );
            object_family.tail_length = tail_length;
            object_family.epsilon = epsilon;
            object_family.attributes.push(ObjectAttribute::Charge);
            object_families.push(object_family);

        }, _ => {}
    }

    object_families
}

