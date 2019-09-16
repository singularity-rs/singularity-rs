use amethyst::{
    assets::{PrefabLoaderSystemDesc, Processor},
    audio::Source,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle, SystemDesc, Time},
    derive::SystemDesc,
    ecs::prelude::{Entity, System, SystemData, World, WorldExt, Write},
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::{EventChannel, ReaderId},
    ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiFinder, UiText},
    utils::{
        application_root_dir,
        fps_counter::{FpsCounter, FpsCounterBundle},
        scene::BasicScenePrefab,
    },
    winit::VirtualKeyCode,
};
use log::info;

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
struct Example {
    fps_display: Option<Entity>,
    random_text: Option<Entity>,
}

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        // Initialise the scene with an object, a light and a camera.
        // let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/sphere.ron", RonFormat, ())
        // });
        // world.create_entity().with(handle).build();
        // init_output(&mut world);
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/example.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
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

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }
        if self.random_text.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("random_text") {
                    self.random_text = Some(entity);
                }
            });
        }

        let mut ui_text = world.write_storage::<UiText>();
        {
            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }

        {
            if let Some(random_text) = self.random_text.and_then(|entity| ui_text.get_mut(entity)) {
                if let Ok(value) = random_text.text.parse::<i32>() {
                    let mut new_value = value * 10;
                    if new_value > 100_000 {
                        new_value = 1;
                    }
                    random_text.text = new_value.to_string();
                } else {
                    random_text.text = String::from("1");
                }
            }
        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, Example::default())?
        // Unlimited FPS
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;
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
            info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        }
    }
}

// use amethyst::{
//     core::transform::TransformBundle,
//     // ecs::prelude::{ReadExpect, Resources, SystemData},
//     prelude::*,
//     renderer::{
//         plugins::{RenderFlat2D, RenderToWindow},
//         types::DefaultBackend,
//         RenderingBundle,
//     },
//     utils::application_root_dir,
// };
//
// struct MyState;
//
// impl SimpleState for MyState {
//     fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
// }
//
// fn main() -> amethyst::Result<()> {
//     amethyst::start_logger(Default::default());
//
//     let app_root = application_root_dir()?;
//
//     let config_dir = app_root.join("config");
//     let display_config_path = config_dir.join("display.ron");
//
//     let game_data = GameDataBuilder::default()
//         .with_bundle(
//             RenderingBundle::<DefaultBackend>::new()
//                 .with_plugin(
//                     RenderToWindow::from_config_path(display_config_path)
//                         .with_clear([0.34, 0.36, 0.52, 1.0]),
//                 )
//                 .with_plugin(RenderFlat2D::default()),
//         )?
//         .with_bundle(TransformBundle::new())?;
//
//     let mut game = Application::new("/", MyState, game_data)?;
//     game.run();
//
//     Ok(())
// }

// use amethyst::prelude::*;
// use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
//
// struct GameState;
//
// impl SimpleState for GameState {
//     fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) {
//         println!("Starting game!");
//     }
//
//     fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
//         if let StateEvent::Window(event) = &event {
//             match event {
//                  Event::WindowEvent { event, .. } => match event {
//                     WindowEvent::KeyboardInput {
//                         input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
//                     } |
//                     WindowEvent::CloseRequested => Trans::Quit,
//                     _ => Trans::None,
//                 },
//                 _ => Trans::None,
//             }
//         } else {
//             Trans::None
//         }
//     }
//
//     fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
//         println!("Computing some more whoop-ass...");
//         // Trans::Quit
//         Trans::None
//     }
// }
//
// fn main() {
//     amethyst::start_logger(Default::default());
//     let mut game = Application::new("assets/", GameState, GameDataBuilder::default())
//         .expect("Fatal error");
//     game.run();
// }
