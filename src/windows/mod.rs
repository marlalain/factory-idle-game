use egui::{Context, Ui};

pub use production::*;

use crate::app::OpenWindows;

pub mod production;

pub trait View {
	fn ui(&mut self, ui: &mut Ui);
}

pub trait AppWindow {
	fn name(&self) -> &'static str;

	fn is_enabled(&self, _ctx: &Context) -> bool {
		true
	}

	fn show(&mut self, ctx: &Context, open: &mut bool);
}

pub fn set_open(open_windows: &mut OpenWindows, key: &'static str, is_open: bool) {
	if is_open {
		if !open_windows.contains(key) {
			open_windows.insert(key.to_owned());
		}
	} else {
		open_windows.remove(key);
	}
}
