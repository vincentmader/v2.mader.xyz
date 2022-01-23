
use rand::Rng;

pub mod field;
pub mod object;

use field::Field;
use object::ObjectFamily;
use object::ObjectVariant;
use object::ObjectAttribute;

use crate::integrator::setup::EngineSetup;
use crate::integrator::field::FieldIntegrator;
use crate::integrator::object::ObjectIntegrator;
use crate::integrator::object::IntegratorVariant as ObjectIntegratorVariant;
use crate::integrator::field::IntegratorVariant as FieldIntegratorVariant;

use crate::interaction::object as object_interactions;
use crate::interaction::field as field_interactions;
use crate::boundary::object::ObjectBoundary;
use crate::boundary::object::variant::BoundaryVariant as ObjectBoundaryVariant;


#[derive(Clone)]
pub struct State {
    pub object_families: Vec<ObjectFamily>,
    pub fields: Vec<Field>,
}
impl State {

    pub fn new(sim_id: &str, integrators: &mut EngineSetup) -> Self {

        // let object_families = Vec::new();
        // let fields = Vec::new();

        let object_families = Self::setup_objects(sim_id, integrators);
        let fields          = Self::setup_fields(sim_id, integrators);

        let state = State {
            object_families, fields,
        };
        state
    }

    pub fn setup_objects(
        sim_id: &str,
        engine_setup: &mut EngineSetup,
    ) -> Vec<ObjectFamily> {

        const TAU: f64 = 2. * 3.14159265358979;

        const OBJECT_INTEGRATOR_VARIANT: ObjectIntegratorVariant = ObjectIntegratorVariant::EulerExplicit;
        const OBJECT_ATTRIBUTES: [ObjectAttribute; 5] = [
            ObjectAttribute::Mass,
            ObjectAttribute::PositionX,
            ObjectAttribute::PositionY,
            ObjectAttribute::VelocityX,
            ObjectAttribute::VelocityY,
        ];
        // const ACC_OBJECT_ATTRIBUTES: [ObjectAttribute; 7] = [
        //     ObjectAttribute::Mass,
        //     ObjectAttribute::PositionX,
        //     ObjectAttribute::PositionY,
        //     ObjectAttribute::VelocityX,
        //     ObjectAttribute::VelocityY,
        //     ObjectAttribute::AccelerationX,
        //     ObjectAttribute::AccelerationY,
        // ];
        // const CHARGED_OBJECT_ATTRIBUTES: [ObjectAttribute; 6] = [
        //     ObjectAttribute::Mass,
        //     ObjectAttribute::Charge,
        //     ObjectAttribute::PositionX,
        //     ObjectAttribute::PositionY,
        //     ObjectAttribute::VelocityX,
        //     ObjectAttribute::VelocityY,
        // ];
        const M: f64 = 1.;
        const DT: f64 = 0.01;
        let mut rng = rand::thread_rng();

        let mut object_families: Vec<ObjectFamily> = Vec::new();

        match sim_id {
            "2body-kepler" => {

                // OBJECT FAMILIES
                // -------------------------------------------------------------------------------

                let object_variant = ObjectVariant::Body;
                let mut family = ObjectFamily::new(0, object_variant, Vec::from(OBJECT_ATTRIBUTES));
                // integrator
                let integrator = ObjectIntegrator::new(
                    OBJECT_INTEGRATOR_VARIANT, DT, 
                    Vec::from([
                        // object-field interactions
                    ]),
                    Vec::from([
                        // object-object interactions
                        object_interactions::object::Interaction::ForceNewtonianGravity,
                    ])
                ); 
                engine_setup.object.push(integrator);
                // boundaries
                let boundary = ObjectBoundary::new(
                    // ObjectBoundaryVariant::Periodic
                    // ObjectBoundaryVariant::WallCollisionElastic
                    ObjectBoundaryVariant::WallCollisionInelastic
                    // ObjectBoundaryVariant::None
                );
                engine_setup.object_boundaries.push(boundary);

                // objects
                let (r, speed) = (0.7, 1.0);
                let nr_of_objects = 16;
                for obj_idx in 0..nr_of_objects {
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    // let rand: f64 = rng.gen();
                    // let phi = TAU * rand;
                    let x = r * phi.cos();
                    let y = r * phi.sin();
                    let u = -speed * phi.sin();
                    let v = speed * phi.cos();

                    let object = Vec::from([0.05, x, y, u, v]);
                    family.add_object(&object);
                }

                object_families.push(family);

            }, "3body-fig8" => {

                let object_variant = ObjectVariant::Body;
                let mut family = ObjectFamily::new(0, object_variant, Vec::from(OBJECT_ATTRIBUTES));
                // integrator
                let integrator = ObjectIntegrator::new(
                    OBJECT_INTEGRATOR_VARIANT, DT, 
                    Vec::from([
                        // object-field interactions
                    ]),
                    Vec::from([
                        // object-object interactions
                        object_interactions::object::Interaction::ForceNewtonianGravity,
                    ])
                ); 
                engine_setup.object.push(integrator);
                // boundaries
                let boundary = ObjectBoundary::new(ObjectBoundaryVariant::None);
                engine_setup.object_boundaries.push(boundary);

                // objects
                let xs = [0.7775727187509279270, 0., -0.7775727187509279270];
                let ys = [0.6287930240184685937, 0., -0.6287930240184685937];
                let us = [-0.06507160612095737318, 0.1301432122419148851, -0.06507160612095737318];
                let vs = [0.6324957346748190101, -1.264991469349638020, 0.6324957346748190101]; 
                let m = 1.;

                for body_idx in 0..3 {
                    let (x0, y0) = (xs[body_idx], ys[body_idx]);
                    let (u0, v0) = (us[body_idx], vs[body_idx]);
                    let object = Vec::from([m, x0, y0, u0, v0]);
                    family.add_object(&object);
                }

                object_families.push(family);

            }, "nbody-flowers" => {

                let object_variant = ObjectVariant::Particle;
                let mut family = ObjectFamily::new(0, object_variant, Vec::from(OBJECT_ATTRIBUTES));
                // integrator
                let integrator = ObjectIntegrator::new(
                    OBJECT_INTEGRATOR_VARIANT, DT, 
                    Vec::from([
                        // object-field interactions
                    ]),
                    Vec::from([
                        // object-object interactions
                        object_interactions::object::Interaction::ForceNewtonianGravity,
                    ])
                ); 
                engine_setup.object.push(integrator);
                // boundaries
                let boundary = ObjectBoundary::new(
                    // ObjectBoundaryVariant::Periodic
                    // ObjectBoundaryVariant::WallCollisionElastic
                    // ObjectBoundaryVariant::WallCollisionInelastic
                    ObjectBoundaryVariant::None
                );
                engine_setup.object_boundaries.push(boundary);

                // objects
                let (r, speed) = (0.7, 0.7);
                let nr_of_objects = 64;
                for obj_idx in 0..nr_of_objects {
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    // let rand: f64 = rng.gen();
                    // let phi = TAU * rand;
                    let x = r * phi.cos();
                    let y = r * phi.sin();
                    let u = -speed * phi.sin();
                    let v = speed * phi.cos();

                    let object = Vec::from([0.01, x, y, u, v]);
                    family.add_object(&object);
                }

                object_families.push(family);


                let object_variant = ObjectVariant::Static;
                let mut family = ObjectFamily::new(1, object_variant, Vec::from(OBJECT_ATTRIBUTES));
                // integrator
                let integrator = ObjectIntegrator::new(
                    OBJECT_INTEGRATOR_VARIANT, DT, 
                    Vec::from([
                        // object-field interactions
                    ]),
                    Vec::from([
                        // object-object interactions
                        object_interactions::object::Interaction::ForceNewtonianGravity,
                    ])
                ); 
                engine_setup.object.push(integrator);
                // boundaries
                let boundary = ObjectBoundary::new(
                    // ObjectBoundaryVariant::Periodic
                    // ObjectBoundaryVariant::WallCollisionElastic
                    // ObjectBoundaryVariant::WallCollisionInelastic
                    ObjectBoundaryVariant::None
                );
                engine_setup.object_boundaries.push(boundary);

                let object = Vec::from([1., 0., 0., 0., 0.]);
                family.add_object(&object);
                object_families.push(family);

            },
            _ => {}
        }

        object_families

    }

    pub fn setup_fields(
        sim_id: &str,
        engine_setup: &mut EngineSetup,
    ) -> Vec<Field> {

        let fields: Vec<Field> = Vec::new();
            // setup integrators.field

        Vec::new()

    }
    
}

