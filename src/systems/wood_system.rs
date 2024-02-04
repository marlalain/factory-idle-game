use std::ops::DerefMut;

use bevy::prelude::ResMut;

use crate::GameState;

pub fn wood_per_second_system(mut state: ResMut<GameState>) {
	let state = state.deref_mut();
	state.wood += &state.wood_per_second;
	state.total_wood += &state.wood_per_second;
}
