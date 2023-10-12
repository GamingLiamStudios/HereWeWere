#![windows_subsystem = "windows"]
#![allow(incomplete_features)] // stfu clippy
#![feature(adt_const_params)]

mod menu;

use bevy::{
	asset::ChangeWatcher,
	prelude::*,
};
use menu::UiState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
	#[default]
	Menu,
	InWorld,
}

fn main() {
	let mut app = App::new();

	app.add_state::<AppState>(); // Global state - Are we in the Menu or Playing the Game?
	app.add_state::<menu::UiState>(); // UI State - What Menu pane are we in?

	#[allow(clippy::needless_update)] // stfu clippy i know what i'm doing
	let assets = AssetPlugin {
		#[cfg(feature = "debug")]
		watch_for_changes: ChangeWatcher::with_delay(std::time::Duration::from_secs(1)),
		asset_folder: "resources".to_string(),
		..Default::default()
	};

	app.add_plugins(DefaultPlugins.set(assets));

	// Menu systems
	app.add_systems(Startup, (startup, menu::startup));

	// Overlay

	app.add_systems(
		Update,
		menu::overlay.run_if(in_state(AppState::InWorld).and_then(in_state(UiState::Overlay))),
	); // Run the overlay system only if we're also in the InWorld state

	app.run();
}

fn startup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}
