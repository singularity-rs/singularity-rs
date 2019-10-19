use crate::pause::PauseMenuState;
use crate::platform::*;
use crate::resources::*;
use crate::general_unit::*;
use crate::util::delete_hierarchy;
use crate::util::load_sprite_sheet;
use amethyst::{
    audio::output::init_output,
    core::{transform::Transform, Time},
    ecs::prelude::{Entity, WorldExt},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        palette::Srgba, resources::Tint, Camera, SpriteRender, SpriteSheet, Texture, Transparent,
    },
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    winit::VirtualKeyCode,
};
use log::info;

pub const ARENA_HEIGHT: f32 = 900.0;
pub const ARENA_WIDTH: f32 = 1600.0;

fn initialise_camera(world: &mut World) {
    // this gets the current screen dimensions:
    // let dim = world.read_resource::<ScreenDimensions>();

    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 10.0);
    // transform.set_translation_xyz(0., 0., 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_platforms(world: &mut World, sprite_render: SpriteRender) {
    // let sprite_sheet = load_sprite_sheet(world, "pong_spritesheet");

    world.register::<PlatformAttributes>();

    create_platform(
        PlatformAttributes::default(),
        world,
        sprite_render.clone(),
        200.,
        300.,
    );
    create_platform(
        PlatformAttributes::default(),
        world,
        sprite_render.clone(),
        800.,
        600.,
    );
}

fn initialize_resources(world: &mut World, sprite_render: SpriteRender) {

    world.register::<ResourceAttributes>();

    create_resource(
        ResourceAttributes::new(ResourceType::Perl),
        world,
        sprite_render.clone(),
        200.,
        300.,
    );
}

fn initialize_gunits(world: &mut World, sprite_render: SpriteRender) {

    world.register::<GUnitAttributes>();

    create_gunit(
        GUnitAttributes::default(),
        world,
        sprite_render.clone(),
        810.,
        605.,
    );
}


#[derive(Default)]
pub struct Game {
    paused: bool,
    ui_root: Option<Entity>,
    fps_display: Option<Entity>,
    random_text: Option<Entity>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // let StateData { mut world, .. } = data;
        let mut world = data.world;

        let sprite_sheet = load_sprite_sheet(world, "ball");

        // Assign the sprite for the platform(s)
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0, // platform is the first/only sprite in the sprite_sheet
        };

        // world.insert(sprite_render.clone());

        initialise_camera(&mut world);
        initialize_platforms(&mut world, sprite_render.clone());
        initialize_resources(&mut world, sprite_render.clone());
        initialize_gunits(&mut world, sprite_render.clone());

        // needed for registering audio output.
        init_output(&mut world);

        // self.ui_root =
        //     Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())));
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = true;
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = false;
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("Failed to remove Game Screen");
        }
        self.ui_root = None;
        self.fps_display = None;
        self.random_text = None;
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Push(Box::new(PauseMenuState::default()))
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

        // this cannot happen in 'on_start', as the entity might not be fully
        // initialized/registered/created yet.
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

        // it is important that the 'paused' field is actually pausing your game.
        if !self.paused {
            let mut ui_text = world.write_storage::<UiText>();

            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 && !self.paused {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }

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
