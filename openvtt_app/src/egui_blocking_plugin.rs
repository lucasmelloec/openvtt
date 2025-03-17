use bevy::prelude::*;

use bevy_egui::EguiContexts;

pub struct EguiBlockingPlugin;

#[derive(Default, Resource)]

pub struct EguiBlockInputState {
    pub wants_keyboard_input: bool,

    pub wants_pointer_input: bool,
}

impl Plugin for EguiBlockingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiBlockInputState>()
            .add_systems(PostUpdate, egui_wants_input);
    }
}

fn egui_wants_input(mut state: ResMut<EguiBlockInputState>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    state.wants_keyboard_input = ctx.wants_keyboard_input();

    state.wants_pointer_input = ctx.wants_pointer_input();
}
