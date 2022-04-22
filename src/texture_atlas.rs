use bevy_ecs::component::Component;
use bevy_math::Vec2;
use bevy_sprite::Anchor;

/// 2D Texture Atlas Sprite Component
#[derive(Component, Debug, Default, Clone)]
pub struct TextureAtlasSprite {
    /// The Atlas section index
    pub index: usize,
    /// Flip the sprite along the X axis
    pub flip_x: bool,
    /// Flip the sprite along the Y axis
    pub flip_y: bool,
    /// An optional custom size for the sprite that will be used when rendering, instead of the size
    /// of the sprite's image in the atlas
    pub custom_size: Option<Vec2>,
    /// [`bevy_sprite::Anchor`] point of the sprite in the world
    pub anchor: Anchor,
}

impl TextureAtlasSprite {
    /// Creates a new `TextureAtlasSprite` with a custom `index`
    pub fn new(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }
}
