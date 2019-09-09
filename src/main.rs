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


use amethyst::prelude::*;
use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("Starting game!");
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                 Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                    } |
                    WindowEvent::CloseRequested => Trans::Quit,
                    _ => Trans::None,
                },
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("Computing some more whoop-ass...");
        Trans::Quit
    }
}

fn main() {
    let mut game = Application::new("assets/", GameState, GameDataBuilder::default())
        .expect("Fatal error");
    game.run();
}

