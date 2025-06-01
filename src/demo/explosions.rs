use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Explosion>();
    app.register_type::<ExplosionController>();

    app.register_type::<ExplosionAssets>();
    app.load_resource::<ExplosionAssets>();

    app.add_systems(
        Update,
        (update_explosion)
            .chain()
            .run_if(resource_exists::<ExplosionAssets>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ExplosionController {
    ///
    pub is_exploding: bool,
    pub explosion_radius: f32,
    pub hitbox_radius: f32,
    pub frame_time: f32,
    pub frame_index: usize,
}

impl Default for ExplosionController {
    fn default() -> Self {
        Self {
            is_exploding: false,
            explosion_radius: 100.0,
            hitbox_radius: 5.0,
            frame_time: 0.1,
            frame_index: 0,
        }
    }
}

impl ExplosionController {
    pub fn explode(&mut self) {
        self.is_exploding = true;
        self.frame_index = 0;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Explosion;

pub fn explosion(
    exploding: bool,
    position: Vec2,
    explosion_assets: &ExplosionAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let mut initial_transform = Transform::from_scale(Vec2::splat(2.0).extend(1.0));
    initial_transform.translation = position.extend(1.0);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 11, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mut explosion_controller = ExplosionController::default();
    if exploding {
        explosion_controller.explode();
    }
    (
        Name::new("Explosion"),
        Explosion,
        explosion_controller,
        Sprite {
            image: explosion_assets.explosion.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        initial_transform,
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ExplosionAssets {
    #[dependency]
    explosion: Handle<Image>,
    #[dependency]
    pub boom: Handle<AudioSource>,
}

impl FromWorld for ExplosionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            explosion: assets.load_with_settings(
                "images/explode.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            boom: assets.load("audio/sound_effects/sh1.mp3"),
        }
    }
}

fn update_explosion(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ExplosionController, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut controller, mut sprite) in query.iter_mut() {
        if controller.is_exploding {
            if controller.frame_time <= 0.0 {
                controller.frame_time = 0.1;
                controller.frame_index += 1;
                if controller.frame_index >= 11 {
                    controller.frame_index = 0;
                    controller.is_exploding = false;
                    commands.entity(entity).despawn();
                }
            } else {
                controller.frame_time -= time.delta_secs();
            }
            if sprite.texture_atlas.is_some() {
                sprite.texture_atlas.as_mut().unwrap().index = controller.frame_index;
            }
        }
    }
}
