use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{AppSystems, PausableSystems, asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ScoreController>();
    app.register_type::<ScoreUIAssets>();
    app.load_resource::<ScoreUIAssets>();

    app.init_resource::<Score>();

    app.add_systems(
        Update,
        (update_combo_timer)
            .chain()
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
}

#[derive(Resource)]
pub struct Score(pub u32);

impl FromWorld for Score {
    fn from_world(_world: &mut World) -> Self {
        Self(0)
    }
}

pub fn score_ui(score_ui_assets: &ScoreUIAssets) -> impl Bundle {
    let slicer = TextureSlicer {
        border: BorderRect::all(5.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    (
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            bottom: Val::Px(5.0),
            left: Val::Px(10.0),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ImageNode {
            image: score_ui_assets.frame.clone(),
            image_mode: NodeImageMode::Sliced(slicer),
            ..default()
        },
        GlobalZIndex(-1),
        Transform::from_translation(Vec2::ZERO.extend(-9.0)),
        children![
            (Text::new("Score"), TextFont::from_font_size(18.0),),
            (ScoreVal, Text::new("0"), TextFont::from_font_size(24.0),)
        ],
        Observer::new(
            |trigger: Trigger<NewScore>,
             mut score_text: Query<&mut Text, With<ScoreVal>>,
             mut score_res: ResMut<Score>| {
                for mut text in score_text.iter_mut() {
                    text.0 = format!("{}", trigger.score);
                }
                score_res.0 = trigger.score;
            },
        ),
        StateScoped(Screen::Gameplay),
    )
}

pub fn combo_ui(score_ui_assets: &ScoreUIAssets) -> impl Bundle {
    let slicer = TextureSlicer {
        border: BorderRect::all(5.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    (
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            bottom: Val::Px(5.0),
            right: Val::Px(10.0),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ImageNode {
            image: score_ui_assets.frame.clone(),
            image_mode: NodeImageMode::Sliced(slicer),
            ..default()
        },
        children![
            (Text::new("combo"), TextFont::from_font_size(18.0),),
            (ComboVal, Text::new("1x"), TextFont::from_font_size(24.0),)
        ],
        Observer::new(
            |trigger: Trigger<NewScore>, mut combo_text: Query<&mut Text, With<ComboVal>>| {
                for mut text in combo_text.iter_mut() {
                    text.0 = format!("{}x", trigger.combo);
                }
            },
        ),
        StateScoped(Screen::Gameplay),
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

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ScoreUIAssets {
    #[dependency]
    frame: Handle<Image>,
}

impl FromWorld for ScoreUIAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            frame: assets.load_with_settings(
                "images/box_frame.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
