use amethyst::{
    assets::{HotReloadBundle, Processor},
    audio::Source,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

extern crate rand;

mod credits;
mod events;
mod game;
mod layers;
mod menu;
mod pause;
mod platform;
mod resources;
mod util;
mod welcome;
mod roads;
mod general_unit;
mod gunit_movement;
mod distribution_manager;
mod tasks;
mod platform_actions;


/// Quick overview what you can do when running this example:
///
/// Switch from the 'Welcome' Screen to the 'Menu' Screen.
/// From the 'Menu', switch to either 'Credits' (from which you can only switch back) or to 'Game'.
/// From 'Game', you can enter the 'Pause' menu.
/// Here you can select to resume (go back to 'Game'), exit to menu (go to 'Menu') or exit (quit).
///
/// During the 'Pause' menu, the 'Game' is paused accordingly.

pub fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // this will be the directory the 'Cargo.toml' is defined in.
    let app_root = application_root_dir()?;

    // our display config is in our configs folder.
    let display_config_path = app_root.join("config/display.ron");

    // other assets ('*.ron' files, '*.png' textures, '*.ogg' audio files, ui prefab files, ...) are here
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        // a lot of other bundles/systems depend on this (without it being explicitly clear), so it
        // makes sense to add it early on
        .with_bundle(TransformBundle::new())?
        // this bundle allows us to 'find' the Buttons and other UI elements later on
        .with_bundle(UiBundle::<StringBindings>::new())?
        // this allows us to reload '*.ron' files during execution
        .with_bundle(HotReloadBundle::default())?
        // without this System, our Program will silently (!) fail when trying to load the 'Game'.
        // (try it!)
        // This System takes care of Audio (in this case, the Button audio for hovering/clicking/...
        .with(Processor::<Source>::new(), "source_processor", &[])
        // With this System, we can register UI events and act accordingly.
        // In this example it simply prints the events, excluding it does not provide less
        // functionality.
        .with_system_desc(
            crate::events::UiEventHandlerSystemDesc::default(),
            "ui_event_handler",
            &[],
        )
        // This system is in 'events.rs'. Basically, it registers UI events that
        // happen. Without it, the buttons will not react.
        .with_bundle(InputBundle::<StringBindings>::new())?
        // Without this, we would not get a picture.
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // This creates the window and draws a background, if we don't specify a
                // background in the loaded ui prefab file.
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        // .with_clear([0.005, 0.005, 0.005, 1.0]), // Dark Grey
                        .with_clear([0.05, 0.05, 0.05, 1.0]),
                )
                // Without this, all of our beautiful UI would not get drawn.
                // It will work, but we won't see a thing.
                .with_plugin(RenderUi::default())
                // Required for rendering SpriteRender instances
                .with_plugin(RenderFlat2D::default()),
        )?;

    // creating the Application with the assets_dir, the first Screen, and the game_data with it's
    // systems.
    let mut game = Application::new(
        assets_dir,
        crate::game::Game::default(),
        // crate::pong::Pong,
        game_data,
    )?;
    game.run();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
