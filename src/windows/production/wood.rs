use bevy_egui::egui::{Align, Context, Layout, Ui, Window};
use serde::{Deserialize, Serialize};

use crate::math::{float, int};
use crate::state::OpenWindows;
use crate::top_bar_button::TopBarButton;
use crate::Upgrades::{ChopOnWeekends, FindBetterTrees, SharpenAxe};
use crate::{AppWindow, GameState, Upgrade, View};

#[derive(Serialize, Deserialize, Default)]
pub struct WoodProduction;

impl AppWindow for WoodProduction {
	fn name(&self) -> &'static str {
		"Wood Production"
	}

	fn show(&mut self, ctx: &Context, state: &mut GameState, open: &mut bool) {
		Window::new(self.name())
			.open(open)
			.default_size([300., 300.])
			.resizable(false)
			.collapsible(false)
			.show(ctx, |ui| self.ui(ui, state));
	}
}

impl View for WoodProduction {
	fn ui(&mut self, ui: &mut Ui, state: &mut GameState) {
		ui.add_space(4.);

		ui.with_layout(Layout::top_down(Align::Center), |ui| {
			if ui.button("Chop wood").clicked() {
				state.wood += &state.wood_per_click;
				state.total_wood += &state.wood_per_click;
			}

			ui.add_space(4.);

			ui.label(format!("You have {} units of wood.", state.wood));
			ui.label(format!(
				"You are chopping {}w/s (wood per second).",
				state.wood_per_second
			));

			ui.add_space(4.);

			let wood_upgrades: Vec<Upgrade> = vec![
				Upgrade {
					name: "Sharpen axe",
					visible: |state| !state.upgrades.contains(&SharpenAxe),
					enabled: |state| state.wood >= int(5),
					action: |state| {
						state.upgrades.insert(SharpenAxe);
						state.wood_per_click += int(1);
						state.wood -= int(5);
					},
				},
				Upgrade {
					name: "Find better trees",
					visible: |state| {
						state.upgrades.contains(&SharpenAxe) && !state.upgrades.contains(&FindBetterTrees)
					},
					enabled: |state| state.wood >= int(5),
					action: |state| {
						state.upgrades.insert(FindBetterTrees);
						state.wood_per_click += float(0.5);
						state.wood -= int(5);
					},
				},
				Upgrade {
					name: "Chop wood on the weekends",
					visible: |state| {
						state.upgrades.contains(&SharpenAxe)
							&& state.upgrades.contains(&FindBetterTrees)
							&& !state.upgrades.contains(&ChopOnWeekends)
					},
					enabled: |state| state.wood >= int(5),
					action: |state| {
						state.upgrades.insert(ChopOnWeekends);
						state.wood_per_second += float(0.1);
						state.wood -= int(5);
					},
				},
			];

			for upgrade in wood_upgrades {
				if (upgrade.visible)(state) {
					ui.set_enabled((upgrade.enabled)(state));
					if ui.button(upgrade.name).clicked() {
						(upgrade.action)(state);
					}
				}
			}
		});

		ui.add_space(4.);
	}
}

impl WoodProduction {
	pub fn spawn_button(open_windows: &mut OpenWindows, ui: &mut Ui) {
		let default = Self::default();
		let button = TopBarButton {
			label: "Wood".to_string(),
			value: default.name().to_string(),
		};
		button.ui(ui, open_windows);
	}
}
