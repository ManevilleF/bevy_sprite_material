use crate::Sprite;
use bevy_asset::Handle;
use bevy_ecs::bundle::Bundle;
use bevy_render::view::Visibility;
use bevy_sprite::ColorMaterial;
use bevy_transform::components::{GlobalTransform, Transform};

#[derive(Bundle, Clone, Default)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub material: Handle<ColorMaterial>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
}
