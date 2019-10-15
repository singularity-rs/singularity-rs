use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::ParentHierarchy,
    ecs::{
        error::WrongGeneration,
        prelude::{Entity, World, WorldExt},
    },
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

use std::iter;

/// delete the specified root entity and all of its descendents as specified
/// by the Parent component and maintained by the ParentHierarchy resource
// from https://github.com/amethyst/evoli src/utils/hierarchy_util.rs
pub fn delete_hierarchy(root: Entity, world: &mut World) -> Result<(), WrongGeneration> {
    let entities = {
        iter::once(root)
            .chain(
                world
                    .read_resource::<ParentHierarchy>()
                    .all_children_iter(root),
            )
            .collect::<Vec<Entity>>()
    };
    world.delete_entities(&entities)
}

pub fn load_sprite_sheet(world: &mut World, file: &str) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let texture_handle = {
        loader.load(
            "textures/".to_owned() + file + ".png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/".to_owned() + file + ".ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
