#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(incomplete_features)] // stfu clippy
#![feature(adt_const_params)]

mod menu;

use bevy::{
	asset::ChangeWatcher,
	input::keyboard::KeyboardInput,
	prelude::*,
	render::{
		settings::{
			Backends,
			WgpuSettings,
		},
		RenderPlugin,
	},
};
use menu::{
	OverlayMarker,
	UiState,
};

// TODO: Remove. Replace with UiState + GameState
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
	#[default]
	Paused,
	Interaction,
	Exploration,
}

fn main() {
	let mut app = App::new();

	app.add_state::<GameState>(); // Global state - Are we in the Menu or Playing the Game?
	app.add_state::<menu::UiState>(); // UI State - What Menu pane are we in?

	#[allow(clippy::needless_update)] // stfu clippy i know what i'm doing
	let assets = AssetPlugin {
		#[cfg(feature = "debug")]
		watch_for_changes: ChangeWatcher::with_delay(std::time::Duration::from_secs(1)),
		asset_folder: "resources".to_string(),
		..Default::default()
	};

	let render = RenderPlugin {
		wgpu_settings: WgpuSettings {
			backends: Some(Backends::METAL | Backends::VULKAN), // DX12 was causing issues
			..Default::default()
		},
	};

	app.add_plugins(
		DefaultPlugins
			.set(assets)
			.set(render)
			.set(ImagePlugin::default_nearest()),
	);

	// Menu systems
	app.add_systems(Startup, startup);
	app.add_systems(Update, update);

	// Overlay
	app.add_systems(
		OnEnter(UiState::Overlay),
		menu::overlay::create.after(in_state(GameState::Paused)),
	); // Only render Overlay if we're in the Game
	app.add_systems(
		Update,
		menu::overlay::update
			.run_if(in_state(GameState::Paused).and_then(in_state(UiState::Overlay))),
	); // Tick the Overlay if it's rendered
	app.add_systems(OnExit(UiState::Overlay), menu::destroy_ui::<OverlayMarker>);

	app.run();
}

fn startup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

fn update(
	mut _commands: Commands,
	mut next_game_state: ResMut<NextState<GameState>>,
	mut next_ui_state: ResMut<NextState<UiState>>,
	mut key_event: EventReader<KeyboardInput>,
	mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
	use bevy::input::ButtonState;

	// If `O` is pressed, change to InWorld+Overlay
	// If `M` is pressed, change to Menu+Main
	// If `Escape` is pressed, quit the game
	for key in key_event.iter() {
		match key.key_code {
			Some(KeyCode::O) => {
				if key.state == ButtonState::Pressed {
					next_ui_state.set(UiState::Overlay);
					next_game_state.set(GameState::Paused);
				}
			},
			Some(KeyCode::M) => {
				if key.state == ButtonState::Pressed {
					next_ui_state.set(UiState::Main);
					next_game_state.set(GameState::Exploration);
				}
			},
			Some(KeyCode::Escape) => {
				if key.state == ButtonState::Pressed {
					app_exit_events.send(bevy::app::AppExit);
				}
			},
			_ => {},
		}
	}
}
