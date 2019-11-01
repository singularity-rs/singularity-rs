
pub struct ImguiDemoSystem;
impl<'s> amethyst::ecs::System<'s> for ImguiDemoSystem {
    type SystemData = ();
    fn run(&mut self, _: Self::SystemData) {
        amethyst_imgui::with(|ui| {
            ui.show_demo_window(&mut true);
        });
    }
}

