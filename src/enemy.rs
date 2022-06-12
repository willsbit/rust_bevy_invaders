use std::cmp::max;
use std::f32::consts::PI;
use bevy::core::FixedTimestep;
use bevy::ecs::schedule::ShouldRun;
use crate::{BASE_SPEED, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, ENEMY_SPRITE, EnemyCount, GameTextures, Laser, Movable, SPRITE_SCALE, TIME_STEP, Velocity, WinSize};
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use rand::{Rng, thread_rng};
use crate::components::{Enemy, FromEnemy, SpriteSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0))
            .with_system(enemy_spawn_system)
        )
            .add_system_set(SystemSet::new()
                .with_run_criteria(enemy_fire_criteria)
                .with_system(enemy_fire_system)
            ).add_system(enemy_movement_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>
) {
    if enemy_count.0 < ENEMY_MAX {
        // compute the x, y
        let mut rng = thread_rng();
        let w_span = win_size.w / 2.0 - 100.0;
        let h_span = win_size.h / 2.0 - 100.0;
        let x = rng.gen_range(-w_span.. w_span);
        let y = rng.gen_range(-h_span.. h_span);


        commands.spawn_bundle(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 10.0),
                rotation: Quat::from_rotation_x(PI),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        // spawn enemy laser sprite
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15.0, 0.0),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable {auto_despawn: true})
            .insert(Velocity {x: 0.0, y: -1.0});
    }
}


fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1.0 / 60.0) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Enemy>>
) {

    let now = time.seconds_since_startup() as f32;

    for mut transform in query.iter_mut() {
        // current position
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        // max distance
        let max_distance = TIME_STEP * BASE_SPEED;

        // fixtures
        let dir: f32 = -1.0; // 1 for counter-clockwise, -1 for clockwise
        let (x_pivot, y_pivot) = (0.0, 0.0);
        let (x_radius, y_radius) = (200.0, 130.0);

        // compute next angle
        let angle = dir * BASE_SPEED * TIME_STEP * now % 360.0 / PI;

        // compute target x,y
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = x_radius * angle.sin() + y_pivot;

        // compute distance
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;

        let distance = (dx^2 + dy^2).sqrt();
        let distance_ratio = if distance != 0.0 {max_distance / distance}
                            else {0.0};

        // compute next x,y
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0.0 { x.max(x_dst) }
                    else {x.min(x_dst)};

        let y = y_org - dy * distance_ratio;
        let y = if dy > 0.0 { y.may(y_dst) }
                     else {y.min(y_dst)};

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}