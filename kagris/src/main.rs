use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowResizeConstraints},
};
use bevy_egui::EguiPlugin;
use colored::Colorize;
use kgrs_audio::music::*;
use kgrs_config::Config;
use kgrs_const::color::BG_COL;
use kgrs_core::{board::BoardPlugin, mino::MinoPlugin};
use kgrs_debug::{debug_ui::DebugUiPlugin, toggle_fullscreen::ToggleFullscreenPlugin};

fn main() {
    // The application
    let mut app = App::new();

    // Plugins
    app.add_plugins(default_plugins())
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(DebugUiPlugin)
        .add_plugin(ToggleFullscreenPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MinoPlugin);

    // Startup systems
    app.add_startup_system_to_stage(StartupStage::PreStartup, pre_startup)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_music);

    // Resources
    app.insert_resource(ClearColor(BG_COL));

    // In development
    #[cfg(debug_assertions)]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    // Run the application.
    app.run();
}

/// Pre-startup systems
fn pre_startup(mut _cmds: Commands) {
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
    if cfg!(debug_assertions) {
        info!("{t_v} {}", "[DEV-MODE]".white().bold());
    } else {
        info!("{t_v}");
    }
}

fn setup_camera(mut cmds: Commands) {
    info!("Setting up camera");
    cmds.spawn(Camera2dBundle::default());
}

fn default_plugins() -> PluginGroupBuilder {
    let config = Config::load();
    DefaultPlugins.set(WindowPlugin {
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
    })
}
