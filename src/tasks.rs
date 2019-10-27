use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Component, DenseVecStorage, VecStorage, World, WorldExt, Entity},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};
use derivative::Derivative;
use crate::general_unit::*;
use crate::resources::*;
use crate::platform_actions::*;

/// A `Task` has one of the following configurations, based on GUnitType:
/// Idle:
///     end_platform: where to go next, as to keep the Unit moving.
/// (further Job Types do not exist yet)
// Carry:
//      begin_platform: Platform to take a specific resource from
//      end_platform: Platform to transport the resource to
//      resource_type: Type of Resource to transport
// Production:
//      end_platform: Platform to do the Job
//      action: PlatformAction to work on
// Defense:
//      end_platform: Platform to defend on
//      action: DefensePlatformAction of the defending platform
// Build:
//      begin_platform: Platform to get a Resource from
//      end_platform: Platform on which something is built (e.g. a Blueprint)
#[derive(Default)]
pub struct Task {
    pub job_type: GUnitType,
    pub begin_platform: Option<Entity>,
    pub end_platform: Option<Entity>,
    pub resource_type: Option<ResourceType>,
    pub action: Option<PlatformActionType>,
}

