#![allow(non_snake_case)]

use rand::Rng;

pub mod field;
pub mod object;

// use field;
use field::Field;
use field::FieldVariant;
// use object;  // TODO use more name-spacing
use object::ObjectFamily;
use object::ObjectVariant;
use object::ObjectAttribute;

use crate::setup::EngineSetup;
use crate::integrator::field::FieldIntegrator;
use crate::integrator::object::ObjectIntegrator;
use crate::integrator::object::IntegratorVariant as ObjectIntegratorVariant;
use crate::integrator::field::IntegratorVariant as FieldIntegratorVariant;

use crate::interaction::object as object_interactions;
use crate::interaction::field as field_interactions;
use crate::boundary::object::ObjectBoundary;
use crate::boundary::object::variant::BoundaryVariant as ObjectBoundaryVariant;

use crate::config::EngineConfig;
use crate::config::field::FieldEngineConfig;
use crate::config::obj_family::ObjFamilyEngineConfig;

// use interaction;
// use integrator;
// use boundary


#[derive(Clone)]
pub struct State {
    pub object_families: Vec<ObjectFamily>,
    pub fields: Vec<Field>,
}
impl State {

    pub fn new(
        sim_id: &str, 
        // integrators: &mut EngineSetup,
        engine_config: &mut EngineConfig,
    ) -> Self {
        State {
            object_families: Self::setup_objects(
                sim_id, 
                // integrators, 
                engine_config
            ),
            fields: Self::setup_fields(
                sim_id, 
                // integrators, 
                engine_config
            ),
        }
    }

    pub fn init(&mut self) {
        // TODO call self.setup_objectss/fields from here
    }

