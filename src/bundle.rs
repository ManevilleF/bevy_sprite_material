use crate::Sprite;
use bevy::asset::Handle;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::ComputedVisibility;
use bevy::render::view::Visibility;
use bevy::sprite::ColorMaterial;
use bevy::transform::components::{GlobalTransform, Transform};

/// Component bundle for 2D sprites with a `ColorMaterial`
#[derive(Bundle, Clone, Default)]
pub struct SpriteBundle {
    /// The main sprite component
    pub sprite: Sprite,
    /// transform component, defining translation/rotation/scale informations
    pub transform: Transform,
    /// transform component, defining translation/rotation/scale informations
    pub global_transform: GlobalTransform,
    /// The sprite material, defining its texture and color
    pub material: Handle<ColorMaterial>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// system indication of whether an entity is visible
    pub computed_visibility: ComputedVisibility,
}
