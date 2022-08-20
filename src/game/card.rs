use std::time::Duration;

use bevy::prelude::{shape::Quad, *};
use bevy_rapier3d::prelude::*;

use crate::game::animate::{AnimateRange, Ease};
use crate::game::camera::PlayerCamera;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCard>()
            .init_resource::<HoverPoint>()
            .add_system_to_stage(CoreStage::PostUpdate, on_spawn_card)
            .add_system(select_card.after(crate::game::camera::move_camera))
            .add_system(move_cards.after(select_card));
    }
}

#[derive(Component, Default)]
pub struct Card {
    pub animations: Animations,
    pub color: CardColor,
    pub z: usize,
}

impl Card {
    const ASPECT_RATIO: f32 = 50.0 / 60.0;
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum SelectedCard {
    Some(Entity),
    #[default]
    None,
}

impl SelectedCard {
    fn is_selected(self, entity: Entity) -> bool {
        match self {
            SelectedCard::Some(e) => e == entity,
            SelectedCard::None => false,
        }
    }
}

#[derive(Default)]
pub enum HoverPoint {
    Some(Vec3),
    #[default]
    None,
}

#[derive(Default, Copy, Clone)]
pub enum CardColor {
    #[default]
    Gray,
    Blue,
    Yellow,
}

impl CardColor {
    fn get_color(self) -> Color {
        match self {
            CardColor::Gray => Color::rgb(0.4, 0.4, 0.4),
            CardColor::Blue => Color::rgb(0.4, 0.4, 1.0),
            CardColor::Yellow => Color::rgb(0.7, 0.7, 0.4),
        }
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibiltiy: ComputedVisibility,
}

impl Default for CardBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(Card::ASPECT_RATIO / 2.0, 1.0 / 2.0, 0.2),
            card: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibiltiy: Default::default(),
        }
    }
}

fn on_spawn_card(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    cards: Query<(Entity, &Card), Added<Card>>,
) {
    for (entity, card) in &cards {
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: card.color.get_color(),
                    base_color_texture: Some(asset_server.load("card_base.png")),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                }),
                mesh: meshes.add(
                    Quad {
                        size: Vec2::new(Card::ASPECT_RATIO, 1.0),
                        ..default()
                    }
                    .into(),
                ),
                ..default()
            });
        });
    }
}

fn move_cards(
    time: Res<Time>,
    selected: Res<SelectedCard>,
    hover_point: Res<HoverPoint>,
    mut cards: Query<(Entity, &mut Card, &mut Transform)>,
) {
    for (entity, mut card, mut transform) in &mut cards {
        let mut z_offset = 0.0;
        if selected.is_selected(entity) {
            z_offset += card.animations.select.tick(time.delta());
            if let HoverPoint::Some(hover_point) = *hover_point {
                transform.translation.x = hover_point.x;
                transform.translation.y = hover_point.y;
            }
        } else {
            z_offset += card.animations.deselect.tick(time.delta());
        }
        transform.translation.z = z_offset;
    }
}

fn select_card(
    context: Res<RapierContext>,
    windows: Res<Windows>,
    mut selected_card: ResMut<SelectedCard>,
    mut hover_point: ResMut<HoverPoint>,
    mouse: Res<Input<MouseButton>>,
    mut cards: Query<&mut Card>,
    cameras: Query<(&Camera, &Transform), With<PlayerCamera>>,
) {
    let window = windows.primary();
    if let Some(mut cursor) = window.cursor_position() {
        let (camera, camera_transform) = cameras.single();

        let view = camera_transform.compute_matrix();

        let (viewport_min, viewport_max) = camera.logical_viewport_rect().unwrap();
        let screen_size = camera.logical_target_size().unwrap();
        let viewport_size = viewport_max - viewport_min;
        let adj_cursor_pos = cursor - Vec2::new(viewport_min.x, screen_size.y - viewport_max.y);

        let projection = camera.projection_matrix();
        let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
        let near_ndc = projection.project_point3(Vec3::Z).z;
        let cursor_ndc = (adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE;
        let ndc_to_world: Mat4 = view * projection.inverse();
        let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
        let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
        let direction = far - near;

        let denom = Vec3::Z.dot(direction);
        if denom.abs() > 0.0001 {
            let t = (Vec3::ZERO - near).dot(Vec3::Z) / denom;
            if t >= 0.0 {
                *hover_point = HoverPoint::Some(near + direction * t);
            } else {
                *hover_point = HoverPoint::None;
            }
        } else {
            *hover_point = HoverPoint::None;
        }

        if mouse.just_pressed(MouseButton::Left) {
            let result = context.cast_ray(near, direction, 50.0, true, QueryFilter::new());

            if let Some((entity, toi)) = result {
                let mut card = cards.get_mut(entity).unwrap();
                card.animations.select.reset();
                *selected_card = SelectedCard::Some(entity);
                return;
            }
        }
    }

    if mouse.just_released(MouseButton::Left) {
        if let SelectedCard::Some(entity) = *selected_card {
            let mut card = cards.get_mut(entity).unwrap();
            card.animations.deselect.reset();
            *selected_card = SelectedCard::None;
        }
    }
}

pub struct Animations {
    select: AnimateRange,
    deselect: AnimateRange,
}

impl Default for Animations {
    fn default() -> Self {
        Self {
            select: AnimateRange::new(Duration::from_secs_f32(0.2), Ease::Linear, 0.0..0.5, false),
            deselect: AnimateRange::new(
                Duration::from_secs_f32(0.2),
                Ease::Linear,
                0.5..0.0,
                false,
            ),
        }
    }
}
