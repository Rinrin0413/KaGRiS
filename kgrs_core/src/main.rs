use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowResizeConstraints},
};
use bevy_egui::EguiPlugin;
use colored::Colorize;
use kgrs_audio::music::*;
use kgrs_config::Config;
use kgrs_const::color::*;
use kgrs_game::board::*;
use kgrs_ui::debug::*;

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
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_music)
        .add_startup_system(setup_board)
        .insert_resource(ClearColor(BG_COL))
        .add_system(resize_board)
        .add_system(debug_ui)
        .add_system(toggle_debug_ui)
        .run();
}

/// Main setup function
fn setup(mut _commands: Commands) {
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

fn setup_camera(mut commands: Commands) {
    info!("Setting up camera");
    commands.spawn(Camera2dBundle::default());
}
