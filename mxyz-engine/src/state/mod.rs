#![allow(non_snake_case)]

use rand::Rng;

pub mod field;
pub mod object;
use field::Field;
use object::ObjFamily;

use crate::config::EngineConfig;
use crate::config::field::FieldEngineConfig;
use crate::config::obj_family::ObjFamilyEngineConfig;

use mxyz_physics::classical_mechanics::newtonian_gravity::kepler_velocity;


pub fn get_cell_idx_from_coords(
    x: usize,
    y: usize,
    field: &Field,
    field_conf: &FieldEngineConfig,
) -> usize {
    let dimensions = &field_conf.dimensions;
    let (dim_x, dim_y, dim_z) = (dimensions[0], dimensions[1], dimensions[2]);
    y * dim_x + x

}


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

        // object-specific imports
        use object::attribute::ObjAttribute;
        use object::variant::ObjVariant;
        use crate::boundary::object::variant::ObjBoundaryVariant;
        // use crate::integrator::object::variant::ObjIntegratorVariant;
        // use crate::interaction::object::field::ObjFieldInteraction;
        use crate::interaction::object::object::ObjObjInteraction;

        // math stuff
        const TAU: f64 = 2.*3.14159265358979;
        let mut rng = rand::thread_rng();

        // default values
        const M: f64 = 1.;
        let DT = engine_conf.dt;  // 0.01

        let mut obj_families: Vec<ObjFamily> = Vec::new();
        match sim_id {

            "3body-moon" => {

                let (m,   r) = (0.1,   0.8);  // Earth
                let (mu, dr) = (0.001, 0.1);  // Moon

                let id = 0;
                let mut family      = ObjFamily::new(id);
                let mut conf        = ObjFamilyEngineConfig::new(id);
                conf.obj_variant    = ObjVariant::Static;
                let (x, y, u, v) = (0., 0., 0., 0.);
                let object = Vec::from([M, x, y, u, v]);
                family.add_object(&object, &mut conf);
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

                let id = 1;
                let mut family      = ObjFamily::new(id);
                let mut conf        = ObjFamilyEngineConfig::new(id);
                conf.obj_variant    = ObjVariant::Body;
                let (x, y, u, v) = (r, 0., 0., kepler_velocity(M, r));
                let object = Vec::from([m, x, y, u, v]);
                family.add_object(&object, &mut conf);
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

                let id = 2;
                let mut family      = ObjFamily::new(id);
                let mut conf        = ObjFamilyEngineConfig::new(id);
                conf.obj_variant    = ObjVariant::Particle;
                let (x, y, u, v) = (r+dr, 0., 0., v);
                let object = Vec::from([mu, x, y, u, v+kepler_velocity(m, dr)]);
                family.add_object(&object, &mut conf);
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, "nbody-flowers" => {

                // SATTELITES  (particles)
                // ===============================================================================

                let id = 0;
                let mut family      = ObjFamily::new(id);
                let mut conf        = ObjFamilyEngineConfig::new(id);
                conf.obj_variant    = ObjVariant::Particle;

                let (R, speed, nr_of_objects) = (0.85, 0.7, 32);
                for obj_idx in 0..nr_of_objects {
                    // let rand: f64 = rng.gen(); let phi = TAU * rand;
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    let x =      R * phi.cos();
                    let y =      R * phi.sin();
                    let u = -speed * phi.sin();
                    let v =  speed * phi.cos();

                    let object = Vec::from([0.01, x, y, u, v]);
                    family.add_object(&object, &mut conf);
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

                // STAR  (static)
                // ===============================================================================

                let id = 1;
                let mut family      = ObjFamily::new(id);
                let mut conf        = ObjFamilyEngineConfig::new(id);
                conf.obj_variant    = ObjVariant::Static;

                let object = Vec::from([1., 0., 0., 0., 0.]);
                family.add_object(&object, &mut conf);

                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, "nbody-asteroids" => {

                engine_conf.dt = 0.5 * DT;
                let (nr_of_stars, nr_of_objects) = (2, 1000);
                let m = 1.0;

                // ASTEROID BELT  (particle)
                // ===============================================================================

                let id = 0;
                let mut family      = ObjFamily::new(id);
                let mut conf        = ObjFamilyEngineConfig::new(id);
                conf.obj_variant    = ObjVariant::Particle;

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
                    family.add_object(&object, &mut conf);
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

                // STELLAR BINARY
                // ===============================================================================

                let id = 1;
                let mut family  = ObjFamily::new(id);
                let mut conf    = ObjFamilyEngineConfig::new(id);

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
                    family.add_object(&object, &mut conf);
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, "3body-fig8" => {

                let id = 0;
                let mut family  = ObjFamily::new(id);
                let mut conf    = ObjFamilyEngineConfig::new(id);

                let xs = [ 0.7775727187509279270,  0.,                    -0.7775727187509279270];
                let ys = [ 0.6287930240184685937,  0.,                    -0.6287930240184685937];
                let us = [-0.06507160612095737318, 0.1301432122419148851, -0.06507160612095737318];
                let vs = [ 0.6324957346748190101, -1.264991469349638020,   0.6324957346748190101]; 

                for body_idx in 0..3 {
                    let (x0, y0) = (xs[body_idx], ys[body_idx]);
                    let (u0, v0) = (us[body_idx], vs[body_idx]);
                    let object = Vec::from([M, x0, y0, u0, v0]);
                    family.add_object(&object, &mut conf);
                }

                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, "nbody-cloud" => {

                let id = 0;
                let mut family  = ObjFamily::new(id);
                let mut conf    = ObjFamilyEngineConfig::new(id);

                let (speed, nr_of_objects) = (0., 10);
                for _obj_idx in 0..nr_of_objects {
                    let rand: f64 = rng.gen(); 
                    let rand2: f64 = rng.gen(); 
                    let rand3: f64 = rng.gen(); 
                    let rand4: f64 = rng.gen(); 

                    let x = rand*2. - 1.;
                    let y = rand2*2. - 1.;
                    let u = -speed * (rand3*2.-1.);
                    let v =  speed * (rand4*2.-1.);

                    let object = Vec::from([0.1, x, y, u, v]);
                    family.add_object(&object, &mut conf);
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, "nbody-misc" => {

                let (mass, distance, speed, nr_of_objects) = (0.05, 0.7, 1.0, 12);

                let id = 0;
                let mut family  = ObjFamily::new(id);
                let mut conf    = ObjFamilyEngineConfig::new(id);
                conf.boundary   = ObjBoundaryVariant::WallCollisionInelastic;

                for obj_idx in 0..nr_of_objects {
                    // let rand: f64 = rng.gen(); let phi = TAU * rand;
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    let x = distance * phi.cos();
                    let y = distance * phi.sin();
                    let u =   -speed * phi.sin();
                    let v =    speed * phi.cos();

                    let object = Vec::from([mass, x, y, u, v]);
                    family.add_object(&object, &mut conf);
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, "charge-interaction" => {

                let id = 0;
                let mut family        = ObjFamily::new(id);
                let mut conf          = ObjFamilyEngineConfig::new(id);
                conf.obj_interactions = Vec::from([ObjObjInteraction::ForceCoulomb]);
                conf.boundary         = ObjBoundaryVariant::WallCollisionElastic;
                conf.obj_attributes.push(ObjAttribute::Charge);
                conf.obj_length += 1;
                engine_conf.dt = DT / 10.;

                let nr_of_protons = 10;
                for _ in 0..nr_of_protons {
                    let rand1: f64 = rng.gen(); 
                    let rand2: f64 = rng.gen(); 
                    let x0 = rand1 * 2. - 1.;
                    let y0 = rand2 * 2. - 1.;
                    let object = Vec::from([M, x0, y0, 0., 0., 1.]);
                    family.add_object(&object, &mut conf);
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

                let id = 1;
                let mut family        = ObjFamily::new(id);
                let mut conf          = ObjFamilyEngineConfig::new(id);
                conf.obj_interactions = Vec::from([ObjObjInteraction::ForceCoulomb]);
                conf.boundary         = ObjBoundaryVariant::WallCollisionElastic;
                conf.obj_attributes.push(ObjAttribute::Charge);
                conf.obj_length += 1;

                let nr_of_electrons = 1;
                for _body_idx in 0..nr_of_electrons {
                    let rand1: f64 = rng.gen(); 
                    let rand2: f64 = rng.gen(); 
                    let x0 = rand1 * 2. - 1.;
                    let y0 = rand2 * 2. - 1.;
                    let object = Vec::from([0.01, x0, y0, 0., 0., -1.]);
                    family.add_object(&object, &mut conf);
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, "lennard-jones" => {

                let id = 0;
                let mut family        = ObjFamily::new(id);
                let mut conf          = ObjFamilyEngineConfig::new(id);
                conf.obj_interactions = Vec::from([ObjObjInteraction::ForceLennardJones]);
                conf.boundary         = ObjBoundaryVariant::WallCollisionElastic;

                // TODO add dampening somehow, on collision? over time?
                let foo: usize = 5;
                // let nr_of_bodies = foo.pow(2);
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
                        family.add_object(&object, &mut conf);
                    }
                }
                engine_conf.obj_families.push(conf);
                obj_families.push(family);

            }, _ => {}
        }
        obj_families
    }

    pub fn setup_fields(
        sim_id: &str,
        engine_conf: &mut EngineConfig,
    ) -> Vec<Field> {

        // field-specific imports
        // use field::variant::FieldVariant;
        use field::relevant_cells::FieldRelevantCells;
        use crate::boundary::field::variant::FieldBoundaryVariant;
        use crate::integrator::field::variant::FieldIntegratorVariant;
        use crate::interaction::field::field::FieldFieldInteraction;
        // use crate::interaction::field::object::FieldObjInteraction;

        // math stuff
        const _TAU: f64 = 2. * 3.14159265358979;
        let mut rng = rand::thread_rng();

        // default values
        let GRID_SIZE = 200;
        let _DT = engine_conf.dt;  // 0.01

        let mut fields: Vec<Field> = Vec::new();
        match sim_id {

            "ising-model" => {

                let id = 0;
                let mut conf            = FieldEngineConfig::new(id);
                // conf.field_variant      = FieldVariant::Ising;
                conf.dimensions         = Vec::from([GRID_SIZE, GRID_SIZE, 1]);
                conf.integrator         = FieldIntegratorVariant::CellAuto;
                conf.relevant_cells     = FieldRelevantCells::RandomBatch;
                conf.field_interactions = Vec::from([FieldFieldInteraction::Ising]);
                conf.boundary           = FieldBoundaryVariant::Periodic;

                let mut field = Field::new(id);
                for _row_idx in 0..conf.dimensions[0] {
                    for _col_idx in 0..conf.dimensions[1] {
                        let rand: f64 = rng.gen();
                        let val = if rand > 0.5 { -1. } else { 1. };
                        field.entries.push(val);
                    }
                }
                engine_conf.fields.push(conf);
                fields.push(field);

            }, "game-of-life" => {

                let id = 0;
                let mut conf            = FieldEngineConfig::new(id);
                // conf.field_variant = FieldVariant::GameOfLife;
                conf.dimensions         = Vec::from([20, 20, 1]);
                conf.relevant_cells     = FieldRelevantCells::Entire;
                conf.field_interactions = Vec::from([FieldFieldInteraction::GameOfLife]);
                conf.boundary           = FieldBoundaryVariant::Periodic;

                let mut field = Field::new(id);

                let living_cells = vec![
                    (5, 5),
                    (5, 6),
                    (6, 6),
                    (6, 7),
                ];

                // for _row_idx in 0..conf.dimensions[0] {
                //     for _col_idx in 0..conf.dimensions[1] {
                //         let rand: f64 = rng.gen();
                //         let val = if rand > 0.9 { -1. } else { 1. };
                //         // let val = 1.;
                //         field.entries.push(val);
                //     }
                // }

                for cell_coords in living_cells.iter() {
                    let (x, y) = (cell_coords.0, cell_coords.1);
                    let cell_index = get_cell_idx_from_coords(x, y, &field, &conf);
                }

                engine_conf.fields.push(conf);
                fields.push(field);

            }, _ => {}
        }

        fields
    }

}

