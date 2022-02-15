#![allow(non_snake_case)]


use rand::Rng;

pub mod field;
pub mod object;
use crate::boundary::field::variant::FieldBoundaryVariant;
use crate::boundary::object::variant::ObjBoundaryVariant;
use crate::config::EngineConfig;
use crate::config::field::FieldEngineConfig;
use crate::config::obj_family::ObjFamilyEngineConfig;
use crate::integrator::field::FieldIntegrator;
use crate::integrator::field::FieldIntegratorVariant;
use crate::interaction::field::field::FieldFieldInteraction;
use crate::interaction::field::object::FieldObjInteraction;
use crate::interaction::object::field::ObjFieldInteraction;
use crate::interaction::object::object::ObjObjInteraction;
use field::Field;
use field::FieldVariant;
use object::ObjFamily;
use object::attribute::ObjAttribute;
use object::variant::ObjVariant;


#[derive(Clone)]
pub struct State {

    pub obj_families: Vec<ObjFamily>,
    pub fields:       Vec<Field>,

}
impl State {

    pub fn new( 
        sim_id: &str, 
        engine_conf: &mut EngineConfig 
    ) -> Self {
        State {
            obj_families: Self::setup_objects(sim_id, engine_conf),
            fields:       Self::setup_fields( sim_id, engine_conf),
        }
    }

