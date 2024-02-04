use bevy::prelude::ResMut;

use crate::GameState;

pub fn wood_per_second_system(mut state: ResMut<GameState>) {
	state.wood += state.wood_per_second;
	state.total_wood += state.wood_per_second;
}
