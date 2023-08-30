use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use components::*;
use resources::*;

mod components;
mod online;
mod resources;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum AppState {
    #[default]
    AssetLoading,
    Matchmaking,
    InGame,
}

pub fn run() {
    App::new()
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::Matchmaking),
        )
        .add_collection_to_loading_state::<_, ImageAssets>(AppState::AssetLoading)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Open VTT".into(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(FramepacePlugin)
        .add_plugins(PanCamPlugin)
        .add_plugins(online::OnlinePlugin)
        .add_systems(OnEnter(AppState::Matchmaking), setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut framepace: ResMut<FramepaceSettings>,
    assets: Res<ImageAssets>,
) {
    commands
        .spawn((Camera2dBundle::default(), MainCamera))
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Middle],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.1,
            max_scale: Some(10.0),
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        });
    framepace.limiter = Limiter::from_framerate(30.0);
    commands.spawn(SpriteBundle {
        texture: assets.player_token.clone(),
        ..default()
    });
}
