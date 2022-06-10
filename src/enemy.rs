use bevy::core::FixedTimestep;
use crate::{ENEMY_MAX, ENEMY_SIZE, ENEMY_SPRITE, EnemyCount, GameTextures, SPRITE_SCALE, WinSize};
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::components::{Enemy, SpriteSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0))
            .with_system(enemy_spawn_system)
        )
            .add_system(enemy_fire_system);
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

fn enemy_fire_system() {

}
