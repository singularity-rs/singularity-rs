use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::UiCreator,
    utils::application_root_dir,
};

pub struct MainMenu;

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/own.ron", ());
        });
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // let StateData { world, .. } = state_data;
        let StateData { .. } = state_data;
        Trans::None
    }
}

pub fn menu() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;

    let mut game = Application::new(assets_dir, MainMenu, game_data)?;
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
