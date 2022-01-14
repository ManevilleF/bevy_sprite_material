use bevy_ecs::component::Component;
use bevy_math::Vec2;

#[derive(Component, Debug, Default, Clone)]
#[repr(C)]
pub struct Sprite {
    /// Flip the sprite along the X axis
    pub flip_x: bool,
    /// Flip the sprite along the Y axis
    pub flip_y: bool,
    /// An optional custom size for the sprite that will be used when rendering, instead of the size
    /// of the sprite's image
    pub custom_size: Option<Vec2>,
}

impl Sprite {
    pub fn sized(size: Vec2) -> Self {
        Self {
            custom_size: Some(size),
            ..Default::default()
        }
    }
}
