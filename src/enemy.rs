use std::f32::consts::PI;
use std::time::Duration;

use animation::AnimationTimer;
use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::player::Player;
use crate::state::GameState;
use crate::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                despawn_dead_enemies,
                update_enemy_transform,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }

    let num_enemies: usize = enemy_query.iter().len();
    let enemy_spawn_count: usize = (MAX_NUMBER_ENEMY - num_enemies).min(SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUMBER_ENEMY {
        return;
    }

    let player_pos: Vec2 = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
        let random_position_around_player = get_random_position_around(player_pos);
        let x = random_position_around_player.0;
        let y = random_position_around_player.1;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(vec3(x, y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                texture: handle.image.clone().unwrap(),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 8,
            },
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            Enemy::default(),
        ));
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32){
    let mut rng: ThreadRng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(2048.0..4096.0);

    let offset_x = dist * angle.cos();
    let offset_y = dist * angle.sin();

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    return (random_x, random_y);
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos: Vec2 = player_query.single().translation.truncate();

    for mut transform in enemy_query.iter_mut() {
        let dir: Vec3 = (player_pos.extend(0.0) - transform.translation).normalize();

        transform.translation += dir * ENEMY_SPEED;
    }
}
