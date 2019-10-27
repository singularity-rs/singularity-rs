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
pub struct GUnitAttributes(GUnitType);

impl Component for GUnitAttributes {
    type Storage = DenseVecStorage<Self>;
}


pub fn create_gunit(
    gunit: GUnitAttributes,
    world: &mut World,
    sprite_render: SpriteRender,
    x: f32,
    y: f32,
) {

    let mut trans = Transform::default();
    trans.set_translation_xyz(x, y, crate::layers::GeneralUnitLayer);
    *trans.scale_mut() *= 0.1;

    let color = match gunit.0 {
        GUnitType::Idle => Srgba::new(0.0, 0.0, 0.0, 1.0),
    };

    let tint = Tint(color);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(gunit)
        .with(trans)
        .with(tint)
        .build();
}







