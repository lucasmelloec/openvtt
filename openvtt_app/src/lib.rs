use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_pancam::{PanCam, PanCamPlugin};
use egui_blocking_plugin::{EguiBlockInputState, EguiBlockingPlugin};
use resources::*;

mod egui_blocking_plugin;
mod resources;

pub struct VttPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    AssetLoading,
    Connecting,
    Connected,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum ConnectedState {
    #[default]
    InMenu,
    InCampaign,
    InSession,
}

#[derive(Resource)]
struct IpAddress(String);

impl Plugin for VttPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin)
            .add_plugins(EguiBlockingPlugin)
            .add_plugins(
                bevy_inspector_egui::quick::WorldInspectorPlugin::new()
                    .run_if(input_toggle_active(false, KeyCode::Escape)),
            )
            .add_plugins(FramepacePlugin)
            .add_plugins(PanCamPlugin)
            .init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading)
                    .continue_to_state(AppState::Connecting)
                    .load_collection::<ImageAssets>()
                    .load_collection::<FontAssets>(),
            )
            .add_systems(OnEnter(AppState::Connecting), setup_connecting)
            .add_systems(Update, ui_system.run_if(in_state(AppState::Connecting)))
            .add_systems(OnEnter(AppState::Connected), setup_connected)
            .add_systems(
                Update,
                camera_blocking.run_if(in_state(AppState::Connected)),
            )
            .insert_resource(IpAddress(String::with_capacity(25)));
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    mut ip_address: ResMut<IpAddress>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    egui::Area::new(egui::Id::new("Ip Address"))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 30.0))
        .show(contexts.ctx_mut(), |ui| {
            let text_edit = ui.add(
                egui::TextEdit::singleline(&mut ip_address.0).hint_text("Input the IP Address"),
            );
            text_edit.request_focus();
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let AppState::Connecting = state.get() {
                    next_state.set(AppState::Connected);
                }
            }
        });
}

fn setup_connecting(
    mut commands: Commands,
    mut framepace: ResMut<FramepaceSettings>,
    images: Res<ImageAssets>,
    fonts: Res<FontAssets>,
) {
    framepace.limiter = Limiter::from_framerate(30.0);
    commands.spawn((Camera2d, StateScoped(AppState::Connecting)));
    commands
        .spawn((
            ImageNode {
                image: images.background.clone(),
                ..default()
            },
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                //justify_content: JustifyContent::SpaceBetween,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ZIndex(-1),
            StateScoped(AppState::Connecting),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Enter the IP address with port"),
                TextFont {
                    font: fonts.default_font.clone(),
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}

fn setup_connected(mut commands: Commands, images: Res<ImageAssets>) {
    commands
        .spawn((Camera2d, StateScoped(AppState::Connected)))
        .insert(PanCam::default());
    commands.spawn((
        Sprite::from_image(images.pig.clone()),
        StateScoped(AppState::Connected),
    ));
}

fn camera_blocking(
    mut pancams: Query<&mut PanCam>,
    egui_block_input_state: Res<EguiBlockInputState>,
) {
    for mut pancam in &mut pancams.iter_mut() {
        pancam.enabled = true;
        if egui_block_input_state.wants_pointer_input {
            pancam.enabled = false;
        }
    }
}
