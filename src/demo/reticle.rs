use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Reticle>();

    app.register_type::<ReticleAssets>();
    app.load_resource::<ReticleAssets>();

    // TODO: add systems for:
    // - updating reticle postition based on ShootingController target
    // - update reticle visual over time
}

// TODO: Add spawner for Reticle to call in level init

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
struct Reticle {
    pub speed: f32,
    pub visual_wave_val: f32,
    pub target: Option<Vec2>,
}

impl Default for Reticle {
    fn default() -> Self {
        Self {
            speed: 50.0,
            visual_wave_val: 0.5,
            target: None,
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ReticleAssets {
    #[dependency]
    texture: Handle<Image>,
}

impl FromWorld for ReticleAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            texture: assets.load_with_settings(
                "images/reticle.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
