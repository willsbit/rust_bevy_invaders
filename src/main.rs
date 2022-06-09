#![allow(unused)]

use bevy::prelude::*;

// region: ---- Asset constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144.0, 75.0);
const SPRITE_SCALE: f32 = 0.5;
// endregion: ---- Asset constants

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run()
}


fn setup_system(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut windows: ResMut<Windows>) {
    /// Add a 2D camera entity to the current scene
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window
    window.set_position(IVec2::new(500, 500));

    // Add a rectangle
    let bottom = -win_h/2.0;

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1/2.0 * SPRITE_SCALE + 5.0, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}