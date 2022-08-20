use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera {
    base_speed: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self { base_speed: 1.0 }
    }
}
pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera).add_system(move_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -1.5, 8.0),
                rotation: Quat::from_rotation_x(0.2),
                ..default()
            },
            ..default()
        })
        .insert(PlayerCamera::default());
}

pub fn move_camera(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut cameras: Query<(&PlayerCamera, &mut Transform)>,
) {
    for (camera, mut transform) in &mut cameras {
        let mut direction = Vec3::ZERO;
        if input.any_pressed([KeyCode::A, KeyCode::Left]) {
            direction.x -= 1.0;
        }
        if input.any_pressed([KeyCode::D, KeyCode::Right]) {
            direction.x += 1.0;
        }
        if input.any_pressed([KeyCode::W, KeyCode::Up]) {
            direction.y += 1.0;
        }
        if input.any_pressed([KeyCode::S, KeyCode::Down]) {
            direction.y -= 1.0;
        }
        transform.translation += direction * camera.base_speed * time.delta_seconds();
    }
}
