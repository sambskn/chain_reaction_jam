//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod buildings;
mod enemies;
mod explosions;
mod floating_text;
pub mod level;
mod movement;
pub mod player;
mod reticle;
pub mod score;
mod shooting;
mod shot;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        floating_text::plugin,
        movement::plugin,
        player::plugin,
        shot::plugin,
        shooting::plugin,
        explosions::plugin,
        enemies::plugin,
        reticle::plugin,
        score::plugin,
        buildings::plugin,
    ));
}
