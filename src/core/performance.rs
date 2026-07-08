// performance.rs — Mobile + XR performance presets
use bevy::prelude::*;
use bevy::light::{DirectionalLightShadowMap, PointLightShadowMap};

/// Plugin that applies low-quality rendering settings on Android and XR.
pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(any(target_os = "android", feature = "xr"))]
        {
            // Reduce shadow map resolution to save fill rate.
            app.insert_resource(DirectionalLightShadowMap { size: 512 });
            app.insert_resource(PointLightShadowMap { size: 512 });

            // Disable MSAA on all cameras as they are spawned.
            app.add_systems(Update, disable_msaa_on_cameras);
        }

        #[cfg(not(any(target_os = "android", feature = "xr")))]
        {
            // Desktop defaults: standard shadow resolution.
            app.insert_resource(DirectionalLightShadowMap { size: 2048 });
            app.insert_resource(PointLightShadowMap { size: 1024 });
        }
    }
}

#[cfg(any(target_os = "android", feature = "xr"))]
fn disable_msaa_on_cameras(
    mut commands: Commands,
    cameras: Query<Entity, (With<Camera>, Without<Msaa>)>,
) {
    for entity in &cameras {
        commands.entity(entity).insert(Msaa::Off);
    }
}
