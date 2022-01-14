// #![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    nonstandard_style,
    rustdoc::broken_intra_links
)]

use bevy_app::{App, Plugin};
use bevy_render::RenderStage;
pub use bevy_sprite::ColorMaterial;
pub use sprite::Sprite;

mod bundle;
mod extract;
mod sprite;

#[derive(Default)]
pub struct MaterialSpritePlugin;

impl Plugin for MaterialSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(RenderStage::Extract, extract::extract_sprites);
    }
}
