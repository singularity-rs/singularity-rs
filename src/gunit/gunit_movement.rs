use crate::gunit::general_unit::*;
use amethyst::{
    core::math::*,
    core::transform::Transform,
    ecs::prelude::{Join, System, Write, WriteStorage},
    // renderer::palette::Srgba,
    renderer::debug_drawing::DebugLines,
};

#[derive(Default)]
pub struct GUnitMovementSystem;

impl<'s> System<'s> for GUnitMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, GUnitAttributes>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (mut transforms, mut gunits, mut _debug_lines): Self::SystemData) {
        println!("Moving Units");
        for (gunit, trans) in (&mut gunits, &mut transforms).join() {
            if let Some(target) = &gunit.get_target_location() {
                let mut diff = *target - trans.translation();
                let mut steer = Vector3::zeros();
                let mut steer_weight = 0.0;
                if let Some(begin) = &gunit.get_starting_location() {
                    let a = begin;
                    let b = target;
                    let ab = target - begin;
                    let current = trans.translation();
                    let iso = Isometry3::new(
                        Vector3::new(1.0, 1.0, 0.0),
                        Vector3::new(0.0, 0.0, -std::f32::consts::PI / 2.0),
                    );
                    let bd = iso * (ab / ab.norm()) * TARGET_OFFSET;
                    let d = b + bd;
                    let c = a + bd;

                    diff = d - current;
                    steer = -bd;
                    steer_weight =
                        (c.x - d.x) * (current.y - d.y) - (c.y - d.y) * (current.x - d.x);
                    // for some reason, this fixes everything.
                    // Steering would be too sensitive otherwise.
                    steer_weight /= 1000.0;

                    // // the negative steering value (bd) in blue
                    // debug_lines.draw_direction(
                    //     Point::from(*current),
                    //     bd,
                    //     Srgba::new(0.0, 0.0, 1.0, 1.0),
                    // );

                    // // the current goal in red
                    // debug_lines.draw_direction(
                    //     Point::from(*current),
                    //     diff,
                    //     Srgba::new(1.0, 0.0, 0.0, 1.0),
                    // );

                    // // the weighted steering in green
                    // debug_lines.draw_direction(
                    //     Point::from(*current),
                    //     steer * steer_weight,
                    //     Srgba::new(0.0, 1.0, 0.0, 1.0),
                    // );

                    // A = D
                    // B = C
                    // Line AB:
                    // steer_weight = (Bx - Ax) * (Y - Ay) - (By - Ay) * (X - Ax)
                    // (how much a given point X, Y is away from the line)
                }

                let mut vel = *gunit.get_velocity();

                if diff.norm() < SLOWDOWN_DIST {
                    vel = diff.norm() * vel / SLOWDOWN_DIST;
                }

                if diff.norm() < ARRIVED_DIST {
                    gunit.arrive();
                }

                let moment = diff.normalize() + steer.normalize() * steer_weight;

                *trans.translation_mut() += moment.normalize() * vel;
                trans.set_translation_z(crate::layers::GeneralUnitLayer);
            }
        }
    }
}
