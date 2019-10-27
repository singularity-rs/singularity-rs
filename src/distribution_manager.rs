use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, VecStorage, World, WorldExt, Entity, prelude::WriteStorage},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};
use derivative::Derivative;
use crate::tasks::*;
use crate::general_unit::*;
use crate::platform::*;
use rand::seq::SliceRandom;


pub struct DistributionManager {
    idle_units: Vec<Entity>,
    // ...

    platforms: Vec<Entity>,
    // producing_platforms: Vec<Entity>,
    // defending_platforms: Vec<Entity>,
    // ...
}


impl DistributionManager {

    fn request_new_task<'s>(&self, task_type: GUnitType, unit: GUnitAttributes, storage: WriteStorage<'s, PlatformAttributes>) -> Task {

        let mut task = Task::default();

        match task_type {
            GUnitType::Idle => {

                if let Some(platform) = unit.get_current_platform() {
                    let platform = storage.get(*platform).expect("Failed to get Platform Attributes");
                    let connected = platform.get_connected();
                    
                    if connected.len() > 0 {
                        task = Task { end_platform: connected.choose(&mut rand::thread_rng()).copied(), ..task };
                    }
                }
            },
        };

        task

    }
}
