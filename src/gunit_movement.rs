use amethyst::{
    core::{transform::Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};
use nalgebra::base::Vector3;
use crate::general_unit::*;

#[derive(SystemDesc)]
#[system_desc(name(GUnitMovementSystemDesc))]
pub struct GUnitMovementSystem;

impl<'s> System<'s> for GUnitMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, GUnitAttributes>,
    );

    fn run(&mut self, (mut transforms, gunits): Self::SystemData) {
        for (_gunit, trans) in (&gunits, &mut transforms).join() {

            *trans.translation_mut() += Vector3::new(-1.0, -1.0, 0.0);

        }
    }
}

