use std::collections::BTreeSet;

use bevy::prelude::Resource;

pub type OpenWindows = BTreeSet<String>;

#[derive(Resource)]
pub struct UIState {
	pub open_windows: OpenWindows,
}

impl Default for UIState {
	fn default() -> Self {
		Self {
			open_windows: OpenWindows::default(),
		}
	}
}
