//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::movement::{MovementController, ScreenWrap},
};

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
}

/// The player character.
pub fn player(max_speed: f32, player_assets: &PlayerAssets) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.cannon.clone(),
            ..default()
        },
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
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
    for mut controller in &mut controller_query {
        controller.intent = intent;
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
