use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use enemy::Enemy;
use gun::Bullet;
use kd_tree::{KdPoint, KdTree};

use crate::state::GameState;
use crate::*;

pub struct CollisionPlugin;

#[derive(Component)]
struct Collidable {
    pos: Vec2,
    entity: Entity,
}
#[derive(Resource)]
pub struct EnemyKdTree {
    kd_tree: KdTree<[f32; 2]>,
    collidables: Vec<Collidable>,
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyKdTree::default()).add_systems(
            Update,
            (
                handle_enemy_bullet_collision,
                update_enemy_kd_tree
                    .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self {
            kd_tree: KdTree::build_by_ordered_float(vec![]),
            collidables: Vec::new(),
        }
    }
}

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.pos.x;
        }
        return self.pos.y;
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut positions = Vec::new();
    let mut collidables = Vec::new();

    for (t, e) in enemy_query.iter() {
        let pos = t.translation.truncate();
        positions.push([pos.x, pos.y]);
        collidables.push(Collidable { pos, entity: e });
    }

    tree.kd_tree = KdTree::build_by_ordered_float(positions);
    tree.collidables = collidables;
}

fn handle_enemy_bullet_collision(
    bullet_query: Query<&Transform, With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<&mut Enemy>,
) {
    if bullet_query.is_empty() || tree.collidables.is_empty() {
        return;
    }

    for b_t in bullet_query.iter() {
        let pos = b_t.translation;
        let enemies = tree.kd_tree.within_radius(&[pos.x, pos.y], 50.0);

        for enemy_pos in enemies {
            if let Some(collidable) = tree
                .collidables
                .iter()
                .find(|c| c.pos == Vec2::new(enemy_pos[0], enemy_pos[1]))
            {
                if let Ok(mut enemy) = enemy_query.get_mut(collidable.entity) {
                    enemy.health -= BULLET_DAMAGE;
                }
            }
        }
    }
}
