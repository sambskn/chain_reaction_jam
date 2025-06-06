use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{AppSystems, asset_tracking::LoadResource};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BGLayer1>();

    app.register_type::<BGAssets>();
    app.load_resource::<BGAssets>();
    app.add_systems(Update, move_layer_1_around.in_set(AppSystems::Update));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
struct BGLayer1 {
    pub speed: f32,
    pub offset: f32,
    pub max_offset: f32,
}

const BG_LAYER_1_Z: f32 = -10.0;

pub fn bg_layer_1(bg_assets: &BGAssets) -> impl Bundle {
    (
        Sprite {
            image: bg_assets.texture.clone(),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 8.0,
            },
            custom_size: Some(Vec2::splat(1200.0)),
            ..default()
        },
        Transform::from_translation(Vec2::ZERO.extend(BG_LAYER_1_Z)),
        BGLayer1 {
            speed: 10.0,
            offset: 0.0,
            max_offset: 128.0,
        },
    )
}

const BG_LAYER_2_Z: f32 = -9.0;

pub fn bg_layer_2(bg_assets: &BGAssets) -> impl Bundle {
    (
        Sprite {
            image: bg_assets.play_field_background.clone(),
            ..default()
        },
        Transform::from_translation(Vec2::ZERO.extend(BG_LAYER_2_Z)),
    )
}

const BG_LAYER_3_Z: f32 = -8.0;

pub fn bg_layer_3(bg_assets: &BGAssets) -> impl Bundle {
    (
        Sprite {
            image: bg_assets.play_field_frame.clone(),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(12.0),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 4.0,
            }),
            custom_size: Some(Vec2 { x: 760.0, y: 600.0 }),
            ..default()
        },
        Transform::from_translation(Vec2::ZERO.extend(BG_LAYER_3_Z)),
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BGAssets {
    #[dependency]
    texture: Handle<Image>,
    #[dependency]
    play_field_frame: Handle<Image>,
    #[dependency]
    play_field_background: Handle<Image>,
}

impl FromWorld for BGAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            texture: assets.load_with_settings(
                "images/tiled_bg.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            play_field_frame: assets.load_with_settings(
                "images/game_frame_1.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            play_field_background: assets.load_with_settings(
                "images/temp_bg_field.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

fn move_layer_1_around(mut layer1_query: Query<(&mut Transform, &mut BGLayer1)>, time: Res<Time>) {
    for (mut transform, mut bg_layer_1) in &mut layer1_query {
        let step = bg_layer_1.speed * time.delta_secs();
        bg_layer_1.offset = if bg_layer_1.offset + step >= bg_layer_1.max_offset {
            0.0
        } else {
            bg_layer_1.offset + step
        };
        transform.translation.x = bg_layer_1.offset;
        transform.translation.y = bg_layer_1.offset;
    }
}
