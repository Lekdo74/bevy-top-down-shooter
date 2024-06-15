use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::WindowMode;

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
const BG_COLOR: (u8, u8, u8) = (251, 245, 239);

// Sprites
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const TILE_W: u32 = 16;
const TILE_H: u32 = 16;
const SPRITE_SHEET_W: u32 = 4;
const SPRITE_SHEET_H: u32 = 4;

fn main() {
    App::new()
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
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(SPRITE_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
    ));
}
