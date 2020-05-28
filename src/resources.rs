use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity, World, WorldExt},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Perl,
    _ExhaustiveMatches,
}

#[derive(Debug)]
pub struct ResourceAttributes(ResourceType);

impl ResourceAttributes {
    pub fn new(t: ResourceType) -> Self {
        ResourceAttributes(t)
    }
}

pub fn create_resource(
    resource: ResourceAttributes,
    world: &mut World,
    sprite_render: SpriteRender,
    x: f32,
    y: f32,
) -> Entity {
    let mut trans = Transform::default();
    trans.set_translation_xyz(x, y, crate::layers::ResourceLayer);
    *trans.scale_mut() *= 0.07;

    let color = match resource.0 {
        ResourceType::Perl => Srgba::new(0.1, 1.0, 0.1, 0.8),
        _ => panic!("Unimplemented!"),
    };

    let tint = Tint(color);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(resource)
        .with(trans)
        .with(tint)
        .build()
}

impl Component for ResourceAttributes {
    type Storage = DenseVecStorage<Self>;
}
