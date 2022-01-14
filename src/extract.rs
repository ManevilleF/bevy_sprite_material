use crate::Sprite;
use bevy_asset::{Assets, Handle};
use bevy_ecs::prelude::{Query, Res, ResMut};
use bevy_render::prelude::Visibility;
use bevy_render::texture::DEFAULT_IMAGE_HANDLE;
use bevy_render::RenderWorld;
use bevy_sprite::{ColorMaterial, ExtractedSprite, ExtractedSprites};
use bevy_transform::prelude::GlobalTransform;

pub fn extract_sprites(
    mut render_world: ResMut<RenderWorld>,
    materials: Res<Assets<ColorMaterial>>,
    sprite_query: Query<(
        &Visibility,
        &Sprite,
        &GlobalTransform,
        &Handle<ColorMaterial>,
    )>,
) {
    let mut extracted_sprites = render_world.get_resource_mut::<ExtractedSprites>().unwrap();
    for (visibility, sprite, transform, handle) in sprite_query.iter() {
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
        extracted_sprites.sprites.push(ExtractedSprite {
            color,
            transform: *transform,
            // Use the full texture
            rect: None,
            // Pass the custom size
            custom_size: sprite.custom_size,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            image_handle_id,
        });
    }
}
