use crate::pause::PauseMenuState;
use crate::platform::*;
use crate::resources::*;
use crate::general_unit::*;
use crate::util::delete_hierarchy;
use crate::util::load_sprite_sheet;
use amethyst::{
    audio::output::init_output,
    core::{transform::Transform, ArcThreadPool},
    ecs::prelude::{Entity, WorldExt, Dispatcher, DispatcherBuilder},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Camera, SpriteRender},
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

fn initialize_platforms(world: &mut World) {
    // let sprite_sheet = load_sprite_sheet(world, "pong_spritesheet");

    world.register::<PlatformAttributes>();

}

fn initialize_resources(world: &mut World) {

    world.register::<ResourceAttributes>();
}

fn initialize_gunits(world: &mut World) {

    world.register::<GUnitAttributes>();
}


#[derive(Default)]
pub struct Game<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    paused: bool,
    ui_root: Option<Entity>,
}

impl<'a, 'b> SimpleState for Game<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        let dispatcher_builder = DispatcherBuilder::new()
            .with(crate::gunit_movement::GUnitMovementSystem::default(),
                  "gunit_movement_system",
                  &[],
            );
        // add systems here


        let sprite_sheet = load_sprite_sheet(world, "ball");

        // Assign the sprite for the platform(s)
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0, // platform is the first/only sprite in the sprite_sheet
        };

        // initializing ... mainly adding the required resources
        initialise_camera(&mut world);
        initialize_platforms(&mut world);
        initialize_resources(&mut world);
        initialize_gunits(&mut world);

        // needed for registering audio output.
        init_output(&mut world);

        // self.ui_root =
        //     Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())));


        // Build and setup the `Dispatcher`.
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);


        // ------------ DEBUG / TESTING -------------------------


        // this is only for debugging purposes
        let p1 = create_platform(
            PlatformAttributes::default(),
            world,
            sprite_render.clone(),
            200.,
            300.,
        );
        let p2 = create_platform(
            PlatformAttributes::default(),
            world,
            sprite_render.clone(),
            800.,
            600.,
        );

        connect_platforms(world, p1, p2);


        let unit = create_gunit(
            GUnitAttributes::new(),
            world,
            sprite_render.clone(),
            810.,
            605.,
        );

        unit_set_platform(world, unit, p2);

        unit_set_target_platform(world, unit, p1);


        create_resource(
            ResourceAttributes::new(ResourceType::Perl),
            world,
            sprite_render.clone(),
            200.,
            300.,
        );


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

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &data.world;

        // it is important that the 'paused' field is actually pausing your game.
        if !self.paused {

            if let Some(dispatcher) = self.dispatcher.as_mut() {
                dispatcher.dispatch(world);
            }
        }


        Trans::None
    }
}
