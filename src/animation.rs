use bevy::prelude::*;

use crate::{
    player::Player,
    state::GameState, CursorPosition,
};

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animation_timer_tick, animate_player).run_if(in_state(GameState::InGame)));
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>
){
    for mut timer in query.iter_mut(){
        timer.tick(time.delta());
    }
}

fn animate_player(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut TextureAtlas, &mut Sprite, &Transform, &AnimationTimer), With<Player>>
) {
    if player_query.is_empty() {
        return;
    }

    let (mut texture, mut sprite, transform, timer) = player_query.single_mut();
    if timer.just_finished(){
        texture.index = (texture.index + 1) % 8;
    }

    if let Some(cursor_position) = cursor_position.0{
        if cursor_position.x < transform.translation.x{
            sprite.flip_x = true;
        }
        else{
            sprite.flip_x = false;
        }
    }
}
