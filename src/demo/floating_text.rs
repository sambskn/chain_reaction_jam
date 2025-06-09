use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{AppSystems, PausableSystems, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<FloatingText>();
    app.add_event::<NewText>();
    app.add_systems(
        Update,
        (handle_new_text_event, update_floating_text)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Event)]
pub struct NewText(pub String, pub f32, pub f32); // text, x, y

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FloatingText(pub f32); // text, lifetime

const FLOATING_TEXT_FONT_SIZE: f32 = 16.0;
const FLOATING_TEXT_MOVE_SPEED: f32 = 1.5;
const FLOATING_TEXT_Z: f32 = 6.0;
const FLOATING_TEXT_LIFETIME_MS: f32 = 2000.0;

pub fn handle_new_text_event(mut ev_new_text: EventReader<NewText>, mut commands: Commands) {
    for ev in ev_new_text.read() {
        commands.spawn((
            Text2d::new(ev.0.to_string()),
            TextFont {
                font_size: FLOATING_TEXT_FONT_SIZE,
                ..default()
            },
            StateScoped(Screen::Gameplay),
            Anchor::BottomCenter,
            Transform::from_xyz(ev.1, ev.2, FLOATING_TEXT_Z),
            TextColor(Color::Srgba(Srgba::rgb(1.0, 1.0, 1.0))),
            FloatingText(FLOATING_TEXT_LIFETIME_MS),
        ));
        commands.spawn((
            Text2d::new(ev.0.to_string()),
            TextFont {
                font_size: FLOATING_TEXT_FONT_SIZE,
                ..default()
            },
            Anchor::BottomCenter,
            StateScoped(Screen::Gameplay),
            Transform::from_xyz(ev.1 + 2.0, ev.2 - 2.0, FLOATING_TEXT_Z - 0.1),
            TextColor(Color::Srgba(Srgba::rgb(0.0, 0.0, 0.0))),
            FloatingText(FLOATING_TEXT_LIFETIME_MS),
        ));
    }
}

pub fn update_floating_text(
    mut floating_text_query: Query<(Entity, &mut Transform, &mut FloatingText, &mut TextColor)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // loop through text, update transform pos, kill if lifetime is gone
    for (entity, mut transform, mut floating_text, mut text_color) in &mut floating_text_query {
        floating_text.0 += -(time.delta().as_millis() as f32);
        if floating_text.0 <= 0.0 {
            commands.get_entity(entity).unwrap().despawn();
        } else {
            // update pos of text (and opacity??)
            transform.translation +=
                Vec3::new(0.0, FLOATING_TEXT_MOVE_SPEED * time.delta_secs(), 0.0);
            let current_color = text_color.0.to_srgba();
            text_color.0 = Color::srgba(
                current_color.red,
                current_color.green,
                current_color.blue,
                1.0,
            );
        }
    }
}
