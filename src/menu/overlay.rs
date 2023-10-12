use bevy::prelude::*;

/// Marker component for the overlay UI - This is used to destroy the UI when we
/// leave the UI state
#[derive(Component)]
pub struct Marker;

/// Create the overlay UI
pub fn create(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Will render above everything else
	commands
		.spawn((
			NodeBundle {
				style: Style {
					// Center the UI to 80% window
					top: Val::Percent(10.0),
					left: Val::Percent(10.0),
					width: Val::Percent(80.0),
					height: Val::Percent(80.0),

					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					flex_direction: FlexDirection::ColumnReverse,
					..Default::default()
				},
				border_color: BorderColor(Color::BLACK),
				background_color: BackgroundColor(Color::YELLOW),
				..Default::default()
			},
			Marker,
		))
		.with_children(|parent| {
			// Add Text screaming 'OVERLAY' in NotoSans-Light
			parent.spawn(TextBundle {
				text: Text {
					sections: vec![TextSection {
						value: "OVERLAY".to_string(),
						style: TextStyle {
							font:      asset_server.load("fonts/NotoSans-Light.ttf"),
							font_size: 100.0,
							color:     Color::BLACK,
						},
					}],
					..Default::default()
				},
				..Default::default()
			});
		});
}

/// Update the UI - Navigation, etc.
pub fn update(mut commands: Commands) {}
