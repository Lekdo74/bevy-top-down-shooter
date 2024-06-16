use bevy::prelude::*;

pub struct CloseOnEscapePlugin;

impl Plugin for CloseOnEscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, close_on_esc);
    }
}

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
