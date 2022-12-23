use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::PresentMode,
};
use bevy_egui::{egui, EguiContext};
use kgrs_config::{Config, WindowModeForConf};

/// Debug UI component
#[derive(Component)]
pub struct DebugUi {
    /// Whether the debug window is open
    pub open: bool,
}

impl DebugUi {
    pub fn init() -> Self {
        Self { open: false }
    }
}

pub fn debug_ui(
    mut egui_ctx: ResMut<EguiContext>,
    diags: Res<Diagnostics>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut DebugUi>,
) {
    let window = windows.primary_mut();
    let enable_vsync = matches!(window.present_mode(), PresentMode::AutoVsync);
    egui::Window::new("Debug")
        .open(&mut query.single_mut().open)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Performance");
            ui.label(format!(
                "FPS: {:.2}",
                diags
                    .get(FrameTimeDiagnosticsPlugin::FPS)
                    .unwrap()
                    .smoothed()
                    .unwrap_or_default()
            ));
            if ui.button(format!("VSync: {}", enable_vsync)).clicked() {
                window.set_present_mode(if enable_vsync {
                    PresentMode::AutoNoVsync
                } else {
                    PresentMode::AutoVsync
                });

                let mut config = Config::load();
                config.vsync = !enable_vsync;
                config.save();

                info!("PRESENT_MODE: {:?}", window.present_mode());
            }
            ui.label(format!(
                "Frame: {}",
                diags
                    .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .unwrap()
                    .value()
                    .unwrap_or_default()
            ));
            ui.label(format!(
                "Frame time: {:.2}ms",
                diags
                    .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
                    .unwrap()
                    .smoothed()
                    .unwrap_or_default()
            ));
            ui.separator();
            ui.label(format!(
                "Entities: {}",
                diags
                    .get(bevy::diagnostic::EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                    .unwrap()
                    .value()
                    .unwrap_or_default()
            ));
            ui.separator();
            ui.collapsing("Window", |ui_w| {
                ui_w.label(format!("Size: {}x{}", window.width(), window.height()));
                let w_p = window.position().unwrap_or_default();
                ui_w.label(format!("Pos: [{}, {}]", w_p.x, w_p.y));
                let mut window_mode = window.mode();
                egui::ComboBox::from_id_source("WindowMode")
                    .selected_text(wm_to_string(window_mode))
                    .show_ui(ui_w, |ui_w_wm| {
                        for wm in [
                            WindowMode::Windowed,
                            WindowMode::Fullscreen,
                            WindowMode::BorderlessFullscreen,
                            WindowMode::SizedFullscreen,
                        ]
                        .iter()
                        {
                            ui_w_wm.selectable_value(&mut window_mode, *wm, wm_to_string(*wm));
                        }
                    });
                if window_mode != window.mode() {
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
            });
        });
}

pub fn toggle_debug_ui(mut input: EventReader<KeyboardInput>, mut query: Query<&mut DebugUi>) {
    for r#in in input.iter() {
        if let Some(key) = r#in.key_code {
            if key == KeyCode::F3 && r#in.state == ButtonState::Pressed {
                for mut debug_ui in query.iter_mut() {
                    debug_ui.open = !debug_ui.open;
                }
            }
        }
    }
}

/// Formats `WindowMode` to a string.
fn wm_to_string(mode: WindowMode) -> String {
    match mode {
        WindowMode::Windowed => "Windowed",
        WindowMode::BorderlessFullscreen => "Borderless Fullscreen",
        WindowMode::SizedFullscreen => "Sized Fullscreen",
        WindowMode::Fullscreen => "Fullscreen",
    }
    .to_string()
}
