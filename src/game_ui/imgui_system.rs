
use amethyst_imgui::imgui::*;
use amethyst_imgui::imgui;
use amethyst::ecs::{Write, System};
use imgui_inspect::*;
use imgui_inspect_derive::*;


#[derive(Default, Inspect)]
pub struct MyStruct {
    pub first_value: f32,
    pub second_value: f32,
}


// #[derive(Default, Inspect)]
// // #[derive(Default)]
// pub struct GUnitGraphSelectorSettings {
//     // #[imgui(slider(min = 0, max = 4))]
//     pub units_idle: i32,
//     // #[imgui(input(step = 2))]
//     pub y: i32,
//     // #[imgui(drag(label = "Drag 2D"))]
//     pub drag_2d: [f32; 2],
//     // #[imgui(checkbox(label = "Turbo mode"), display(label = "Is turbo enabled?"))]
//     pub turbo: bool,
// }




pub struct ImguiDemoSystem;

impl<'s> amethyst::ecs::System<'s> for ImguiDemoSystem {
    type SystemData = (
        Write<'s, MyStruct>,
        );
    fn run(&mut self, (mut graph_selector,): Self::SystemData) {
        amethyst_imgui::with(|ui| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
            ui.separator();
            let my_struct = graph_selector;
            // render::<MyStruct>(&[&my_struct], &"test", ui, &InspectArgsDefault::default());
            <MyStruct as InspectRenderDefault<MyStruct>>::render(&[&my_struct], &"test", ui, &InspectArgsDefault::default());

            <u32 as InspectRenderDefault<u32>>::render(&[&5u32], &"test", ui, &InspectArgsDefault::default());
        });
    }
}

