//! This example shows demonstrates how to setup a basic health bar with the [`StatusBarWidget`].

use bevy::{
    math::map_range,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    widget::WidgetPlugin, //{StatusBarInner, StatusBarWidget, }
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let health_bar_background: Color = Color::rgba_u8(92, 1, 1, 255);
    let health_bar_foreground: Color = Color::rgba_u8(224, 20, 20, 255);
    let text_color1: Color = Color::rgba_u8(250, 192, 192, 255);
    let text_color2: Color = Color::rgba_u8(225, 240, 238, 255);

    // spawn world camera
    commands.spawn(Camera2dBundle::default());

    // spawn player to take damage and heal (modified from [`2d_shapes`] example)
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}
