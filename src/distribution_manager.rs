use amethyst::{
    ecs::{Entity, prelude::ReadStorage},
};
use crate::gunit::tasks::Task;
use crate::gunit::general_unit::{GUnitAttributes, GUnitType};
use crate::platform::platform::PlatformAttributes;
use rand::seq::SliceRandom;


#[allow(dead_code)]
#[derive(Default)]
pub struct DistributionManager {
    idle_units: Vec<Entity>,
    // ...

    platforms: Vec<Entity>,
    // producing_platforms: Vec<Entity>,
    // defending_platforms: Vec<Entity>,
    // ...
}


#[allow(dead_code)]
impl DistributionManager {

    pub fn register_unit(&mut self, unit: Entity) {
        self.idle_units.push(unit);
    }

    pub fn request_new_task<'s>(&mut self, task_type: GUnitType, unit: GUnitAttributes, storage: &ReadStorage<'s, PlatformAttributes>) -> Option<Task> {

        let task = Task::default();

        match task_type {
            GUnitType::Idle => {

                if let Some(platform) = unit.get_current_platform() {
                    let platform = storage.get(*platform).expect("Failed to get Platform Attributes");
                    let connected = platform.get_connected();
                    
                    if connected.len() > 0 {
                        return Some(Task { end_platform: connected.choose(&mut rand::thread_rng()).copied(), ..task });
                    }
                }
            },
        };
        None
    }
}
