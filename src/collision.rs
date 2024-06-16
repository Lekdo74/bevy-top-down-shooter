use bevy::prelude::*;
use enemy::Enemy;
use gun::Bullet;

use crate::state::GameState;
use crate::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_enemy_bullet_collision).run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_enemy_bullet_collision(
    bullet_query: Query<&Transform, With<Bullet>>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    if enemy_query.is_empty() || bullet_query.is_empty() {
        return;
    }

    for b_t in bullet_query.iter() {
        for (e_t, mut e) in enemy_query.iter_mut() {
            if b_t.translation.distance_squared(e_t.translation)
                <= TILE_W as f32 * 10.0 * SPRITE_SCALE_FACTOR
            {
                e.health -= BULLET_DAMAGE;
            }
        }
    }
}
