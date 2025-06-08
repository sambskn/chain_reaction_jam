//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::{
        player::{PlayerAssets, player},
        score::{self, ScoreUIAssets, combo_ui, score_ui},
    },
    screens::Screen,
};

use super::background::{BGAssets, bg_layer_1, bg_layer_2, bg_layer_3};
use super::buildings::{BuildingAssets, building};
use super::enemies::EnemyController;
use super::reticle::{ReticleAssets, reticle};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/track_2.mp3"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    reticle_assets: Res<ReticleAssets>,
    bg_assets: Res<BGAssets>,
    score_ui_assets: Res<ScoreUIAssets>,
    building_assets: Res<BuildingAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player(400.0, &player_assets),
            EnemyController::default(),
            score::score_controller(),
            (reticle(&reticle_assets), Name::new("Reticle"),),
            bg_layer_1(&bg_assets),
            bg_layer_2(&bg_assets),
            bg_layer_3(&bg_assets),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),
            building(&building_assets, &mut texture_atlas_layouts),
        ],
    ));
    commands.spawn(score_ui(&score_ui_assets));
    commands.spawn(combo_ui(&score_ui_assets));
}
