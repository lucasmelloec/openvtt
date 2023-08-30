use bevy::prelude::*;
use openvtt_app::VttPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Open VTT".into(),
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(VttPlugin)
        .run();
}