    pub fn setup_objects(
        sim_id: &str,
        engine_conf: &mut EngineConfig,
    ) -> Vec<ObjFamily> {

        // math stuff
        const TAU: f64 = 2. * 3.14159265358979;
        let mut rng = rand::thread_rng();
        // default values
        const M: f64 = 1.;
        let DT = engine_conf.dt;  // 0.01

        let mut obj_families: Vec<ObjFamily> = Vec::new();
        match sim_id {
            "nbody-misc" => {

                let id = 0;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);
                fam_conf.boundary = ObjBoundaryVariant::WallCollisionInelastic;

                let mut family = ObjFamily::new(id);

                let (mass, distance, speed, nr_of_objects) = (0.05, 0.7, 1.0, 12);

                for obj_idx in 0..nr_of_objects {
                    // let rand: f64 = rng.gen(); let phi = TAU * rand;
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    let x = distance * phi.cos();
                    let y = distance * phi.sin();
                    let u = -speed * phi.sin();
                    let v =  speed * phi.cos();

                    let object = Vec::from([mass, x, y, u, v]);
                    family.add_object(&object, &mut fam_conf);
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

            }, "nbody-cloud" => {

                let id = 0;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);

                let mut family = ObjFamily::new(id);

                let (speed, nr_of_objects) = (0., 10);
                for obj_idx in 0..nr_of_objects {
                    let rand: f64 = rng.gen(); 
                    let rand2: f64 = rng.gen(); 
                    let rand3: f64 = rng.gen(); 
                    let rand4: f64 = rng.gen(); 

                    let x = rand*2. - 1.;
                    let y = rand2*2. - 1.;
                    let u = -speed * (rand3*2.-1.);
                    let v =  speed * (rand4*2.-1.);

                    let object = Vec::from([0.1, x, y, u, v]);
                    family.add_object(&object, &mut fam_conf);
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

            }, "nbody-flowers" => {

                // SATTELITES  (particles)
                // ===============================================================================

                let id = 0;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);
                fam_conf.obj_variant = ObjVariant::Particle;

                let mut family = ObjFamily::new(id);

                let (R, speed, nr_of_objects) = (0.85, 0.7, 32);
                for obj_idx in 0..nr_of_objects {
                    // let rand: f64 = rng.gen(); let phi = TAU * rand;
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    let x =      R * phi.cos();
                    let y =      R * phi.sin();
                    let u = -speed * phi.sin();
                    let v =  speed * phi.cos();

                    let object = Vec::from([0.01, x, y, u, v]);
                    family.add_object(&object, &mut fam_conf);
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

                // STAR  (static)
                // ===============================================================================

                let id = 1;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);
                fam_conf.obj_variant = ObjVariant::Static;

                let mut family = ObjFamily::new(1);

                let object = Vec::from([1., 0., 0., 0., 0.]);
                family.add_object(&object, &mut fam_conf);

                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

            }, "nbody-asteroids" => {

                engine_conf.dt = 0.5 * DT;
                let (nr_of_stars, nr_of_objects) = (2, 1000);
                let m = 1.0;

                // ASTEROID BELT  (particle)
                // ===============================================================================

                let id = 0;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);
                fam_conf.obj_variant = ObjVariant::Particle;

                let mut family = ObjFamily::new(0);

                // let R = 0.85;
                // let W = 0.;
                let R = 0.55;
                let W = 0.2;
                let speed = (nr_of_stars as f64 * M / R).powf(0.5);

                for obj_idx in 0..nr_of_objects {
                    let rand: f64 = rng.gen();
                    // let phi = TAU * rand;
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    let dR = rand * W;

                    let x = (R+dR) * phi.cos();
                    let y = (R+dR) * phi.sin();
                    let u = -speed * phi.sin();
                    let v = speed * phi.cos();

                    let object = Vec::from([m, x, y, u, v]);
                    family.add_object(&object, &mut fam_conf);
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

                // STELLAR BINARY
                // ===============================================================================

                let id = 1;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);

                let mut family = ObjFamily::new(1);

                let R: f64 = 0.15;
                let speed = match nr_of_stars {
                    1 => 0.,
                    2 => 0.7*(M / (2.*R) as f64).sqrt(),
                    _ => (M * (nr_of_stars-1) as f64 / (2.*R)).powf(0.5)
                };
                for star_idx in 0..nr_of_stars {
                    let phi = star_idx as f64 / nr_of_stars as f64 * TAU;
                    let x =      R * phi.cos();
                    let y =      R * phi.sin();
                    let u = -speed * phi.sin();
                    let v =  speed * phi.cos();
                    let object = Vec::from([M, x, y, u, v]);
                    family.add_object(&object, &mut fam_conf);
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

            }, "3body-fig8" => {

                let id = 0;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);

                let mut family = ObjFamily::new(0);

                let xs = [ 0.7775727187509279270,  0.,                    -0.7775727187509279270];
                let ys = [ 0.6287930240184685937,  0.,                    -0.6287930240184685937];
                let us = [-0.06507160612095737318, 0.1301432122419148851, -0.06507160612095737318];
                let vs = [ 0.6324957346748190101, -1.264991469349638020,   0.6324957346748190101]; 

                for body_idx in 0..3 {
                    let (x0, y0) = (xs[body_idx], ys[body_idx]);
                    let (u0, v0) = (us[body_idx], vs[body_idx]);
                    let object = Vec::from([M, x0, y0, u0, v0]);
                    family.add_object(&object, &mut fam_conf);
                }

                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

            // ising

            }, "charge-interaction" => {

                let id = 0;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);
                fam_conf.obj_interactions = Vec::from([ObjObjInteraction::ForceCoulomb]);
                fam_conf.boundary = ObjBoundaryVariant::WallCollisionInelastic;
                fam_conf.obj_attributes.push(ObjAttribute::Charge);

                let mut family = ObjFamily::new(id);
                let nr_of_bodies = 1;
                for _ in 0..nr_of_bodies {
                    let rand1: f64 = rng.gen(); 
                    let rand2: f64 = rng.gen(); 
                    let x0 = rand1 * 2. - 1.;
                    let y0 = rand2 * 2. - 1.;
                    let object = Vec::from([M, x0, y0, 0., 0., 1.]);
                    family.add_object(&object, &mut fam_conf);
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

                let id = 1;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);
                fam_conf.obj_interactions = Vec::from([ObjObjInteraction::ForceCoulomb]);
                fam_conf.boundary = ObjBoundaryVariant::WallCollisionElastic;
                fam_conf.obj_attributes.push(ObjAttribute::Charge);

