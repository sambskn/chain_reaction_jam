use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, audio::sound_effect,
    demo::score::ScoreEvent,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Explosion>();
    app.register_type::<ExplosionController>();

    app.register_type::<ExplosionAssets>();
    app.load_resource::<ExplosionAssets>();

    app.add_systems(
        Update,
        (
            create_explosions,
            update_explosions,
            check_for_explosion_chain,
        )
            .chain()
            .run_if(resource_exists::<ExplosionAssets>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ExplosionController {
    pub should_explode: bool,
    pub explosion_radius: f32,
    pub hitbox_radius: f32,
    pub explosion_life_span: f32,
}

impl Default for ExplosionController {
    fn default() -> Self {
        Self {
            should_explode: false,
            explosion_radius: 100.0,
            hitbox_radius: 5.0,
            explosion_life_span: 1.0,
        }
    }
}

impl ExplosionController {
    pub fn new(
        should_explode: bool,
        explosion_radius: f32,
        hitbox_radius: f32,
        explosion_life_span: f32,
    ) -> Self {
        Self {
            should_explode,
            explosion_radius,
            hitbox_radius,
            explosion_life_span,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ExplosionAnimation {
    pub frame_time: f32,
    pub frame_index: usize,
}

impl Default for ExplosionAnimation {
    fn default() -> Self {
        Self {
            frame_time: 0.1,
            frame_index: 0,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Explosion {
    radius: f32,
    center: Vec2,
    explosion_max_life_span: f32,
    explosion_life_span_remaining: f32,
}

impl Explosion {
    pub fn new(radius: f32, center: Vec2, explosion_max_life_span: f32) -> Self {
        Self {
            radius,
            center,
            explosion_max_life_span,
            explosion_life_span_remaining: explosion_max_life_span,
        }
    }
}

pub fn explosion(
    position: Vec2,
    radius: f32,
    lifespan: f32,
    explosion_assets: &ExplosionAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let mut initial_transform = Transform::from_scale(Vec2::splat(radius / 8.0).extend(1.0));
    initial_transform.translation = position.extend(1.0);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 11, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    (
        Explosion::new(radius, position, lifespan),
        Sprite {
            image: explosion_assets.explosion.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        initial_transform,
        sound_effect(explosion_assets.boom.clone()),
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

fn create_explosions(
    mut commands: Commands,
    query: Query<(&Transform, &ExplosionController, Entity)>,
    explosion_assets: Res<ExplosionAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // loop through all entities with ExplosionController component & a Transform
    for (transform, controller, entity) in query.iter() {
        if controller.should_explode {
            // create explosion
            commands.spawn(explosion(
                transform.translation.truncate(),
                controller.explosion_radius,
                controller.explosion_life_span,
                &explosion_assets,
                &mut texture_atlas_layouts,
            ));
            // get rid of thing that exploded
            commands.entity(entity).despawn();
        }
    }
}

const NUM_FRAMES: usize = 11;

fn update_explosions(
    mut commands: Commands,
    mut query: Query<(&mut Sprite, &mut Explosion, Entity)>,
    time: Res<Time>,
) {
    for (mut sprite, mut explosion, entity) in query.iter_mut() {
        if explosion.explosion_life_span_remaining <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            explosion.explosion_life_span_remaining -= time.delta_secs();
            let sprite_frame_index = ((explosion.explosion_life_span_remaining
                / explosion.explosion_max_life_span)
                * (NUM_FRAMES - 1) as f32)
                .ceil() as usize;
            sprite.texture_atlas.as_mut().unwrap().index = sprite_frame_index;
        }
    }
}

fn check_for_explosion_chain(
    explosion_query: Query<(&Transform, &Explosion)>,
    mut can_explode_query: Query<(&Transform, &mut ExplosionController)>,
    mut commands: Commands,
) {
    for (transform, explosion) in &explosion_query {
        for (potential_explosion_transform, mut potential_explosion_controller) in
            &mut can_explode_query
        {
            if !potential_explosion_controller.should_explode {
                let distance = transform
                    .translation
                    .distance(potential_explosion_transform.translation);
                if distance <= explosion.radius + potential_explosion_controller.hitbox_radius {
                    potential_explosion_controller.should_explode = true;
                    commands.trigger(ScoreEvent { score: 1 });
                }
            }
        }
    }
}
