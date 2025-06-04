use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ScoreController>();

    app.add_systems(
        Update,
        (update_combo_timer)
            .chain()
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
}

pub fn score_ui() -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        children![
            (Text::new("Score"), TextFont::from_font_size(24.0),),
            (ScoreVal, Text::new("0"), TextFont::from_font_size(48.0),)
        ],
        Observer::new(
            |trigger: Trigger<NewScore>, mut score_text: Query<&mut Text, With<ScoreVal>>| {
                for mut text in score_text.iter_mut() {
                    text.0 = format!("{}", trigger.score);
                }
            },
        ),
    )
}

pub fn combo_ui() -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        children![
            (Text::new("combo"), TextFont::from_font_size(24.0),),
            (ComboVal, Text::new("1x"), TextFont::from_font_size(48.0),)
        ],
        Observer::new(
            |trigger: Trigger<NewScore>, mut combo_text: Query<&mut Text, With<ComboVal>>| {
                for mut text in combo_text.iter_mut() {
                    text.0 = format!("{}x", trigger.combo);
                }
            },
        ),
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScoreVal;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComboVal;

pub fn score_controller() -> impl Bundle {
    (
        ScoreController {
            score: 0,
            combo: 0,
            time_since_last_score: 0.0,
            combo_window: 1.0,
        },
        Observer::new(
            |trigger: Trigger<ScoreEvent>,
             mut score_controller: Query<&mut ScoreController>,
             mut commands: Commands| {
                for mut controller in score_controller.iter_mut() {
                    controller.score += trigger.score * controller.combo;
                    if controller.time_since_last_score < controller.combo_window {
                        controller.combo += 1;
                    } else {
                        controller.combo = 1;
                    }
                    controller.time_since_last_score = 0.0;
                    commands.trigger(NewScore {
                        score: controller.score,
                        combo: controller.combo,
                    });
                }
            },
        ),
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScoreController {
    pub score: u32,
    pub combo: u32,
    pub time_since_last_score: f32,
    pub combo_window: f32,
}

fn update_combo_timer(
    mut query: Query<&mut ScoreController>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for mut controller in query.iter_mut() {
        controller.time_since_last_score += time.delta_secs();
        if controller.time_since_last_score > controller.combo_window {
            controller.combo = 1;
            commands.trigger(NewScore {
                score: controller.score,
                combo: controller.combo,
            });
        }
    }
}

#[derive(Event)]
pub struct ScoreEvent {
    pub score: u32,
}

#[derive(Event)]
pub struct NewScore {
    pub combo: u32,
    pub score: u32,
}
