use amethyst::{
    assets::{AssetStorage, Handle, HotReloadBundle, Loader, Processor},
    audio::Source,
    core::transform::{Transform, TransformBundle, ParentHierarchy},
    ecs::prelude::{Entity, System, SystemData, World, Component, DenseVecStorage},
    ecs::error::WrongGeneration,
    input::{is_close_requested, is_key_down, is_mouse_button_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        types::DefaultBackend, RenderingBundle,
    },
    ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiFinder, UiText},
    utils::application_root_dir,
    winit::{VirtualKeyCode, MouseButton},
};

use std::iter;
use log::info;


// trait Screen {}
//
// struct Loading {
//     progress: ProgressCounter,
//     next_screen: dyn Screen,
// }





#[derive(Default, Debug)]
struct WelcomeScreen {
    ui_handle: Option<Entity>,
}

// impl Screen for WelcomeScreen {}

impl SimpleState for WelcomeScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle = Some(world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/welcome.ron", ())
        }));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            // WindowEvent { window_id: WindowId(X(WindowId(25165826))), event: KeyboardInput { device_id: DeviceId(X(DeviceId(3))), input: KeyboardInput { scancode: 1, state: Pressed, virtual_keycode: Some(Escape), modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false } } } }
            // ReceivedCharacter: '\u{1b}' ...
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else if is_mouse_button_down(&event, MouseButton::Left) {
                    info!("Switching to MainMenu!");
                    Trans::Switch(Box::new(MainMenu::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
                Trans::None
            }
            StateEvent::Input(input) => {
                info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }


    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(handler) = self.ui_handle {
            delete_hierarchy(handler, data.world)
                .expect("Failed to remove WelcomeScreen");
        }
        self.ui_handle = None;
    }
}


#[derive(Default, Debug)]
struct MainMenu {
    ui_handle: Option<Entity>,
}

// impl Screen for MainMenu {}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        info!("Should load MainMenu?");

        self.ui_handle = Some(world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/menu.ron", ())
        }));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    info!("Switching back!");
                    Trans::Switch(Box::new(WelcomeScreen::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
                Trans::None
            }
            StateEvent::Input(input) => {
                info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(handler) = self.ui_handle {
            delete_hierarchy(handler, data.world)
                .expect("Failed to remove MainMenu");
        }
        self.ui_handle = None;
    }
}


/// delete the specified root entity and all of its descendents as specified
/// by the Parent component and maintained by the ParentHierarchy resource
// from https://github.com/amethyst/evoli src/utils/hierarchy_util.rs
fn delete_hierarchy(root: Entity, world: &mut World) -> Result<(), WrongGeneration> {
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


pub fn menu() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(HotReloadBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())? // required for plugin RenderUi
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderUi::default()),
            // RenderUi failed with
            // thread 'main' panicked at 'Tried to fetch a resource of type
            // "specs::storage::MaskedStorage<amethyst_ui::image::UiImage>",
            // but the resource does not exist. -> UiBundle<StringBinding> needed
        )?;

    let mut game = Application::new(assets_dir, WelcomeScreen::default(), game_data)?;
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
