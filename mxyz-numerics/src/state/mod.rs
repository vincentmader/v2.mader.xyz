
use rand::Rng;

pub mod field;
pub mod object;

use field::Field;
use object::ObjectFamily;
use object::ObjectVariant;
use object::ObjectAttribute;

use crate::integrator::setup::IntegratorSetup;
use crate::integrator::field::FieldIntegrator;
use crate::integrator::object::ObjectIntegrator;
use crate::integrator::object::IntegratorVariant as ObjectIntegratorVariant;
use crate::integrator::field::IntegratorVariant as FieldIntegratorVariant;

use crate::interaction::object as object_interactions;
use crate::interaction::field as field_interactions;
use crate::boundary::object::ObjectBoundary;
use crate::boundary::object::variant::ObjectBoundaryVariant;


#[derive(Clone)]
pub struct State {
    pub object_families: Vec<ObjectFamily>,
    pub fields: Vec<Field>,
}
impl State {

    pub fn new(page_id: &str, integrators: &mut IntegratorSetup) -> Self {

        // let object_families = Vec::new();
        // let fields = Vec::new();

        let object_families = Self::setup_objects(page_id, integrators);
        let fields          = Self::setup_fields(page_id, integrators);

        let state = State {
            object_families, fields,
        };
        state
    }

    pub fn setup_objects(
        page_id: &str,
        integrator_setup: &mut IntegratorSetup,
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

        match page_id {
            "2body-kepler" => {

                // object family
                let object_variant = ObjectVariant::Body;
                let mut family = ObjectFamily::new(
                    0, 
                    object_variant,
                    Vec::from(OBJECT_ATTRIBUTES),
                    Vec::new(),
                );
                // interactions
                let object_field_interactions = Vec::from([]);
                let object_object_interactions = Vec::from([
                    object_interactions::object::Interaction::ForceNewtonianGravity,
                ]);
                // integrator
                let integrator = ObjectIntegrator::new(
                    OBJECT_INTEGRATOR_VARIANT,
                    DT,
                    object_field_interactions, 
                    object_object_interactions,
                ); 
                // boundaries
                let integrators = Vec::from([integrator]);
                let boundaries = Vec::from([
                    ObjectBoundary::new(
                        // ObjectBoundaryVariant::Periodic
                        // ObjectBoundaryVariant::WallCollisionElastic
                        ObjectBoundaryVariant::WallCollisionInelastic
                    ),
                ]);

                let r = 0.7;
                let speed = 2.;

                let nr_of_objects = 16;
                for obj_idx in 0..nr_of_objects {
                    let phi = obj_idx as f64 / nr_of_objects as f64 * TAU;
                    // let rand: f64 = rng.gen();
                    // let phi = TAU * rand;
                    let x = r * phi.cos();
                    let y = r * phi.sin();
                    let u = -speed * phi.sin();
                    let v = speed * phi.cos();

                    let object = Vec::from([0.1, x, y, u, v]);
                    family.add_object(&object);
                }
        
                integrator_setup.object.push(integrators);
                integrator_setup.object_boundaries.push(boundaries);
                object_families.push(family);

            },
            _ => {}
        }

        object_families

    }

    pub fn setup_fields(
        page_id: &str,
        integrator_setup: &mut IntegratorSetup,
    ) -> Vec<Field> {

        let fields: Vec<Field> = Vec::new();
            // setup integrators.field

        Vec::new()

    }
    
}

