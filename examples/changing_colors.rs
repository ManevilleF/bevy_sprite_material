use bevy::prelude::*;
use bevy_sprite_material::{MaterialSpritePlugin, Sprite, SpriteBundle};

#[derive(Debug, Clone)]
struct MaterialResource {
    material: Handle<ColorMaterial>,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game Of Life".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialSpritePlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .add_system(change_color)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let (size_x, size_y) = (10, 10);
    let sprite_size = 30.;

    let material = materials.add(Color::BLUE.into());

    for y in 0..size_y {
        for x in 0..size_x {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite::sized(Vec2::splat(sprite_size)),
                material: material.clone(),
                transform: Transform::from_xyz(x as f32 * 35. - 175., y as f32 * 35. - 175., 0.),
                ..Default::default()
            });
        }
    }
    commands.insert_resource(MaterialResource { material });
    println!("map generated");
}

fn change_color(mut materials: ResMut<Assets<ColorMaterial>>, material: Res<MaterialResource>) {
    let material = materials.get_mut(material.material.clone()).unwrap();
    if material.color.r() < 1. {
        material.color.set_r(material.color.r() + 0.02);
    } else {
        material.color.set_r(0.);
    }
}
