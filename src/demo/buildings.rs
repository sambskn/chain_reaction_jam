use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use super::explosions::Explosion;
use super::movement::MAX_X;
use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Building>();

    app.register_type::<BuildingAssets>();
    app.load_resource::<BuildingAssets>();

    app.add_systems(
        Update,
        (
            check_for_explosion_damage,
            update_building_sprite,
            check_for_game_over.run_if(in_state(Screen::Gameplay)),
        )
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

// TODO: make buildings move so they don't overlap one another??

pub fn building(
    building_assets: &BuildingAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 32, y: 64 }, 3, 1, None, None);
    let texture_atlas = texture_atlas_layouts.add(layout);
    let x_val = rand::random::<f32>() * MAX_X * 2.0 - MAX_X;
    (
        Sprite {
            image: building_assets.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x_val, -175.0, -1.0).with_scale(Vec3::splat(2.0)),
        Building {
            health: 3,
            last_damage_time: 0.0,
        },
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
struct Building {
    pub health: i32,
    pub last_damage_time: f32,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BuildingAssets {
    #[dependency]
    texture: Handle<Image>,
}

impl FromWorld for BuildingAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            texture: assets.load_with_settings(
                "images/building.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

fn update_building_sprite(
    mut query: Query<(&mut Sprite, &Building, Entity)>,
    mut commands: Commands,
) {
    for (mut sprite, building, entity) in query.iter_mut() {
        // if building health is 0, destroy the entity
        if building.health <= 0 {
            commands.entity(entity).despawn();
        }
        // if building health is less than 3, change sprite to frame 2
        if building.health < 3 {
            sprite.texture_atlas.as_mut().unwrap().index = 1;
        }
        // if building health is less than 2, change sprite to frame 3
        if building.health < 2 {
            sprite.texture_atlas.as_mut().unwrap().index = 2;
        }
    }
}

const BUILDING_RADIUS: f32 = 55.0;
const TIME_BETWEEN_DAMAGE: f32 = 2.0;

fn check_for_explosion_damage(
    explosion_query: Query<(&Transform, &Explosion)>,
    mut building_query: Query<(&Transform, &mut Building)>,
    time: Res<Time>,
) {
    for (explosion_transform, explosion) in &explosion_query {
        for (buidling_transform, mut building) in &mut building_query {
            let dist_to_explosion = explosion_transform
                .translation
                .distance(buidling_transform.translation);
            let enough_time_elapsed =
                (time.elapsed_secs() - building.last_damage_time) > TIME_BETWEEN_DAMAGE;
            if (dist_to_explosion - BUILDING_RADIUS) < explosion.radius && enough_time_elapsed {
                building.health -= 1;
                building.last_damage_time = time.elapsed_secs();
            }
        }
    }
}

fn check_for_game_over(
    building_query: Query<&Building>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let building_count = building_query.iter().count();
    if building_count == 0 {
        next_screen.set(Screen::GameOver);
    }
}
