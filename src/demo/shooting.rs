use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

use super::shot::{ShotAssets, shot};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ShootingController>();

    app.add_systems(
        Update,
        (shoot_if_we_shooting)
            .chain()
            .run_if(resource_exists::<ShotAssets>)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ShootingController {
    ///
    pub intent_to_fire: bool,
    pub last_shot_time: f32,
    pub reload_time: f32,
    pub shot_speed: f32,
    pub target_offset: Vec2,
}

impl Default for ShootingController {
    fn default() -> Self {
        Self {
            intent_to_fire: false,
            last_shot_time: 0.0,
            reload_time: 0.5,
            shot_speed: 640.0,
            target_offset: Vec2 { x: 0.0, y: 200.0 },
        }
    }
}

fn shoot_if_we_shooting(
    time: Res<Time>,
    mut shooting_query: Query<(&mut ShootingController, &Transform)>,
    mut commands: Commands,
    shot_assets: Res<ShotAssets>,
) {
    for (mut controller, transform) in &mut shooting_query {
        if controller.intent_to_fire
            && time.elapsed_secs() - controller.last_shot_time > controller.reload_time
        {
            controller.last_shot_time = time.elapsed_secs();
            // todo: use transform and pass to new shot, also grab controller offset amount for target_offset
            commands.spawn(shot(
                controller.shot_speed,
                &shot_assets,
                Some(transform.translation.truncate() + controller.target_offset),
                transform.translation.truncate(),
            ));
        }
    }
}
