use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, World, WorldExt, Entity},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};
use derivative::Derivative;

#[derive(Debug, Derivative, Copy, Clone)]
#[derivative(Default)]
pub enum PlatformType {
    #[derivative(Default)]
    Blank,
    // // As soon as there is 'Production' capability:
    // Well,
    // Mine,
    // Quarry,
    //
    // // As soon as there is PlatformActions:
    // Factory,
    // Junkyard,
    // Energy,
    //
    // // When creation of FreeMovingUnits is possible:
    // Barracks,
    // Command,
    //
    // // When there is fighting:
    // Kinetic,
    // Laser,
    //
    // // Other, more complex:
    // Packaging,
    // Storage,
    _ExhaustiveMatches,
}

#[derive(Debug, Default)]
pub struct PlatformAttributes{
    platform_type: PlatformType,
    connected: Vec<Entity>,
}

pub fn create_platform(
    platform: PlatformAttributes,
    world: &mut World,
    sprite_render: SpriteRender,
    x: f32,
    y: f32,
) -> Entity {

    let mut trans = Transform::default();
    trans.set_translation_xyz(x, y, crate::layers::BasePlatformLayer);
    *trans.scale_mut() *= 0.25;

    let color = match platform.platform_type {
        PlatformType::Blank => Srgba::new(1.0, 1.0, 1.0, 0.8),
        _ => panic!("Unimplemented!"),
    };

    let tint = Tint(color);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(platform)
        .with(trans)
        .with(tint)
        .build()
}

pub fn connect_platforms(
    world: &mut World,
    p1: Entity,
    p2: Entity,
    ) {

    // let my_entity = world.create_entity().with(MyComponent).build();
    // let mut storage = world.write_storage::<MyComponent>();
    // let mut my = storage.get_mut(my_entity).expect("Failed to get component for entity");

    let mut platform_attr_storage = world.write_storage::<PlatformAttributes>();

    let p1_attrs = platform_attr_storage.get_mut(p1).expect("Failed to get Platform Attributes from supposed Platform Entity");

    p1_attrs.connected.push(p2);


    let p2_attrs = platform_attr_storage.get_mut(p2).expect("Failed to get Platform Attributes from supposed Platform Entity");

    p2_attrs.connected.push(p1);

}


impl Component for PlatformAttributes {
    type Storage = DenseVecStorage<Self>;
}

