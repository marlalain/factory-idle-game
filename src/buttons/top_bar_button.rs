use bevy_egui::egui::Ui;

use crate::state::OpenWindows;

pub struct TopBarButton {
	pub(crate) label: String,
	pub(crate) value: String,
}

impl TopBarButton {
	pub fn ui(&self, ui: &mut Ui, open_windows: &mut OpenWindows) {
		ui.set_enabled(!open_windows.contains(&self.value));
		if ui.button(&self.label).clicked() {
			open_windows.insert(self.value.to_owned());
			ui.close_menu()
		}
	}
}
