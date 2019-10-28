use amethyst::{
    ecs::{Entity, prelude::WriteStorage},
};
use crate::gunit::tasks::Task;
use crate::gunit::general_unit::{GUnitAttributes, GUnitType};
use crate::platform::platform::PlatformAttributes;
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
