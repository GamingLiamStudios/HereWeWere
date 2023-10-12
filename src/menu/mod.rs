use std::marker::ConstParamTy;

use bevy::prelude::*;

pub mod overlay;
pub use overlay::Marker as OverlayMarker;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, ConstParamTy)]
pub enum UiState {
	#[default]
	Main,
	Settings,
	MusicBox,
	Credits,
	Overlay,
}

pub fn destroy_ui<T>(
	mut commands: Commands,
	mut query: Query<(Entity, &T), With<T>>,
) where
	T: Component,
{
	for (entity, _) in query.iter_mut() {
		commands.entity(entity).despawn_recursive();
	}
}
