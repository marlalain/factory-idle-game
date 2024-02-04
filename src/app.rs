use std::collections::BTreeSet;

use egui::TopBottomPanel;

use crate::wood::WoodProduction;
use crate::{windows, AppWindow};

pub type OpenWindows = BTreeSet<String>;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
	// Example stuff:
	label: String,

	#[serde(skip)]
	windows: Vec<Box<dyn AppWindow>>,

	open_windows: OpenWindows,

	#[serde(skip)] // This how you opt-out of serialization of a field
	value: f32,
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			open_windows: BTreeSet::new(),
			windows: vec![Box::<WoodProduction>::default()],
			label: "Hello World!".to_owned(),
			value: 2.7,
		}
	}
}

impl TemplateApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}

		Default::default()
	}
}

impl eframe::App for TemplateApp {
	/// Called each time the UI needs repainting, which may be many times per second.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
		// For inspiration and more examples, go to https://emilk.github.io/egui

		TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:

			for window in &mut self.windows {
				let mut is_open = self.open_windows.contains(window.name());
				window.show(ctx, &mut is_open);
				windows::set_open(&mut self.open_windows, window.name(), is_open);
			}

			egui::menu::bar(ui, |ui| {
				egui::widgets::global_dark_light_mode_buttons(ui);

				ui.menu_button("Production", |ui| {
					WoodProduction::spawn_button(&mut self.open_windows, ui);
				})
			});
		});
	}

	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}
}
