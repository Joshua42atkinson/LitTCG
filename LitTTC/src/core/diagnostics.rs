// diagnostics.rs — FPS and runtime diagnostics for desktop / XR targets
use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone)]
pub struct FrameDiagnostics {
    pub fps: f32,
    pub frame_count: u32,
}

pub fn update_frame_diagnostics(
    time: Res<Time>,
    mut diagnostics: ResMut<FrameDiagnostics>,
) {
    diagnostics.frame_count += 1;
    let delta = time.delta_secs();
    if delta > 0.0 {
        diagnostics.fps = 1.0 / delta;
    }
    if diagnostics.frame_count.is_multiple_of(120) {
        info!("FPS Diagnostic Overlay: {:.1} fps", diagnostics.fps);
    }
}
