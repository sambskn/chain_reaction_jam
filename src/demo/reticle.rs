use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use super::shooting::ShootingController;
use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Reticle>();

    app.register_type::<ReticleAssets>();
    app.load_resource::<ReticleAssets>();

    // TODO: add systems for:
    // - update reticle visual over time
    app.add_systems(
        Update,
        (update_reticle_visual_wave_val_timer, update_reticle_target)
            .chain()
            .run_if(resource_exists::<ReticleAssets>)
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        update_reticle_position
            .run_if(resource_exists::<ReticleAssets>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub fn reticle(reticle_assets: &ReticleAssets) -> impl Bundle {
    (
        Sprite {
            image: reticle_assets.texture.clone(),
            ..default()
        },
        Transform::default().with_scale(Vec3::splat(2.0)),
        Reticle::default(),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
struct Reticle {
    pub move_speed: f32,
    pub visual_speed: f32,
    pub visual_wave_val: f32,
    pub target: Option<Vec2>,
}

impl Default for Reticle {
    fn default() -> Self {
        Self {
            move_speed: 450.0,
            visual_speed: 5.0,
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

fn update_reticle_visual_wave_val_timer(mut reticle_query: Query<&mut Reticle>, time: Res<Time>) {
    for mut reticle in reticle_query.iter_mut() {
        reticle.visual_wave_val += time.delta_secs() * reticle.visual_speed;
        while reticle.visual_wave_val > 1.0 {
            reticle.visual_wave_val -= 1.0;
        }
    }
}

fn update_reticle_target(
    mut reticle_query: Query<&mut Reticle>,
    shooting_controller_query: Query<(&Transform, &ShootingController)>,
) {
    for mut reticle in reticle_query.iter_mut() {
        if let Some((transform, shooting_controller)) = shooting_controller_query.iter().next() {
            reticle.target =
                Some(transform.translation.truncate() + shooting_controller.target_offset);
        }
    }
}

fn update_reticle_position(mut reticle_query: Query<(&mut Transform, &Reticle)>, time: Res<Time>) {
    for (mut transform, reticle) in reticle_query.iter_mut() {
        if let Some(target) = reticle.target {
            let direction = target - transform.translation.truncate();
            let distance = direction.length();
            if distance > 0.0 {
                let step = time.delta_secs() * reticle.move_speed;
                let step = step.min(distance);
                transform.translation += (direction.normalize() * step).extend(1.0);
            }
        }
    }
}
