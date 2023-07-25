use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const PLAYER_SPEED: f32 = 500.0; // player movement speed
pub const PLAYER_SIZE: f32 = 64.0; // player sprite size

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star);
    }
}