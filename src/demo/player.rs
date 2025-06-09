//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, demo::movement::MovementController,
};

use super::shooting::ShootingController;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );

    // Record spacebar as shooting input.
    app.add_systems(
        Update,
        record_player_shooting_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

const PLAYER_Y: f32 = -220.0;

/// The player character.
pub fn player(max_speed: f32, player_assets: &PlayerAssets) -> impl Bundle {
    let mut initial_transform = Transform::from_scale(Vec2::splat(2.0).extend(1.0));
    initial_transform.translation = Vec3 {
        x: 0.0,
        y: PLAYER_Y,
        z: 0.0,
    };
    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.cannon.clone(),
            ..default()
        },
        initial_transform,
        MovementController {
            max_speed,
            ..default()
        },
        ShootingController::default(),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

const PLAYER_RETICLE_Y_SPEED: f32 = 10.0;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<(&mut MovementController, &mut ShootingController), With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for (mut movement_controller, mut shooting_controller) in &mut controller_query {
        movement_controller.intent = Vec2 {
            x: intent.x,
            y: 0.0,
        };
        shooting_controller.target_offset += Vec2::new(0.0, intent.y * PLAYER_RETICLE_Y_SPEED);
        shooting_controller.intent_to_fire = input.just_pressed(KeyCode::Space);
    }
}

fn record_player_shooting_input(
    input: Res<ButtonInput<KeyCode>>,
    mut shooting_query: Query<&mut ShootingController, With<Player>>,
) {
    for mut controller in &mut shooting_query {
        controller.intent_to_fire = input.just_pressed(KeyCode::Space);
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    cannon: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            cannon: assets.load_with_settings(
                "images/cannon.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/sh1.mp3"),
                assets.load("audio/sound_effects/sh2.mp3"),
                assets.load("audio/sound_effects/sh3.mp3"),
                assets.load("audio/sound_effects/sh4.mp3"),
            ],
        }
    }
}
