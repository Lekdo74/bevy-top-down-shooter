use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    player::{Player, PlayerState},
    state::GameState,
    CursorPosition, SPRITE_SHEET_W,
};

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animation_timer_tick,
                animate_player,
                flip_player_sprite_x,
                animate_enemy,
                flip_enemy_sprite_x,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlas, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut texture_atlas, player_state, timer) = player_query.single_mut();
    if timer.just_finished() {
        texture_atlas.index = match player_state {
            PlayerState::Idle => 2,
            PlayerState::Moving => (texture_atlas.index + 1) % SPRITE_SHEET_W as usize,
        };
    }
}

fn animate_enemy(mut enemy_query: Query<(&mut TextureAtlas, &AnimationTimer), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut texture_atlas, timer) in enemy_query.iter_mut() {
        if timer.just_finished() {
            texture_atlas.index = 8 + (texture_atlas.index + 1) % 4;
        }
    }
}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = player_query.single_mut();

    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x < transform.translation.x {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}

fn flip_enemy_sprite_x(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos: Vec3 = player_query.single().translation;
    for (mut sprite, transform) in enemy_query.iter_mut() {
        if transform.translation.x > player_pos.x {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}
