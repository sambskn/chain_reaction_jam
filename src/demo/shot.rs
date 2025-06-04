use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use super::explosions::{ExplosionAssets, explosion};
use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource, audio::sound_effect};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Shot>();

    app.register_type::<ShotAssets>();
    app.load_resource::<ShotAssets>();

    app.add_systems(
        Update,
        (update_shot_position, shot_end_of_life)
            .chain()
            .run_if(resource_exists::<ExplosionAssets>)
            .run_if(resource_exists::<ShotAssets>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
struct Shot {
    pub speed: f32,
    pub target: Option<Vec2>,
    pub initial_location: Vec2,
}

pub fn shot(
    speed: f32,
    shot_assets: &ShotAssets,
    target: Option<Vec2>,
    initial_location: Vec2,
) -> impl Bundle {
    (
        Name::new("Shot"),
        Shot {
            speed,
            target,
            initial_location,
        },
        Transform::from_translation(initial_location.extend(0.0)),
        Sprite {
            image: shot_assets.texture.clone(),
            ..default()
        },
        sound_effect(shot_assets.bang.clone()),
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ShotAssets {
    #[dependency]
    texture: Handle<Image>,
    #[dependency]
    pub bang: Handle<AudioSource>,
}

impl FromWorld for ShotAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            texture: assets.load_with_settings(
                "images/shot.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            bang: assets.load("audio/sound_effects/bass_two_note.mp3"),
        }
    }
}

const SLOWDOWN_THRESHOLD_DISTANCE: f32 = 10.0;
const SLOWDOWN_FACTOR: f32 = 0.5;

fn update_shot_position(mut query: Query<(&mut Transform, &mut Shot)>, time: Res<Time>) {
    for (mut transform, mut shot) in query.iter_mut() {
        if let Some(target) = shot.target {
            let direction = target - transform.translation.xy();
            let distance = direction.length();
            if distance < 0.1 {
                // Shot has reached its target
                shot.target = None;
            } else {
                let direction = direction.normalize();
                shot.speed = if distance < SLOWDOWN_THRESHOLD_DISTANCE {
                    shot.speed * SLOWDOWN_FACTOR
                } else {
                    shot.speed
                };
                transform.translation.x += direction.x * shot.speed * time.delta_secs();
                transform.translation.y += direction.y * shot.speed * time.delta_secs();
                // modify scale based on distance from initial location
                let initial_target_distance = shot.initial_location.distance(target);
                let percent_travelled =
                    (initial_target_distance - distance) / initial_target_distance;
                // use a quadratic function to scale the shot, with a desired starting scale and ending scale
                // TODO: maybe cubic would look better with the peak scale happening a little after the initial location
                let initial_scale = 2.0;
                let final_scale = 0.5;
                let scale_factor =
                    initial_scale + (final_scale - initial_scale) * percent_travelled.powi(2);
                transform.scale = Vec3::splat(scale_factor);
            }
        }
    }
}

fn shot_end_of_life(
    query: Query<(&Shot, &Transform, Entity)>,
    mut commands: Commands,
    explosion_assets: Res<ExplosionAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (shot, transform, entity) in query.iter() {
        if shot.target.is_none() {
            // Shot has reached its target
            commands.spawn(explosion(
                transform.translation.truncate(),
                16.0,
                1.0,
                &explosion_assets,
                &mut texture_atlas_layouts,
            ));
            commands.entity(entity).despawn();
        }
    }
}
