use bevy::prelude::*;
use crate::{BASE_SPEED, GameTextures, PLAYER_SIZE, PLAYER_SPRITE, SPRITE_SCALE, TIME_STEP, WinSize};
use crate::components::{Player, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(mut commands: Commands,
                       game_texture: Res<GameTextures>,
                       win_size: Res<WinSize>) {
    // Add a rectangle
    let bottom = -win_size.h/2.0;

    commands.spawn_bundle(SpriteBundle {
        texture: game_texture.player.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1/2.0 * SPRITE_SCALE + 5.0, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Player)
        .insert(Velocity {x:0.0, y: 0.0});
}


fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.0
        } else if kb.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };
    }
}


fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}