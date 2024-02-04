use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_egui::egui::{global_dark_light_mode_switch, menu, CentralPanel, TopBottomPanel};
use bevy_egui::{EguiContexts, EguiPlugin};

pub use buttons::*;
pub use state::*;
pub use windows::*;

use crate::production::wood::WoodProduction;
use crate::systems::wood_system::wood_per_second_system;

pub mod state;

pub mod buttons;
mod math;
mod systems;
pub mod windows;

// TODO Use trunk instead
// TODO Hot-reload
pub fn main() {
	App::new()
		.init_resource::<UIState>()
		.init_resource::<GameState>()
		.add_plugins(DefaultPlugins)
		.add_plugins(EguiPlugin)
		.add_systems(Update, top_bottom_panel)
		.add_systems(Update, setup_windows)
		.add_systems(
			Update,
			wood_per_second_system.run_if(on_timer(Duration::from_secs(1))),
		)
		.run();
}

fn top_bottom_panel(mut ui_state: ResMut<UIState>, mut contexts: EguiContexts) {
	TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
		menu::bar(ui, |ui| {
			global_dark_light_mode_switch(ui);

			ui.separator();

			ui.menu_button("Production", |ui| {
				WoodProduction::spawn_button(&mut ui_state.open_windows, ui);
			})
		})
	});

	CentralPanel::default().show(contexts.ctx_mut(), |_ui| {});
}

fn setup_windows(
	mut ui_state: ResMut<UIState>,
	mut state: ResMut<GameState>,
	mut contexts: EguiContexts,
) {
	let mut windows: Vec<Box<dyn AppWindow>> = vec![Box::<WoodProduction>::default()];

	for window in &mut windows {
		let mut is_open = ui_state.open_windows.contains(window.name());
		window.show(contexts.ctx_mut(), &mut state, &mut is_open);
		set_open(&mut ui_state.open_windows, window.name_as_string(), is_open);
	}
}
