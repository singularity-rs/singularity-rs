use crate::gunit::general_unit::GUnitType;
use crate::platform::platform_actions::PlatformActionType;
use crate::resources::ResourceType;
use amethyst::ecs::Entity;

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
#[derive(Default, Debug, Copy, Clone)]
pub struct Task {
    pub job_type: GUnitType,
    pub begin_platform: Option<Entity>,
    pub end_platform: Option<Entity>,
    pub resource_type: Option<ResourceType>,
    pub action: Option<PlatformActionType>,
}
