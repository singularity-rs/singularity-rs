/// Provides layer values for the different game objects. The smaller the value the more in the back the unit
/// gets drawn. The idea for this class was this we can dynamically add and change values without having to
/// touch the actual draw code at all. These constants should be used for the layerDepth argument
/// in the Draw() method for the sprite batch.

#[allow(non_upper_case_globals, dead_code)]
/// A float depicting the layer for the map
const MapLayer: f32 = 0.0;

#[allow(non_upper_case_globals, dead_code)]
/// A float depicting the layer for resources
const MapResourceLayer: f32 = 0.1;

#[allow(non_upper_case_globals, dead_code)]
/// A float depicting the layer for military units
const FreeMovingUnitLayer: f32 = 0.2;

#[allow(non_upper_case_globals, dead_code)]
/// A float depicting the layer for roads
pub const RoadLayer: f32 = 0.3;

#[allow(non_upper_case_globals)]
/// A float depicting the layer for platforms bases
pub const BasePlatformLayer: f32 = 0.4;

#[allow(non_upper_case_globals)]
/// A float depicting the layer for Resources
pub const ResourceLayer: f32 = 0.5;

#[allow(non_upper_case_globals, dead_code)]
/// A float depicting the layer for general units
pub const GeneralUnitLayer: f32 = 0.6;

#[allow(non_upper_case_globals, dead_code)]
/// A float depicting the layer for the fog of war
const FogOfWarLayer: f32 = 0.9;

//*
#[allow(non_upper_case_globals, dead_code)]
/// A float depicting the layer for the connection between Resources and Units.
const ConnectingLayer: f32 = 0.52;
// */
