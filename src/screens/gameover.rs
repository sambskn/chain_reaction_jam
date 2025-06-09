use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::demo::score::Score;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), spawn_game_over);

    app.add_systems(
        Update,
        update_game_over_timer.run_if(in_state(Screen::GameOver)),
    );
    // Toggle pause on key press.
    app.add_systems(
        Update,
        ((go_to_main_menu)
            .run_if(in_state(Screen::GameOver).and(input_just_pressed(KeyCode::Space))),),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
struct GameOverTimer(pub f32);

fn spawn_game_over(mut commands: Commands, current_score: Res<Score>) {
    commands.spawn((
        StateScoped(Screen::GameOver),
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameOverTimer(0.0),
        children![
            (
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                Text::new("Game Over"),
                TextFont::from_font_size(22.0)
            ),
            (
                Node {
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                Text::new(format!("{} dang points,  wow", current_score.0)),
                TextFont::from_font_size(20.0)
            ),
            (
                Node {
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                Text::new("Press space to get some more"),
                TextFont::from_font_size(16.0)
            )
        ],
    ));
}

fn update_game_over_timer(mut query: Query<&mut GameOverTimer>, time: Res<Time>) {
    for mut timer in query.iter_mut() {
        timer.0 += time.delta_secs();
    }
}

fn go_to_main_menu(
    mut next_screen: ResMut<NextState<Screen>>,
    query: Query<&GameOverTimer>,
    mut score_res: ResMut<Score>,
) {
    let game_over_timer = query.single().unwrap();
    if game_over_timer.0 >= 2.0 {
        next_screen.set(Screen::Title);
        score_res.0 = 0;
    }
}
