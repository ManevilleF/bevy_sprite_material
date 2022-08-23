use crate::Sprite;
use bevy::asset::{Assets, Handle};
use bevy::ecs::prelude::Entity;
use bevy::ecs::prelude::{Query, Res, ResMut};
use bevy::render::prelude::Visibility;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::render::Extract;
use bevy::sprite::{ColorMaterial, ExtractedSprite, ExtractedSprites};
use bevy::transform::prelude::GlobalTransform;
use copyless::VecHelper;

#[allow(clippy::type_complexity)]
pub fn extract_sprites(
    mut extracted_sprites: ResMut<ExtractedSprites>,
    materials: Extract<Res<Assets<ColorMaterial>>>,
    sprite_query: Extract<
        Query<(
            Entity,
            &Visibility,
            &Sprite,
            &GlobalTransform,
            &Handle<ColorMaterial>,
        )>,
    >,
) {
    for (entity, visibility, sprite, transform, handle) in sprite_query.iter() {
        if !visibility.is_visible {
            continue;
        }
        let material = materials.get(handle).cloned().unwrap_or_default();
        let (color, image_handle_id) = (
            material.color,
            material
                .texture
                .unwrap_or_else(|| DEFAULT_IMAGE_HANDLE.typed())
                .id,
        );
        // PERF: we don't check in this function that the `Image` asset is ready, since it should be in most cases and hashing the handle is expensive
        extracted_sprites.sprites.alloc().init(ExtractedSprite {
            entity,
            color,
            transform: *transform,
            // Use the full texture
            rect: None,
            // Pass the custom size
            custom_size: sprite.custom_size,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            image_handle_id,
            anchor: sprite.anchor.as_vec(),
        });
    }
}
