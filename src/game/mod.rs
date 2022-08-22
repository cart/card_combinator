pub mod animate;
pub mod camera;
pub mod card;
pub mod progress_bar;
pub mod tile;

use std::f32::consts::PI;

use self::camera::PlayerCameraPlugin;
use crate::game::{
    card::{Card, CardBundle, CardColor, CardPlugin},
    progress_bar::{ProgressBar, ProgressBarBundle, ProgressBarPlugin},
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CardPlugin)
            .add_plugin(PlayerCameraPlugin)
            .add_plugin(ProgressBarPlugin)
            .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    asset_server.watch_for_changes().unwrap();
    // stage
    // commands.spawn_bundle(PbrBundle {
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::rgb(0.6, 0.6, 0.6),
    //         unlit: true,
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(0.0, 0.0, -0.1),
    //     mesh: meshes.add(
    //         shape::Quad {
    //             size: Vec2::new(8.0, 5.0),
    //             ..default()
    //         }
    //         .into(),
    //     ),
    //     ..default()
    // });

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(-1.0, 0.6, 0.0),
        card: Card {
            color: CardColor::Gray,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(0.0, 0.6, 0.0),
        card: Card {
            color: CardColor::Gray,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(1.0, 0.6, 0.0),
        card: Card {
            color: CardColor::Gray,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(-0.5, -0.6, 0.0),
        card: Card {
            color: CardColor::Blue,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(CardBundle {
        transform: Transform::from_xyz(0.5, -0.6, 0.0),
        card: Card {
            color: CardColor::Yellow,
            ..default()
        },
        ..default()
    });

    let tile_mesh = meshes.add(
        shape::Quad {
            size: Vec2::new(3.0, 3.0),
            ..default()
        }
        .into(),
    );

    let tile_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("tile.png")),
        base_color: Color::rgb_u8(90, 127, 90),
        unlit: true,
        depth_bias: -10.0,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(-3.0, 0.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(-3.0, -3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(3.0, -3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(3.0, 3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, -3.0, 0.0),
        material: tile_material.clone(),
        mesh: tile_mesh.clone(),
        ..default()
    });
    commands.spawn_bundle(ProgressBarBundle {
        progress_bar: ProgressBar {
            current: 60.0,
            total: 100.0,
            width: 2.0,
            height: 0.15,
            padding: 0.05,
        },
        ..default()
    });

    // 3d tiles
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     scene: asset_server.load("tile_base.glb#Scene0"),
    //     ..default()
    // });

    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform::from_xyz(3.0, 0.0, 0.0),
    //     scene: asset_server.load("tile_base.glb#Scene0"),
    //     ..default()
    // });

    // commands.spawn_bundle(DirectionalLightBundle {
    //     transform: Transform::from_rotation(Quat::from_euler(
    //         EulerRot::XYZ,
    //         2. * PI * 1.98,
    //         11.8,
    //         0.0,
    //     )),
    //     directional_light: DirectionalLight {
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     ..default()
    // });
}
