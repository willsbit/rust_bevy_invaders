#![allow(unused)]

use bevy::prelude::*;
use crate::player::PlayerPlugin;

mod components;
mod player;

// region: ---- Asset constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144.0, 75.0);
const SPRITE_SCALE: f32 = 0.5;
// endregion: ---- Asset constants

// region:  --- Game constants
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;
// endregion:  --- Game constants

// region: ---- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>
}
// endregion: ---- Resources

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
        .add_plugin(PlayerPlugin)
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

    /// Add WinSize resource
    let win_size = WinSize{w: win_w, h: win_h};
    commands.insert_resource(win_size);

    /// Add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };

    commands.insert_resource(game_textures);
}
