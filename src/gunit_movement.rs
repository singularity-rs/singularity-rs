use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    core::math::Vector3,
};
use crate::general_unit::*;
use crate::platform::*;

#[derive(Default)]
pub struct GUnitMovementSystem;

impl<'s> System<'s> for GUnitMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, GUnitAttributes>,
    );

    fn run(&mut self, (mut transforms, gunits): Self::SystemData) {
        for (gunit, trans) in (&gunits, &mut transforms).join() {

            if let Some(target) = gunit.target_location {
                let diff = target - trans.translation();

                let mut vel = gunit.velocity;

                if diff.norm() < 30.0 {
                    vel = diff.norm() * vel / 30.0;
                }

                *trans.translation_mut() += diff.normalize() * vel;

            }
        }
    }
}

