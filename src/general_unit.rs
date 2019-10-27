use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, World, WorldExt, Entity},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};
use derivative::Derivative;


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

    unit_attrs.on = Some(platform);
}

pub fn unit_set_target_platform(
    world: &mut World,
    unit: Entity,
    platform: Entity,
    ) {

    let mut gunit_attr_storage = world.write_storage::<GUnitAttributes>();

    let unit_attrs = gunit_attr_storage.get_mut(unit).expect("Failed to get Unit Attributes from supposed Unit Entity");

    let trans_store = world.read_storage::<Transform>();

    let plat_trans = trans_store.get(platform).expect("Failed to get Transform from supposed Platform Entity");

    unit_attrs.target_location = Some(*plat_trans.translation());
}


