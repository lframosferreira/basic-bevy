use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::*;
use crate::game::SimulationState;
use crate::AppState;

pub const PLAYER_SPEED: f32 = 500.0; // player movement speed
pub const PLAYER_SIZE: f32 = 64.0; // player sprite size

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]

pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement),
        )
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(
            Update,
            (
                player_movement.in_set(PlayerSystemSet::Movement),
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
                enemy_hit_player,
                player_hit_star,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        )
        .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
