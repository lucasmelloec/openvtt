use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use resources::*;

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
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
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
        .insert_resource(IpAddress(String::with_capacity(25)));
    }
}

fn setup_connecting(
    mut commands: Commands,
    mut framepace: ResMut<FramepaceSettings>,
    images: Res<ImageAssets>,
    fonts: Res<FontAssets>,
) {
    framepace.limiter = Limiter::from_framerate(30.0);
    commands.spawn((Camera2dBundle::default(), StateScoped(AppState::Connecting)));
    commands
        .spawn((
            ImageBundle {
                z_index: ZIndex::Local(-1),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    //justify_content: JustifyContent::SpaceBetween,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: UiImage {
                    texture: images.background.clone(),
                    ..default()
                },
                ..default()
            },
            StateScoped(AppState::Connecting),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Enter the IP address with port",
                TextStyle {
                    font: fonts.default_font.clone(),
                    color: Color::BLACK,
                    ..default()
                },
            ));
        });
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

fn setup_connected(mut commands: Commands, images: Res<ImageAssets>) {
    commands
        .spawn((Camera2dBundle::default(), StateScoped(AppState::Connected)))
        .insert(PanCam::default());
    commands.spawn((
        SpriteBundle {
            texture: images.pig.clone(),
            ..default()
        },
        StateScoped(AppState::Connected),
    ));
}
