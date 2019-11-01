use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, VecStorage, World, WorldExt, Entity, ReadStorage},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};
use derivative::Derivative;
use crate::gunit::tasks::Task;
use crate::distribution_manager::DistributionManager;
use crate::platform::platform::PlatformAttributes;

pub const SLOWDOWN_DIST: f32 = 50.0;
pub const ARRIVED_DIST: f32 = 1.0;
pub const TARGET_OFFSET: f32 = 10.0;


#[derive(Default)]
pub struct Arrived;

impl Component for Arrived {
    type Storage = VecStorage<Self>;
}


#[derive(Debug, Derivative, Copy, Clone)]
#[derivative(Default)]
pub enum GUnitType {
    /// By default, any unit is Idle at first.
    #[derivative(Default)]
    Idle,

    // Carry,
    // Production,
    // Build,
    // Defense,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct GUnitAttributes{
    gtype: GUnitType,
    /// The Task the Unit is working on
    task: Option<Task>,
    /// might be idling/working on a Platform
    on: Option<Entity>,
    /// or, in transit between two Platforms
    last: Option<Entity>,
    next: Option<Entity>,
    /// this is the actual location of the next target
    target_location: Option<Vector3<f32>>,
    /// Location where the unit started. Relevant for calculating the right side of the road.
    starting_location: Option<Vector3<f32>>,
    /// Overarching, the goal could be a platform
    goal: Option<Entity>,
    /// Units tend to have a maximum Velocity
    velocity: f32,
}

impl GUnitAttributes {

    pub fn new() -> Self {
        GUnitAttributes { velocity: 3.0, ..Default::default() }
    }

    #[inline]
    pub fn get_target_location(&self) -> &Option<Vector3<f32>> {
        &self.target_location
    }

    #[inline]
    pub fn get_starting_location(&self) -> &Option<Vector3<f32>> {
        &self.starting_location
    }

    #[inline]
    pub fn get_velocity(&self) -> &f32 {
        &self.velocity
    }

    #[inline]
    pub fn get_current_platform(&self) -> &Option<Entity> {
        &self.on
    }

    pub fn set_platform(
        &mut self,
        platform: Entity,
    ) {
        self.target_location = None;
        self.on = Some(platform);
    }

    pub fn set_goal(
        &mut self,
        platform: Entity,
    ) {
        self.goal = Some(platform);
    }

    pub fn set_target_platform(
        &mut self,
        platform: Entity,
        target_location: Vector3<f32>,
    ) {
        self.next = Some(platform);
        self.last = self.on;
        self.on = None;
        self.target_location = Some(target_location);
    }

    pub fn update_starting_platform<'s>(
        &mut self,
        transform: &ReadStorage<'s, Transform>,
    ) {
        if let Some(current) = self.on {
            let start_trans = transform.get(current).expect("25");
            self.starting_location = Some(*start_trans.translation());
        }
    }

    pub fn arrive(
        &mut self
    ) {
        self.on = self.next;
        self.next = None;
        self.target_location = None;
    }

    #[allow(dead_code)]
    pub fn get_task(
        &self
    ) -> &Option<Task> {
        &self.task
    }

    pub fn update<'s>(
        &mut self,
        distr_mgr: &mut DistributionManager,
        platform_attrs: &ReadStorage<'s, PlatformAttributes>,
        transform: &ReadStorage<'s, Transform>,
    ) {
        if let Some(task) = &self.task {
            match task.job_type {
                GUnitType::Idle => {
                    if let Some(_) = self.on {
                        if self.on == task.end_platform {
                            self.task = None;
                        }
                    }
                }
            }
        } else {
            self.task = distr_mgr.request_new_task(self.gtype, *self, &platform_attrs);
            if let Some(task) = self.task {
                if let Some(platform) = task.end_platform {
                    self.update_starting_platform(transform);
                    let plat_trans = transform.get(platform).expect("23");
                    self.set_target_platform(platform, *plat_trans.translation());
                }
            }
        }
    }
}


impl Component for GUnitAttributes {
    type Storage = DenseVecStorage<Self>;
}


pub fn create_gunit(
    gunit: GUnitAttributes,
    world: &mut World,
    sprite_render: SpriteRender,
    x: f32,
    y: f32,
) -> Entity {

    let mut trans = Transform::default();
    trans.set_translation_xyz(x, y, crate::layers::GeneralUnitLayer);
    *trans.scale_mut() *= 0.1;

    let color = match gunit.gtype {
        GUnitType::Idle => Srgba::new(0.0, 0.0, 0.0, 1.0),
    };

    let tint = Tint(color);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(gunit)
        .with(trans)
        .with(tint)
        .build()
}


// --------------------- Only for testing? ---------------------

#[allow(dead_code)]
pub fn unit_set_goal(
    world: &mut World,
    unit: Entity,
    platform: Entity
    ) {

    let mut gunit_attr_storage = world.write_storage::<GUnitAttributes>();

    let unit_attrs = gunit_attr_storage.get_mut(unit).expect("Failed to get Unit Attributes from supposed Unit Entity");

    unit_attrs.set_goal(platform);
}

#[allow(dead_code)]
pub fn unit_set_platform(
    world: &mut World,
    unit: Entity,
    platform: Entity
    ) {

    let mut gunit_attr_storage = world.write_storage::<GUnitAttributes>();

    let unit_attrs = gunit_attr_storage.get_mut(unit).expect("Failed to get Unit Attributes from supposed Unit Entity");

    unit_attrs.set_platform(platform);
}

#[allow(dead_code)]
pub fn unit_set_target_platform(
    world: &mut World,
    unit: Entity,
    platform: Entity,
    ) {

    let mut gunit_attr_storage = world.write_storage::<GUnitAttributes>();

    let unit = gunit_attr_storage.get_mut(unit).expect("Failed to get Unit Attributes from supposed Unit Entity");

    let trans_store = world.read_storage::<Transform>();

    let plat_trans = trans_store.get(platform).expect("Failed to get Transform from supposed Platform Entity");

    unit.set_target_platform(platform, *plat_trans.translation());
}


