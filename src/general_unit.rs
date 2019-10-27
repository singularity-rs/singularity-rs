use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, VecStorage, World, WorldExt, Entity},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};
use derivative::Derivative;


pub const SLOWDOWN_DIST: f32 = 50.0;
pub const ARRIVED_DIST: f32 = 1.0;


#[derive(Default)]
pub struct Arrived;

impl Component for Arrived {
    type Storage = VecStorage<Self>;
}


#[derive(Debug, Derivative)]
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

#[derive(Debug, Default)]
pub struct GUnitAttributes{
    gtype: GUnitType,
    /// might be idling/working on a Platform
    on: Option<Entity>,
    /// or, in transit between two Platforms
    last: Option<Entity>,
    next: Option<Entity>,
    pub velocity: f32,
    pub target_location: Option<Vector3<f32>>,
}

impl GUnitAttributes {

    pub fn new() -> Self {
        GUnitAttributes { velocity: 3.0, ..Default::default() }
    }

    pub fn set_platform(
        &mut self,
        platform: Entity,
    ) {
        self.target_location = None;
        self.on = Some(platform);
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

    pub fn arrive(
        &mut self
    ) {
        self.on = self.next;
        self.next = None;
        self.target_location = None;
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

pub fn unit_set_platform(
    world: &mut World,
    unit: Entity,
    platform: Entity
    ) {

    let mut gunit_attr_storage = world.write_storage::<GUnitAttributes>();

    let unit_attrs = gunit_attr_storage.get_mut(unit).expect("Failed to get Unit Attributes from supposed Unit Entity");

    unit_attrs.set_platform(platform);
}

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


