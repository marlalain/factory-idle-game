use bevy_egui::egui::{Context, Ui};

use crate::state::OpenWindows;
use crate::GameState;

pub mod production;

pub trait View {
	fn ui(&mut self, ui: &mut Ui, state: &mut GameState);
}

pub trait AppWindow {
	fn name(&self) -> &'static str;

	fn name_as_string(&self) -> String {
		self.name().to_string()
	}

	fn is_enabled(&self, _ctx: &Context) -> bool {
		true
	}

	fn show(&mut self, ctx: &Context, state: &mut GameState, is_open: &mut bool);
}

pub fn set_open(open_windows: &mut OpenWindows, key: String, is_open: bool) {
	if is_open {
		if !open_windows.contains(&key) {
			open_windows.insert(key);
		}
	} else {
		open_windows.remove(&key);
	}
}
