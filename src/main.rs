use ascii::AsciiPlugin;
use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

mod ascii;
mod debug;
mod player;

use debug::DebugPlugin;
use player::PlayerPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    let height = 900.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Bevy Tutorial".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .run()
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
