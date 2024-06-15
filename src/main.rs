use std::f32::consts::PI;

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::time::Stopwatch;
use bevy::window::{PrimaryWindow, WindowMode};

pub fn close_on_esc(
    mut commands: Commands<'_, '_>,
    focused_windows: Query<'_, '_, (Entity, &Window)>,
    input: Res<'_, ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

// Window
// const WW: f32 = 1024.0;
// const WH: f32 = 576.0;

// Sprites
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const TILE_W: u32 = 16;
const TILE_H: u32 = 16;
const SPRITE_SHEET_W: u32 = 4;
const SPRITE_SHEET_H: u32 = 4;

//Player
const PLAYER_SPEED: f32 = 2.0;

// Gun
const GUN_HEIGHT: f32 = 16.0;
const BULLET_SPAWN_INTERVAL: f32 = 0.1;
const BULLET_SPEED: f32 = 8.0;

// Colors
const BG_COLOR: (u8, u8, u8) = (251, 245, 239);

// Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);
#[derive(Resource)]
struct CursorPosition(Option<Vec2>);

// Components
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Gun;
#[derive(Component)]
struct GunTimer(Stopwatch);
#[derive(Component)]
struct Bullet;
#[derive(Component)]
struct BulletDirection(Vec3);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        focused: true,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        // Custom resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .insert_resource(CursorPosition(None))
        // Systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(
            Update,
            (
                handle_player_input,
                update_gun_transform,
                update_cursor_position,
                handle_gun_input,
                update_bullets,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, close_on_esc)
        .run();
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    texture_atlas.0 = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::GameInit);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            texture: image_handle.0.clone().unwrap(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 0,
        },
        Player,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            texture: image_handle.0.clone().unwrap(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 2,
        },
        Gun,
        GunTimer(Stopwatch::new()),
    ));

    next_state.set(GameState::InGame);
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform: Mut<Transform> = player_query.single_mut();
    let w_key: bool =
        keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let s_key: bool =
        keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let a_key: bool =
        keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let d_key: bool =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta: Vec2 = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    delta = delta.normalize_or_zero();

    transform.translation += Vec3 {
        x: delta.x,
        y: delta.y,
        z: 0.0,
    } * PLAYER_SPEED;
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    image_handle: ResMut<GlobalSpriteSheetHandle>,
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

    let bullet_direction: Vec3 = gun_transform.local_x().into();
    let rotation_90 = Quat::from_rotation_z(std::f32::consts::PI / 2.0);

    gun_timer.0.reset();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(vec3(gun_pos.x, gun_pos.y, -1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            texture: image_handle.0.clone().unwrap(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 3,
        },
        Bullet,
        BulletDirection(rotation_90.mul_vec3(bullet_direction)),
    ));
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
) {
    if bullet_query.is_empty(){
        return;
    }

    for(mut t, dir) in bullet_query.iter_mut(){
        t.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED);
    }
}

fn update_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_pos: Vec2 = player_query.single().translation.truncate();
    let cursor_pos: Vec2 = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut gun_transform = gun_query.single_mut();

    let gun_offset = Vec3::new(4.0, -8.0, 0.0);

    let angle: f32 =
        (cursor_pos.y - player_pos.y - gun_offset.y).atan2(cursor_pos.x - player_pos.x) - PI / 2.0;
    let rotation_quat = Quat::from_rotation_z(angle);

    let origin_offset = Vec3::new(0.0, GUN_HEIGHT * SPRITE_SCALE_FACTOR / 2.0, 0.0);

    let new_gun_pos =
        player_pos.extend(gun_transform.translation.z) + rotation_quat * origin_offset + gun_offset;

    gun_transform.rotation = rotation_quat;
    gun_transform.translation = new_gun_pos;
}
