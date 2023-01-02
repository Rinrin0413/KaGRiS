use bevy::prelude::*;
use kgrs_config::{Config, WindowModeForConf};
use kgrs_util::function::fmt::wm_to_string;

pub struct ToggleFullscreenPlugin;

impl Plugin for ToggleFullscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(toggle_fullscreen);
    }
}

fn toggle_fullscreen(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if input.just_pressed(KeyCode::F11) {
        let window = windows.primary_mut();
        let window_mode = match window.mode() {
            WindowMode::Windowed => WindowMode::Fullscreen,
            _ => WindowMode::Windowed,
        };
        info!(
            "Changing window mode from {} to {}",
            wm_to_string(window.mode()),
            wm_to_string(window_mode)
        );
        window.set_mode(window_mode);

        let mut config = Config::load();
        config.window_mode = WindowModeForConf::from_window_mode(window_mode);
        config.save();
    }
}