                let mut family = ObjFamily::new(id);
                let nr_of_bodies = 1;
                for body_idx in 0..nr_of_bodies {
                    let rand1: f64 = rng.gen(); 
                    let rand2: f64 = rng.gen(); 
                    let x0 = rand1 * 2. - 1.;
                    let y0 = rand2 * 2. - 1.;
                    let object = Vec::from([0.01, x0, y0, 0., 0., -1.]);
                    family.add_object(&object, &mut fam_conf);
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

            }, "lennard-jones" => {

                let id = 0;
                let mut fam_conf = ObjFamilyEngineConfig::new(id);
                fam_conf.obj_interactions = Vec::from([ObjObjInteraction::ForceLennardJones]);
                fam_conf.boundary = ObjBoundaryVariant::WallCollisionElastic;

                let mut family = ObjFamily::new(id);

                // TODO add dampening somehow, on collision? over time?
                let foo: usize = 5;
                let nr_of_bodies = foo.pow(2);
                let speed = 0.05;
                for jdx in 0..foo {
                    for idx in 0..foo {
                        let x0 = (idx as f64 + 0.5) / foo as f64 * 2. - 1.;
                        let y0 = (jdx as f64 + 0.5) / foo as f64 * 2. - 1.;
                        let rand1: f64 = rng.gen(); 
                        let rand2: f64 = rng.gen(); 
                        let u0 = (rand1 * 2. - 1.) * speed;
                        let v0 = (rand2 * 2. - 1.) * speed;
                        // let (u0, v0) = (0., 0.);
                        let x0 = x0 + rand1 / foo as f64;
                        let y0 = y0 + rand2 / foo as f64;
                        let object = Vec::from([1., x0, y0, u0, v0]);
                        family.add_object(&object, &mut fam_conf);
                    }
                }
                engine_conf.obj_families.push(fam_conf);
                obj_families.push(family);

            }, _ => {}
        }
        obj_families
    }

    pub fn setup_fields(
        sim_id: &str,
        // engine_setup: &mut EngineSetup,
        engine_config: &mut EngineConfig,
    ) -> Vec<Field> {

        let mut rng = rand::thread_rng();

        let mut fields: Vec<Field> = Vec::new();

        let GRID_SIZE = 200;

        match sim_id {
            "ising-model" => {

                let id = 0;
                let mut field_conf = FieldEngineConfig::new(id);
                field_conf.field_variant = FieldVariant::Ising;
                field_conf.dimensions = Vec::from([GRID_SIZE, GRID_SIZE, 1]);
                field_conf.integrator = FieldIntegratorVariant::RandomBatch;
                field_conf.field_interactions = Vec::from([FieldFieldInteraction::Ising]);
                field_conf.boundary = FieldBoundaryVariant::Periodic;

                let mut field = Field::new(id);
                for row_idx in 0..field_conf.dimensions[0] {
                    for col_idx in 0..field_conf.dimensions[1] {
                        let rand: f64 = rng.gen();
                        let val = if rand > 0.5 { -1. } else { 1. };
                        field.entries.push(val);
                    }
                }
                engine_config.fields.push(field_conf);
                fields.push(field);

            }, "game-of-life" => {

                let id = 0;
                let mut field_conf = FieldEngineConfig::new(id);
                field_conf.field_variant = FieldVariant::GameOfLife;
                field_conf.dimensions = Vec::from([GRID_SIZE, GRID_SIZE, 1]);
                // field_conf.integrator = FieldIntegratorVariant::Entire;
                field_conf.integrator = FieldIntegratorVariant::RandomBatch;
                field_conf.field_interactions = Vec::from([FieldFieldInteraction::GameOfLife]);
                field_conf.boundary = FieldBoundaryVariant::None;

                let mut field = Field::new(id);
                for row_idx in 0..field_conf.dimensions[0] {
                    for col_idx in 0..field_conf.dimensions[1] {
                        let rand: f64 = rng.gen();
                        let val = if rand > 0.5 { -1. } else { 1. };
                        field.entries.push(val);
                    }
                }
                engine_config.fields.push(field_conf);
                fields.push(field);

            }, _ => {}
        }

        fields
    }

}