    pub fn setup_objects(
        sim_id: &str,
        // engine_setup: &mut EngineSetup,
        engine_config: &mut EngineConfig,
    ) -> Vec<ObjectFamily> {

        const TAU: f64 = 2. * 3.14159265358979;

        const OBJECT_INTEGRATOR_VARIANT: ObjectIntegratorVariant = ObjectIntegratorVariant::EulerExplicit;
        // TODO find better way to do stuff below
        const OBJECT_ATTRIBUTES: [ObjectAttribute; 5] = [
            ObjectAttribute::Mass,
            ObjectAttribute::PositionX,
            ObjectAttribute::PositionY,
            ObjectAttribute::VelocityX,
            ObjectAttribute::VelocityY,
        ];
        const CHARGED_OBJECT_ATTRIBUTES: [ObjectAttribute; 6] = [
            ObjectAttribute::Mass,
            ObjectAttribute::Charge,
            ObjectAttribute::PositionX,
            ObjectAttribute::PositionY,
            ObjectAttribute::VelocityX,
            ObjectAttribute::VelocityY,
        ];
        const M: f64 = 1.;
        let DT = 0.01;  // TODO get from config (update config on slider change)
        let mut rng = rand::thread_rng();

        let mut object_families: Vec<ObjectFamily> = Vec::new();

        // let mut config = EngineConfig::new();
        let mut config = engine_config;

        match sim_id {
            "nbody-misc" => {

                let id = 0;
                let mut family = ObjFamilyEngineConfig::new(id, config.dt);
                family.boundaries[0].variant = ObjectBoundaryVariant::WallCollisionInelastic;
                config.obj_families.push(family);

                // OBJECT FAMILIES
                // -------------------------------------------------------------------------------

                let mut family = ObjectFamily::new(
                    id, 
                    ObjectVariant::Body, 
                    // Vec::from(OBJECT_ATTRIBUTES)
                );

                let R = 0.7;
                let speed = 1.0;
                let nr_of_objects = 12;

                // objects
                for obj_idx in 0..nr_of_objects {
                    // let rand: f64 = rng.gen(); let phi = TAU * rand;
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    let x = R * phi.cos();
                    let y = R * phi.sin();
                    let u = -speed * phi.sin();
                    let v =  speed * phi.cos();

                    let object = Vec::from([0.05, x, y, u, v]);
                    family.add_object(&object);
                }

                object_families.push(family);

//             }, "nbody-cloud" => {

//                 // OBJECT FAMILIES
//                 // -------------------------------------------------------------------------------

//                 let mut family = ObjectFamily::new(
//                     0, 
//                     ObjectVariant::Body, 
//                     Vec::from(OBJECT_ATTRIBUTES)
//                 );
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT,  // TODO make default?
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceNewtonianGravity,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(
//                     ObjectBoundaryVariant::None
//                 );
//                 engine_setup.object_boundaries.push(boundary);

//                 let speed = 0.;
//                 let nr_of_objects = 10;

//                 // objects
//                 for obj_idx in 0..nr_of_objects {
//                     let rand: f64 = rng.gen(); 
//                     let rand2: f64 = rng.gen(); 
//                     let rand3: f64 = rng.gen(); 
//                     let rand4: f64 = rng.gen(); 

//                     let x = rand*2. - 1.;
//                     let y = rand2*2. - 1.;
//                     let u = -speed * (rand3*2.-1.);
//                     let v =  speed * (rand4*2.-1.);

//                     let object = Vec::from([0.1, x, y, u, v]);
//                     family.add_object(&object);
//                 }

//                 object_families.push(family);
//                 // config.obj_families.push(family_config);

//             }, "charge-interaction" => {

//                 let DT = 0.0005;

//                 let mut family = ObjectFamily::new(
//                     0, ObjectVariant::Body, Vec::from(CHARGED_OBJECT_ATTRIBUTES)
//                 );
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceCoulomb,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::WallCollisionElastic);
//                 engine_setup.object_boundaries.push(boundary);

//                 // objects
//                 let nr_of_bodies = 16;
//                 for body_idx in 0..nr_of_bodies {
//                     let rand1: f64 = rng.gen(); 
//                     let rand2: f64 = rng.gen(); 
//                     let x0 = rand1 * 2. - 1.;
//                     let y0 = rand2 * 2. - 1.;
//                     let object = Vec::from([1., x0, y0, 0., 0., 1.]);
//                     family.add_object(&object);
//                 }
//                 object_families.push(family);

//                 let mut family = ObjectFamily::new(
//                     1, ObjectVariant::Body, Vec::from(CHARGED_OBJECT_ATTRIBUTES)
//                 );
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceCoulomb,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::WallCollisionElastic);
//                 engine_setup.object_boundaries.push(boundary);

//                 // objects
//                 let nr_of_bodies = 25;
//                 for body_idx in 0..nr_of_bodies {
//                     let rand1: f64 = rng.gen(); 
//                     let rand2: f64 = rng.gen(); 
//                     let x0 = rand1 * 2. - 1.;
//                     let y0 = rand2 * 2. - 1.;
//                     let object = Vec::from([0.01, x0, y0, 0., 0., -1.]);
//                     family.add_object(&object);
//                 }
//                 object_families.push(family);

//             }, "lennard-jones" => {

//                 let mut family = ObjectFamily::new(
//                     0, ObjectVariant::Body, Vec::from(OBJECT_ATTRIBUTES)
//                 );
//                 // integrator
//                 let DT = 0.01;
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceLennardJones,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::WallCollisionInelastic);
//                 engine_setup.object_boundaries.push(boundary);

//                 // TODO add dampening somehow, on collision? over time?

//                 // objects
//                 let foo: usize = 5;
//                 let nr_of_bodies = foo.pow(2);
//                 let speed = 0.1;
//                 for jdx in 0..foo {
//                     for idx in 0..foo {
//                         let x0 = (idx as f64 + 0.5) / foo as f64 * 2. - 1.;
//                         let y0 = (jdx as f64 + 0.5) / foo as f64 * 2. - 1.;
//                         let rand1: f64 = rng.gen(); 
//                         let rand2: f64 = rng.gen(); 
//                         let u0 = (rand1 * 2. - 1.) * speed;
//                         let v0 = (rand2 * 2. - 1.) * speed;
//                         // let (u0, v0) = (0., 0.);
//                         let x0 = x0 + rand1 / foo as f64;
//                         let y0 = y0 + rand2 / foo as f64;
//                         let object = Vec::from([1., x0, y0, u0, v0]);
//                         family.add_object(&object);
//                     }
//                 }
//                 // for body_idx in 0..nr_of_bodies {
//                 //     let x0 = rand1 * 2. - 1.;
//                 //     let y0 = rand2 * 2. - 1.;
//                 //     // let x0 = f64::from(body_idx % nr_of_bodies) * 2. - 1.;
//                 //     // let y0 = f64::from((body_idx - (body_idx % nr_of_bodies)) / nr_of_bodies) * 2. - 1.;
//                 //     let object = Vec::from([1., x0, y0, 0., 0.]);
//                 //     family.add_object(&object);
//                 // }
//                 object_families.push(family);

//             }, "3body-fig8" => {

//                 let mut family = ObjectFamily::new(
//                     0, ObjectVariant::Body, Vec::from(OBJECT_ATTRIBUTES)
//                 );
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceNewtonianGravity,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::None);
//                 engine_setup.object_boundaries.push(boundary);

//                 // objects
//                 let xs = [ 0.7775727187509279270,  0.,                    -0.7775727187509279270];
//                 let ys = [ 0.6287930240184685937,  0.,                    -0.6287930240184685937];
//                 let us = [-0.06507160612095737318, 0.1301432122419148851, -0.06507160612095737318];
//                 let vs = [ 0.6324957346748190101, -1.264991469349638020,   0.6324957346748190101]; 
//                 let m = 1.;

//                 for body_idx in 0..3 {
//                     let (x0, y0) = (xs[body_idx], ys[body_idx]);
//                     let (u0, v0) = (us[body_idx], vs[body_idx]);
//                     let object = Vec::from([m, x0, y0, u0, v0]);
//                     family.add_object(&object);
//                 }

//                 object_families.push(family);

//             }, "nbody-flowers" => {

//                 // SATTELITES  (particles)
//                 // ===============================================================================
//                 let mut family = ObjectFamily::new(
//                     0, ObjectVariant::Particle, Vec::from(OBJECT_ATTRIBUTES));
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceNewtonianGravity,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::None);
//                 engine_setup.object_boundaries.push(boundary);
//                 // objects
//                 let R = 0.85;
//                 let speed = 0.7;
//                 let nr_of_objects = 32;

