use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::sprite::Anchor;

/// 2D Sprite component
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
    /// [`bevy::sprite::Anchor`] point of the sprite in the world
    pub anchor: Anchor,
}

impl Sprite {
    /// Instantiates an unflipped sprite with a custom size
    pub fn sized(size: Vec2) -> Self {
        Self {
            custom_size: Some(size),
            ..Default::default()
        }
    }
}
