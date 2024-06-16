use animation::AnimationTimer;
use bevy::{math::vec3, prelude::*, time::Stopwatch};
use player::PlayerState;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::*;
use crate::{
    gun::{Gun, GunTimer},
    player::Player,
    state::GameState,
    GlobalTextureAtlas,
};

pub struct WorldPlugin;

#[derive(Component)]
pub struct GameEntity;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorations).run_if(in_state(GameState::GameInit)),
        );
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(vec3(0.0, 0.0, 10.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            texture: handle.image.clone().unwrap(),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        PlayerState::default(),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(vec3(0.0, 0.0, 12.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            texture: handle.image.clone().unwrap(),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 17,
        },
        Gun,
        GunTimer(Stopwatch::new()),
    ));

    next_state.set(GameState::InGame);
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng: ThreadRng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..=WORLD_W);
        let y = rng.gen_range(-WORLD_H..=WORLD_H);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                texture: handle.image.clone().unwrap(),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(24..=27),
            },
        ));
    }
}