//                 for obj_idx in 0..nr_of_objects {
//                     // let rand: f64 = rng.gen(); let phi = TAU * rand;
//                     let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
//                     let x =      R * phi.cos();
//                     let y =      R * phi.sin();
//                     let u = -speed * phi.sin();
//                     let v =  speed * phi.cos();

//                     let object = Vec::from([0.01, x, y, u, v]);
//                     family.add_object(&object);
//                 }
//                 object_families.push(family);

//                 // STAR  (static)
//                 // ===============================================================================
//                 let mut family = ObjectFamily::new(
//                     1, ObjectVariant::Static, Vec::from(OBJECT_ATTRIBUTES)
//                 );
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceNewtonianGravity,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::None);
//                 engine_setup.object_boundaries.push(boundary);
//                 // object
//                 let object = Vec::from([1., 0., 0., 0., 0.]);
//                 family.add_object(&object);
//                 object_families.push(family);

//             }, "nbody-asteroids" => {

//                 let DT = 0.5 * DT;

//                 let nr_of_stars = 2;
//                 let nr_of_objects = 1000;
//                 let m = 1.0;

//                 // ASTEROID BELT  (particle)
//                 // ===============================================================================
//                 let mut family = ObjectFamily::new(
//                     0, ObjectVariant::Particle, Vec::from(OBJECT_ATTRIBUTES)
//                 );
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceNewtonianGravity,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::None);
//                 engine_setup.object_boundaries.push(boundary);
//                 // objects
//                 let R = 0.85;
//                 let W = 0.0;
//                 let speed = (nr_of_stars as f64 * M / R).powf(0.5);

