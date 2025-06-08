use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use super::{
    explosions::{ExplosionAssets, ExplosionController},
    movement::MovementController,
};
use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, demo::movement::MAX_X,
    screens::Screen,
};

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

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
struct Enemy {
    pub speed: f32,
    pub last_movement_time: f32,
    pub time_between_movements: f32,
    pub target_index: usize,
    pub target_locs: Vec<Vec2>,
}

const OFFSCREEN_ABOVE: f32 = 500.0;
const MIN_ENEMY_Y_BELOW: f32 = -125.0;

pub fn create_target_locs() -> Vec<Vec2> {
    let mut locs = vec![];
    // create an x val that's somewhere in between max and min x vals
    let x_val = rand::random::<f32>() * MAX_X * 2.0 - MAX_X;
    // start at the top of the screen (out of view)
    locs.push(Vec2::new(x_val, OFFSCREEN_ABOVE));
    let x_val_2 = rand::random::<f32>() * MAX_X * 2.0 - MAX_X;
    locs.push(Vec2 {
        x: x_val_2,
        y: (OFFSCREEN_ABOVE + MIN_ENEMY_Y_BELOW) / 2.0,
    });
    // go down to the bottom of the screen (in view at about player level)
    locs.push(Vec2::new(x_val, MIN_ENEMY_Y_BELOW));
    locs
}

// todo:: pass initial transfrom and use to spawn shot
pub fn enemy(speed: f32, enemy_assets: &EnemyAssets) -> impl Bundle {
    let target_locs = create_target_locs();
    let initial_location = target_locs[0];
    (
        Name::new("Enemy"),
        Enemy {
            speed,
            last_movement_time: 0.0,
            time_between_movements: 1.0,
            target_index: 0,
            target_locs,
        },
        Transform::from_translation(initial_location.extend(0.0)).with_scale(Vec3::splat(4.0)),
        Sprite {
            image: enemy_assets.texture.clone(),
            ..default()
        },
        MovementController::default(),
        ExplosionController::new(false, 32.0, 32.0, 1.5),
        StateScoped(Screen::Gameplay),
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

const SLOWDOWN_DISTANCE: f32 = 200.0;
const EXPLOSION_DISTANCE: f32 = 10.0;

fn update_enemy_movement_intent(
    mut query: Query<(
        &mut MovementController,
        &mut ExplosionController,
        &mut Enemy,
        &Transform,
    )>,
) {
    for (mut movement_controller, mut explosion_controller, mut enemy, transform) in
        query.iter_mut()
    {
        // check if we're close to the current target location
        let diff = transform
            .translation
            .truncate()
            .distance(enemy.target_locs[enemy.target_index]);
        // iterate over the target locations if we are close to the current one
        if diff < 10.0 {
            enemy.target_index = (enemy.target_index + 1) % enemy.target_locs.len();
        }
        // set intent based on diff to current target
        let target = enemy.target_locs[enemy.target_index];
        let direction = (target - transform.translation.truncate()).normalize();
        movement_controller.intent = direction;
        // modify speed based on distance to min y val
        let distance_to_min_y = transform.translation.y - MIN_ENEMY_Y_BELOW;
        // make enemy explode if it got real close
        if distance_to_min_y < EXPLOSION_DISTANCE {
            explosion_controller.should_explode = true;
        }
        if distance_to_min_y < SLOWDOWN_DISTANCE {
            let speed_modifier = distance_to_min_y / SLOWDOWN_DISTANCE;
            movement_controller.max_speed = enemy.speed * speed_modifier;
        } else {
            movement_controller.max_speed = enemy.speed;
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyController {
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
            commands.spawn(enemy(75.0, &enemy_assets));
        }
    }
}
