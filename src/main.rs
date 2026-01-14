//! {{project-name}} - Game created with bevy_map_editor
//!
//! Run with hot-reload: cargo run --features dev
//! Run for release: cargo run --release

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_map_runtime::{MapHandle, MapRuntimePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{project-name}}".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        // Add the map runtime plugin for rendering tilemaps
        .add_plugins(MapRuntimePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controls)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn((
        Camera2d,
        Transform::from_xyz(400.0, 300.0, 0.0),
    ));

    // Load the map from assets/maps/
    // Replace "your_map.map.json" with your actual map filename
    // The map will be copied here when you click "Run Game" in the editor
    commands.spawn(MapHandle(asset_server.load("maps/your_map.map.json")));

    info!("Game started!");
    info!("Controls: WASD to pan, Q/E to zoom");

    #[cfg(feature = "dev")]
    info!("Hot-reload enabled: Save your map in the editor to see changes!");
}

/// Basic camera controls: WASD to pan, Q/E to zoom
fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut projection)) = query.single_mut() else {
        return;
    };

    let speed = 300.0 * time.delta_secs();

    // WASD or Arrow keys to pan
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= speed;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed;
    }

    // Q/E to zoom
    if let Projection::Orthographic(ref mut ortho) = *projection {
        if keyboard.pressed(KeyCode::KeyQ) {
            ortho.scale *= 1.0 + time.delta_secs();
        }
        if keyboard.pressed(KeyCode::KeyE) {
            ortho.scale *= 1.0 - time.delta_secs();
        }
        ortho.scale = ortho.scale.clamp(0.25, 4.0);
    }
}
