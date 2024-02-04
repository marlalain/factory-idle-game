use bevy::prelude::*;
use bevy_egui::egui::*;
use bevy_egui::{EguiContexts, EguiPlugin};

pub use buttons::*;
pub use state::*;
pub use windows::*;

use crate::production::wood::WoodProduction;

pub mod state;

pub mod buttons;
pub mod windows;

// TODO Use trunk instead
// TODO Hot-reload
pub fn main() {
	App::new()
		.init_resource::<UIState>()
		.add_plugins(DefaultPlugins)
		.add_plugins(EguiPlugin)
		.add_systems(Update, top_bottom_panel)
		.add_systems(Update, setup_windows)
		.run();
}

fn top_bottom_panel(mut ui_state: ResMut<UIState>, mut contexts: EguiContexts) {
	TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
		menu::bar(ui, |ui| {
			global_dark_light_mode_buttons(ui);

			ui.menu_button("Production", |ui| {
				WoodProduction::spawn_button(&mut ui_state.open_windows, ui);
			})
		})
	});

	CentralPanel::default().show(contexts.ctx_mut(), |_ui| {});
}

fn setup_windows(mut ui_state: ResMut<UIState>, mut contexts: EguiContexts) {
	let mut windows: Vec<Box<dyn AppWindow>> = vec![Box::<WoodProduction>::default()];

	for window in &mut windows {
		let mut is_open = ui_state.open_windows.contains(window.name());
		window.show(contexts.ctx_mut(), &mut is_open);
		set_open(&mut ui_state.open_windows, window.name_as_string(), is_open);
	}
}
