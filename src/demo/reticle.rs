use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use super::shooting::ShootingController;
use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

const RETICLE_Z: f32 = 10.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Reticle>();

    app.register_type::<ReticleAssets>();
    app.load_resource::<ReticleAssets>();

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
        (update_reticle_position, update_reticle_visual)
            .chain()
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
        Transform::default()
            .with_scale(Vec3::splat(2.0))
            .with_translation(Vec2::ZERO.extend(RETICLE_Z)),
        Reticle::default(),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Reticle {
    pub move_speed: f32,
    pub visual_speed: f32,
    pub visual_wave_val: f32,
    pub target: Option<Vec2>,
}

impl Default for Reticle {
    fn default() -> Self {
        Self {
            move_speed: 450.0,
            visual_speed: 8.0,
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
                "images/reticle2.png", //imo better than reticle.png, gonna leave both in assets for now
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
        reticle.visual_wave_val = (time.elapsed_secs() * reticle.visual_speed).sin() * 0.5 + 0.5;
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
                transform.translation += (direction.normalize() * step).extend(0.0);
            }
        }
    }
}

fn update_reticle_visual(mut reticle_query: Query<(&Reticle, &mut Sprite)>) {
    for (reticle, mut sprite) in reticle_query.iter_mut() {
        // adjust hue of sprite across color spectrum based on visual_wave_val (between 0.0 and 1.0)
        // (sprite is grayscale)
        sprite.color = Color::Hsla(Hsla {
            hue: reticle.visual_wave_val * 255.0,
            saturation: 0.9,
            lightness: 0.5,
            alpha: 1.0,
        });
    }
}
