use std::f32::consts::PI;
use bevy::utils::Instant;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::player::Player;
use crate::state::GameState;
use crate::*;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer(pub Stopwatch);
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
struct BulletDirection(Vec3);
#[derive(Component)]
struct SpawnInstant(Instant);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_gun_transform,
                despawn_old_bullets,
                update_bullets,
                handle_gun_input,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (instant, entity) in bullet_query.iter_mut() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
            commands.entity(entity).despawn();
        }
    }
}

fn update_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<(&mut Sprite, &mut Transform), (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_pos: Vec2 = player_query.single().translation.truncate();
    let cursor_pos: Vec2 = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let (mut gun_sprite, mut gun_transform) = gun_query.single_mut();

    let mut gun_offset = Vec3::new(4.0, -8.0, 0.0);
    if cursor_pos.x < player_pos.x {
        gun_offset = Vec3::new(-4.0, -8.0, 0.0);
        gun_sprite.flip_x = true;
    } else {
        gun_sprite.flip_x = false;
    }

    let angle: f32 =
        (cursor_pos.y - player_pos.y - gun_offset.y).atan2(cursor_pos.x - player_pos.x) - PI / 2.0;
    let rotation_quat = Quat::from_rotation_z(angle);

    let origin_offset = Vec3::new(0.0, GUN_HEIGHT * SPRITE_SCALE_FACTOR / 2.0, 0.0);

    let new_gun_pos =
        player_pos.extend(gun_transform.translation.z) + rotation_quat * origin_offset + gun_offset;

    gun_transform.rotation = rotation_quat;
    gun_transform.translation = new_gun_pos;
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos: Vec2 = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    if gun_timer.0.elapsed_secs() < BULLET_SPAWN_INTERVAL {
        return;
    }

    gun_timer.0.reset();

    let rotation_90 = Quat::from_rotation_z(PI / 2.0);
    let bullet_direction: Vec3 = rotation_90.mul_vec3(gun_transform.local_x().into());

    let mut rng = rand::thread_rng();
    for _ in 0..NUM_BULLETS_PER_SHOT {
        let random_dir = Vec3 {
            x: bullet_direction.x + rng.gen_range(-1.0..1.0),
            y: bullet_direction.y + rng.gen_range(-1.0..1.0),
            z: bullet_direction.z,
        };

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 11.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                texture: handle.image.clone().unwrap(),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 16,
            },
            Bullet,
            BulletDirection(bullet_direction + random_dir),
            SpawnInstant(Instant::now()),
        ));
    }
}

fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
    time: Res<Time>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED * time.delta_seconds());
    }
}
