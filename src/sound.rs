use bevy::prelude::*;

use crate::{state::GameState, MUSIC_PATH};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Loading), setup_volume);
    }
}

fn setup_volume(
    asset_server: Res<AssetServer>, mut commands: Commands
){
    commands.spawn(AudioBundle {
        source: asset_server.load(MUSIC_PATH),
        ..default()
    });
}