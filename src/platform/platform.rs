use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, World, WorldExt, Entity},
    prelude::*,
    core::math::Point,
    renderer::{palette::Srgba, resources::Tint, SpriteRender, debug_drawing::DebugLinesComponent},
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


impl PlatformAttributes {
    pub fn get_connected(&self) -> &Vec<Entity> {
        &self.connected
    }
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

    let mut platform_attr_storage = world.write_storage::<PlatformAttributes>();
    let loc_storage = world.read_storage::<Transform>();

    let p1_attrs = platform_attr_storage.get_mut(p1).expect("Failed to get Platform Attributes from supposed Platform Entity");
    let mut p1_loc = loc_storage.get(p1).expect("24").clone();
    p1_loc.set_translation_z(crate::layers::RoadLayer);

    p1_attrs.connected.push(p2);


    let p2_attrs = platform_attr_storage.get_mut(p2).expect("Failed to get Platform Attributes from supposed Platform Entity");
    let mut p2_loc = loc_storage.get(p2).expect("25").clone();
    p2_loc.set_translation_z(crate::layers::RoadLayer);

    p2_attrs.connected.push(p1);

    let mut lines = DebugLinesComponent::new();

    lines
        .add_line(
            Point::from(*p1_loc.translation()),
            Point::from(*p2_loc.translation()),
            Srgba::new(0.0, 0.0, 0.0, 1.0)
        );

    drop(platform_attr_storage);
    drop(loc_storage);

    world
        .create_entity()
        .with(lines)
        .build();
}


impl Component for PlatformAttributes {
    type Storage = DenseVecStorage<Self>;
}

