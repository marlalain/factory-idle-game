use std::collections::BTreeSet;

use bevy::prelude::Resource;
use bevy::utils::HashSet;
use rust_decimal::Decimal;

use crate::math::int;

pub type OpenWindows = BTreeSet<String>;

#[derive(Default, Resource)]
pub struct UIState {
	pub open_windows: OpenWindows,
}

type ValueType = Decimal;

#[derive(Resource)]
pub struct GameState {
	pub total_wood: ValueType,
	pub wood: ValueType,
	pub wood_per_click: ValueType,
	pub wood_per_second: ValueType,
	pub upgrades: HashSet<Upgrades>,
}

impl Default for GameState {
	fn default() -> Self {
		Self {
			total_wood: int(0),
			wood: int(0),
			wood_per_click: int(1),
			wood_per_second: int(0),
			upgrades: HashSet::<Upgrades>::default(),
		}
	}
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Upgrades {
	SharpenAxe,
	FindBetterTrees,
	ChopOnWeekends,
}

pub struct Upgrade {
	pub name: &'static str,
	pub visible: fn(&GameState) -> bool,
	pub enabled: fn(&GameState) -> bool,
	pub action: fn(&mut GameState),
}
