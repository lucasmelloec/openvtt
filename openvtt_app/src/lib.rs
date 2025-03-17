use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct VttPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum AppState {
    #[default]
    Wip,
}

impl Plugin for VttPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(
                WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Escape)),
            )
            .add_plugins(FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    text_color: Color::srgb(0.0, 1.0, 0.0),
                    enabled: false,
                    ..default()
                },
            })
            .init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_systems(Startup, setup)
            .add_systems(Update, toggle_fps_overlay);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn toggle_fps_overlay(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::Escape) {
        overlay.enabled = !overlay.enabled;
    }
}
