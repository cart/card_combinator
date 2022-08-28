use std::time::Duration;

use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::game::animate::{AnimateRange, Ease};

#[derive(Component)]
pub struct PlayerCamera {
    base_speed: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self { base_speed: 4.0 }
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
    mut view_height: Local<i8>,
    mut scroll_accumulation: Local<f32>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cameras: Query<(&PlayerCamera, &mut Transform)>,
) {
    for event in mouse_wheel_events.iter() {
        match event.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                *scroll_accumulation += 20.0 * event.y.signum()
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => *scroll_accumulation += event.y,
        }
        if *scroll_accumulation >= 20.0 {
            *scroll_accumulation = 0.0;
            *view_height += 1;
        } else if *scroll_accumulation <= -20.0 {
            *scroll_accumulation = 0.0;
            *view_height -= 1;
        }

        *view_height = view_height.min(1).max(-1);
    }

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

        if direction.length() > 0.01 {
            direction = direction.normalize();
        }
        transform.translation += direction * camera.base_speed * time.delta_seconds();

        let target_z = 8.0 + *view_height as f32 * 2.0;
        let mut animation = AnimateRange::new(
            Duration::from_secs_f32(0.2),
            Ease::Linear,
            transform.translation.z..target_z,
            false,
        );
        transform.translation.z = animation.tick(time.delta());
    }
}
