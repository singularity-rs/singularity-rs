use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};
use crate::general_unit::*;

#[derive(Default)]
pub struct GUnitMovementSystem;

impl<'s> System<'s> for GUnitMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, GUnitAttributes>,
    );

    fn run(&mut self, (mut transforms, mut gunits): Self::SystemData) {
        for (gunit, trans) in (&mut gunits, &mut transforms).join() {

            if let Some(target) = gunit.target_location {
                let diff = target - trans.translation();

                let mut vel = gunit.velocity;

                if diff.norm() < SLOWDOWN_DIST {
                    vel = diff.norm() * vel / SLOWDOWN_DIST;
                }

                *trans.translation_mut() += diff.normalize() * vel;
                trans.set_translation_z(crate::layers::GeneralUnitLayer);

                if diff.norm() < ARRIVED_DIST {
                    gunit.arrive();
                }
            }
        }
    }
}

