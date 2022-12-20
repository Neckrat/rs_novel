use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::{RichText, Color32};

fn main() {
    App::new()
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

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: _asset_server.load("backgrounds/scene01.png"),
        ..default()
    });
}

fn dialog_box(keyboard_input: Res<Input<KeyCode>>, mut egui_context: ResMut<EguiContext>) {
    let mut name = ["Иван", "Анатолий", "Василий"];
    let mut dialog = ["Заходит как-то улитка в бар...", "...и говорит бармену:", "Можно мне виски?"];
    let my_frame = egui::containers::Frame {
        inner_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
        rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
        fill: Color32::from_black_alpha(224),
        ..Default::default()
    };
    egui::TopBottomPanel::bottom("").frame(my_frame).show(egui_context.ctx_mut(), |ui| {
        ui.label(RichText::new(name[0]).heading().size(30.0).color(Color32::from_white_alpha(192)));
        ui.separator();
        ui.label(RichText::new(dialog[0]).size(20.0).color(Color32::from_white_alpha(192)));
    });

}
