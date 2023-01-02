use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowResizeConstraints},
};
use bevy_egui::EguiPlugin;
use colored::Colorize;
use kgrs_audio::music::*;
use kgrs_config::Config;
use kgrs_const::color::BG_COL;
use kgrs_debug::{debug_ui::DebugUiPlugin, toggle_fullscreen::ToggleFullscreenPlugin};
use kgrs_game::{board::BoardPlugin, mino::MinoPlugin};

fn main() {
    let config = Config::load();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: format!("KaGRiS v{}", env!("CARGO_PKG_VERSION")),
                resize_constraints: WindowResizeConstraints {
                    min_width: 512.,
                    min_height: 288.,
                    ..default()
                },
                present_mode: if config.vsync {
                    PresentMode::AutoVsync
                } else {
                    PresentMode::AutoNoVsync
                },
                mode: config.window_mode.to_window_mode(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(DebugUiPlugin)
        .add_plugin(ToggleFullscreenPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MinoPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_music)
        .insert_resource(ClearColor(BG_COL))
        .run();
}

/// Main setup function
fn setup(mut _cmds: Commands) {
    let title = format!(
        "{}{}{}{}{}{}",
        "K".red(),
        "a".magenta(),
        "G".yellow(),
        "R".green(),
        "i".cyan(),
        "S".blue()
    )
    .bold();
    let version = format!("v{}", env!("CARGO_PKG_VERSION")).white();
    let t_v = format!("| {} {} |", title, version).on_black();
    info!("{t_v}");
}

fn setup_camera(mut cmds: Commands) {
    info!("Setting up camera");
    cmds.spawn(Camera2dBundle::default());
}
