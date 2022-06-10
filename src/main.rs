#![allow(unused)]

use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;
use crate::components::{Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;

mod components;
mod enemy;
mod player;

// region: ---- Asset constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;
// endregion: ---- Asset constants

// region:  --- Game constants
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;
const SPRITE_SCALE: f32 = 0.5;


const PLAYER_SIZE: (f32, f32) = (144.0, 75.0);
const PLAYER_LASER_SIZE: (f32, f32) = (9.0, 54.0);

const ENEMY_SIZE: (f32, f32) = (144.0, 75.0);
const ENEMY_LASER_SIZE: (f32, f32) = (17.0, 55.0);
// endregion:  --- Game constants

// region: ---- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>
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
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .run()
}

fn setup_system(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    /// Create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    /// Add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion
    };

    commands.insert_resource(game_textures);
}


fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            /// Despawn components when they go beyond window bounds
            const MARGIN: f32 = 200.0;
            if translation.y > win_size.h /2.0 + MARGIN
                || translation.y < -win_size.h /2.0 - MARGIN
                || translation.x > win_size.w /2.0 + MARGIN
                || translation.x < -win_size.w /2.0 - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}


fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>
) {
    // iterate through the lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate through the enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // check if there's a collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale
            );

            // perform the collision

            if let Some(_) = collision {
                // remove the enemy
                commands.entity(enemy_entity).despawn();

                // remove the laser
                commands.entity(laser_entity).despawn();

                // spawn the ExplosionToSpawn
                commands.spawn().insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}


fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        // spawn the explosion sprite
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        // despawn the explosion
        commands.entity(explosion_spawn_entity).despawn();
    }
}


fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}