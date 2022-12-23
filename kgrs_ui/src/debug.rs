use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::PresentMode,
};
use bevy_egui::{egui, EguiContext};
use kgrs_config::Config;

pub fn debug_ui(
    mut egui_ctx: ResMut<EguiContext>,
    diags: Res<Diagnostics>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.primary_mut();
    let enable_vsync = matches!(window.present_mode(), PresentMode::AutoVsync);
    egui::Window::new("Debug")
        .open(&mut true) // TODO: what
        // .vscroll(false)
        // .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Frame time");
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
        });
}

pub fn toggle_debug_ui(mut input: EventReader<KeyboardInput>) {
    for r#in in input.iter() {
        if let Some(key) = r#in.key_code {
            if key == KeyCode::F3 && r#in.state == ButtonState::Pressed {
                info!("F3 pressed");
            }
        }
    }
}