//                 for obj_idx in 0..nr_of_objects {
//                     let rand: f64 = rng.gen();
//                     // let phi = TAU * rand;
//                     let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
//                     let dR = rand * W;

//                     let x = (R+dR) * phi.cos();
//                     let y = (R+dR) * phi.sin();
//                     let u = -speed * phi.sin();
//                     let v = speed * phi.cos();

//                     let object = Vec::from([m, x, y, u, v]);
//                     family.add_object(&object);
//                 }
//                 object_families.push(family);

//                 // STELLAR BINARY
//                 // ===============================================================================
//                 let mut family = ObjectFamily::new(
//                     1, ObjectVariant::Body, Vec::from(OBJECT_ATTRIBUTES) // TODO static on rail
//                 );
//                 // integrator
//                 let integrator = ObjectIntegrator::new(
//                     OBJECT_INTEGRATOR_VARIANT, 
//                     DT, 
//                     Vec::from([]),  // object-field interactions
//                     Vec::from([
//                         object_interactions::object::Interaction::ForceNewtonianGravity,
//                     ])  // object-object interactions
//                 ); 
//                 engine_setup.object_integrators.push(integrator);
//                 // boundaries
//                 let boundary = ObjectBoundary::new(ObjectBoundaryVariant::None);
//                 engine_setup.object_boundaries.push(boundary);

//                 let R: f64 = 0.15;
//                 let speed = match nr_of_stars {
//                     1 => 0.,
//                     2 => 0.7*(M / (2.*R) as f64).sqrt(),
//                     _ => (M * (nr_of_stars-1) as f64 / (2.*R)).powf(0.5)
//                 };
//                 for star_idx in 0..nr_of_stars {
//                     let phi = star_idx as f64 / nr_of_stars as f64 * TAU;
//                     let x =      R * phi.cos();
//                     let y =      R * phi.sin();
//                     let u = -speed * phi.sin();
//                     let v =  speed * phi.cos();
//                     let object = Vec::from([M, x, y, u, v]);
//                     family.add_object(&object);
//                 }
//                 object_families.push(family);

            }, _ => {}
        }
        object_families
    }

    pub fn setup_fields(
        sim_id: &str,
        // engine_setup: &mut EngineSetup,
        engine_config: &mut EngineConfig,
    ) -> Vec<Field> {

        let mut rng = rand::thread_rng();

        let mut fields: Vec<Field> = Vec::new();

        match sim_id {
            "ising-model" => {

                let GRID_SIZE = 200;
                let dimensions = (GRID_SIZE, GRID_SIZE, 1);
                let mut entries = Vec::new();
                for row_idx in 0..dimensions.0 {
                    for col_idx in 0..dimensions.1 {
                        let rand: f64 = rng.gen();
                        let val = if rand > 0.5 { -1. } else { 1. };
                        entries.push(val);
                    }
                }
                let field = Field::new(
                    0, FieldVariant::Ising, dimensions, entries
                );

                // integrator
                let integrator = FieldIntegrator::new(
                    FieldIntegratorVariant::RandomBatch,
                    Vec::from([
                        crate::interaction::field::field::Interaction::Ising,
                    ]),
                    Vec::new(),
                );
                // let integrator = ObjectIntegrator::new(
                //     OBJECT_INTEGRATOR_VARIANT, 
                //     DT, 
                //     Vec::from([]),  // object-field interactions
                //     Vec::from([
                //         object_interactions::object::Interaction::ForceNewtonianGravity,
                //     ])  // object-object interactions
                // ); 

        // engine_setup.field_integrators.push(integrator);

                // let field_config = FieldEngineConfig {
                //     id: 0,
                //     dimensions: Vec::from([100, 100]),
                // };
                // engine_config.fields.push(field_config);


                // mxyz_utils::dom::console::log("aba");
                // // boundaries
                // let boundary = ObjectBoundary::new(ObjectBoundaryVariant::None);
                // engine_setup.object_boundaries.push(boundary);

                fields.push(field);
            }, _ => {}
        }

        fields
    }

}

