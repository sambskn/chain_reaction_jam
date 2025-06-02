use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use super::{
    explosions::{ExplosionAssets, ExplosionController},
    movement::MovementController,
};
use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Enemy>();

    app.register_type::<EnemyAssets>();
    app.load_resource::<EnemyAssets>();

    app.register_type::<EnemyController>();

    app.add_systems(
        Update,
        (update_enemy_movement_intent)
            .chain()
            .run_if(resource_exists::<ExplosionAssets>)
            .run_if(resource_exists::<EnemyAssets>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        (update_enemy_controller)
            .chain()
            .run_if(resource_exists::<ExplosionAssets>)
            .run_if(resource_exists::<EnemyAssets>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
struct Enemy {
    pub speed: f32,
    pub last_movement_time: f32,
    pub time_between_movements: f32,
}

// todo:: pass initial transfrom and use to spawn shot
pub fn enemy(speed: f32, enemy_assets: &EnemyAssets, initial_location: Vec2) -> impl Bundle {
    (
        Name::new("Enemy"),
        Enemy {
            speed,
            last_movement_time: 0.0,
            time_between_movements: 1.0,
        },
        Transform::from_translation(initial_location.extend(0.0)).with_scale(Vec3::splat(4.0)),
        Sprite {
            image: enemy_assets.texture.clone(),
            ..default()
        },
        MovementController::default(),
        ExplosionController::new(false, 32.0, 32.0, 1.5),
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct EnemyAssets {
    #[dependency]
    texture: Handle<Image>,
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            texture: assets.load_with_settings(
                "images/dude.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

fn update_enemy_movement_intent(
    mut query: Query<(&mut MovementController, &mut Enemy)>,
    time: Res<Time>,
) {
    for (mut movement_controller, mut enemy) in query.iter_mut() {
        movement_controller.intent = Vec2::ZERO;
        movement_controller.max_speed = enemy.speed;
        if enemy.last_movement_time >= enemy.time_between_movements {
            movement_controller.intent = Vec2::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
            );
            enemy.last_movement_time = 0.0;
        }
        enemy.last_movement_time += time.delta_secs();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyController {
    ///
    pub max_num_enemies: usize,
}

impl Default for EnemyController {
    fn default() -> Self {
        Self { max_num_enemies: 5 }
    }
}

fn update_enemy_controller(
    enemy_query: Query<&Enemy>,
    controller_query: Query<&EnemyController>,
    enemy_assets: Res<EnemyAssets>,
    mut commands: Commands,
) {
    let enemy_count = enemy_query.iter().count();
    for controller in controller_query.iter() {
        if enemy_count < controller.max_num_enemies {
            commands.spawn(enemy(
                800.0,
                &enemy_assets,
                Vec2 {
                    x: rand::random::<f32>() * 1000.0 - 500.0,
                    y: 100.0 + rand::random::<f32>() * 50.0,
                },
            ));
        }
    }
}
