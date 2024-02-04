use bevy_egui::egui::{Context, Ui, Window};
use serde::{Deserialize, Serialize};

use crate::state::OpenWindows;
use crate::top_bar_button::TopBarButton;
use crate::{AppWindow, View};

#[derive(Serialize, Deserialize, Default)]
pub struct WoodProduction;

impl AppWindow for WoodProduction {
	fn name(&self) -> &'static str {
		"Wood Production"
	}

	fn show(&mut self, ctx: &Context, open: &mut bool) {
		Window::new(self.name())
			.open(open)
			.default_size([300., 300.])
			.show(ctx, |ui| self.ui(ui));
	}
}

impl View for WoodProduction {
	fn ui(&mut self, ui: &mut Ui) {
		ui.label("Hello!!");
	}
}

impl WoodProduction {
	pub fn spawn_button(open_windows: &mut OpenWindows, ui: &mut Ui) {
		let default = Self::default();
		let button = TopBarButton {
			label: "Wood".to_owned(),
			value: default.name().to_owned(),
		};
		button.ui(ui, open_windows);
	}
}
