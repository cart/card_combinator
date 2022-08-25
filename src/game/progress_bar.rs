use bevy::prelude::{shape::Quad, *};

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_spawn_progress_bar)
            .add_system(set_progress_bar);
    }
}

#[derive(Component, Default)]
pub struct ProgressBar {
    pub current: f32,
    pub total: f32,
    pub padding: f32,
    pub width: f32,
    pub height: f32,
}

impl ProgressBar {
    const Z: f32 = 0.1;
    pub fn finished(&self) -> bool {
        self.current >= self.total
    }

    pub fn add(&mut self, amount: f32) {
        self.current += amount;
        self.current = self.current.min(self.total);
    }
}

#[derive(Component, Default)]
pub struct ProgressBarStatus;

#[derive(Bundle, Default)]
pub struct ProgressBarBundle {
    pub progress_bar: ProgressBar,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibiltiy: ComputedVisibility,
}

fn on_spawn_progress_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    bars: Query<(Entity, &ProgressBar), Added<ProgressBar>>,
) {
    for (entity, bar) in &bars {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb_u8(50, 50, 50),
                    depth_bias: 100.0,
                    unlit: true,
                    ..default()
                }),
                mesh: meshes.add(
                    Quad {
                        size: Vec2::new(bar.width, bar.height),
                        ..default()
                    }
                    .into(),
                ),
                transform: Transform::from_xyz(0.0, 0.0, ProgressBar::Z),
                ..default()
            });
            parent
                .spawn_bundle(PbrBundle {
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb_u8(150, 150, 150),
                        depth_bias: 102.0,
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, ProgressBar::Z + 0.005),
                    mesh: meshes.add(
                        Quad {
                            size: Vec2::new(
                                bar.width - bar.padding * 2.0,
                                bar.height - bar.padding * 2.0,
                            ),
                            ..default()
                        }
                        .into(),
                    ),
                    ..default()
                })
                .insert(ProgressBarStatus);
        });
    }
}

fn set_progress_bar(
    mut statuses: Query<(&Parent, &mut Transform), With<ProgressBarStatus>>,
    bars: Query<&ProgressBar>,
) {
    for (parent, mut transform) in &mut statuses {
        if let Ok(bar) = bars.get(parent.get()) {
            let percent = bar.current / bar.total;
            transform.scale.x = percent;
            transform.translation.x = -(bar.width - bar.padding * 2.0) * (1.0 - percent) / 2.0;
        }
    }
}
