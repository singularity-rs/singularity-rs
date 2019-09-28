use amethyst::{
    assets::{HotReloadBundle, Processor},
    audio::Source,
    core::{
        transform::{ParentHierarchy, TransformBundle},
        SystemDesc,
    },
    derive::SystemDesc,
    ecs::{
        error::WrongGeneration,
        prelude::{Entity, System, SystemData, World, WorldExt, Write},
    },
    input::{is_close_requested, is_key_down, is_mouse_button_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        // rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::{EventChannel, ReaderId},
    ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiEventType, UiFinder},
    utils::application_root_dir,
    winit::{MouseButton, VirtualKeyCode},
};

use log::info;
use std::iter;

// trait Screen {}
//
// struct Loading {
//     progress: ProgressCounter,
//     next_screen: dyn Screen,
// }

// type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default, Debug)]
struct WelcomeScreen {
    ui_handle: Option<Entity>,
}

// impl Screen for WelcomeScreen {}

impl SimpleState for WelcomeScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
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
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(handler) = self.ui_handle {
            delete_hierarchy(handler, data.world).expect("Failed to remove WelcomeScreen");
        }
        self.ui_handle = None;
    }
}

#[derive(Default, Debug)]
struct MainMenu {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

// impl Screen for MainMenu {}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.button_start.is_none()
            || self.button_load.is_none()
            || self.button_options.is_none()
            || self.button_credits.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find("start");
                self.button_load = ui_finder.find("load");
                self.button_options = ui_finder.find("options");
                self.button_credits = ui_finder.find("credits");
            });
        }

        Trans::None
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
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    target
                );
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("Failed to remove MainMenu");
        }
        self.ui_root = None;
        self.button_start = None;
        self.button_load = None;
        self.button_options = None;
        self.button_credits = None;
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
        // .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())? // required for plugin RenderUi
        .with_bundle(HotReloadBundle::default())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.005, 0.005, 0.005, 1.0]),
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

/// This shows how to handle UI events.
#[derive(SystemDesc)]
#[system_desc(name(UiEventHandlerSystemDesc))]
pub struct UiEventHandlerSystem {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<UiEvent>,
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        // Reader id was just initialized above if empty
        for ev in events.read(&mut self.reader_id) {
            drop(ev);
            // info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
