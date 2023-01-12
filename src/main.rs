use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::{RichText, Color32};

fn main() {
    App::new()
        .init_resource::<DialogState>()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rs Novel".to_string(),
                width: 1600.0,
                height: 900.0,
                resizable: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        //.add_system(ui_example)
        .add_system(dialog_box)
        .run();
}

#[derive(Resource, Default)]
struct DialogState {
    name: usize,
    dialog: usize,
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>, mut dialog_state: ResMut<DialogState>) {
    dialog_state.name = 0;
    dialog_state.dialog = 0;
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: _asset_server.load("backgrounds/scene01.png"),
        ..default()
    });
}

fn dialog_box(keyboard_input: Res<Input<KeyCode>>, mut egui_context: ResMut<EguiContext>, mut dialog_state: ResMut<DialogState>) {
    let name = ["Иван", "Анатолий", "Василий"];
    let dialog = ["Заходит как-то улитка в бар...", "...и говорит бармену:", "Можно мне виски?"];
    let my_frame = egui::containers::Frame {
        inner_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
        rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
        fill: Color32::from_black_alpha(224),
        ..Default::default()
    };
    if keyboard_input.just_pressed(KeyCode::Space) && dialog_state.name < name.len() - 1 && dialog_state.dialog < dialog.len() - 1 {
        dialog_state.name += 1;
        dialog_state.dialog += 1;
    }
    egui::TopBottomPanel::bottom("").frame(my_frame).show(egui_context.ctx_mut(), |ui| {
        ui.label(RichText::new(name[dialog_state.name]).heading().size(30.0).color(Color32::from_white_alpha(192)));
        ui.separator();
        ui.label(RichText::new(dialog[dialog_state.dialog]).size(20.0).color(Color32::from_white_alpha(192)));
    });
}