use std::marker::ConstParamTy;

use bevy::prelude::*;

use crate::AppState;

mod overlay;
pub use overlay::update as overlay;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, ConstParamTy)]
pub enum UiState {
	#[default]
	Main,
	Settings,
	MusicBox,
	Credits,
	Overlay,
}

#[derive(Resource)]
struct UiFont(Handle<Font>);

/// Load commonly used resources for the UI
pub fn startup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.insert_resource(UiFont(asset_server.load("fonts/NotoSans-Light.ttf")));
}
