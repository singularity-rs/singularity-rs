use crate::distribution_manager::*;
use crate::gunit::general_unit::*;
use crate::platform::platform::PlatformAttributes;
use amethyst::{
    core::Transform,
    ecs::prelude::{ReadStorage, System, Write, WriteStorage},
    ecs::Join,
};

#[derive(Default)]
pub struct GUnitStateSystem;

impl<'s> System<'s> for GUnitStateSystem {
    type SystemData = (
        WriteStorage<'s, GUnitAttributes>,
        Write<'s, DistributionManager>,
        ReadStorage<'s, PlatformAttributes>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut gunits, mut distr_mgr, platforms, transforms): Self::SystemData) {
        for gunit in (&mut gunits).join() {
            gunit.update(&mut distr_mgr, &platforms, &transforms);
        }
    }
}
