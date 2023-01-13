use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::{RichText, Color32};

fn main() {
    App::new()
        .init_resource::<GameState>()
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
struct GameState {
    name: usize,
    dialog: usize,
    image: usize
}

fn setup(mut commands: Commands, mut game_state: ResMut<GameState>, _asset_server: Res<AssetServer>) {
    game_state.name = 0;
    game_state.dialog = 0;
    game_state.image = 0;
    let image = ["backgrounds/scene01.png", "backgrounds/scene02.png", "backgrounds/scene03.png"];
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: _asset_server.load(image[game_state.image]),
        ..default()
    });
}

fn dialog_box(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, mut egui_context: ResMut<EguiContext>, mut game_state: ResMut<GameState>, _asset_server: Res<AssetServer>, ) {
    let name = ["Иван", "Анатолий", "Василий", "Иван", "Анатолий", "Василий", "Иван", "Анатолий"];
    let dialog = ["Заходит как-то улитка в бар...", "...и говорит бармену:", "\"Можно мне виски?\"", "А бармен ей отвечает:", "\"Мы не продаём выпивку улиткам\"", "...и вышвыривает её за дверь.", "Проходят дни, недели, улитка возвращается в бар, и говорит бармену:", "\"А нафига ты это сделал?\""];
    let image = ["backgrounds/scene01.png", "backgrounds/scene02.png", "backgrounds/scene03.png", "backgrounds/scene04.png", "backgrounds/scene05.png", "backgrounds/scene06.png", "backgrounds/scene07.png", "backgrounds/scene08.png"];

    let my_frame = egui::containers::Frame {
        inner_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
        rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
        fill: Color32::from_black_alpha(224),
        ..Default::default()
    };

    if keyboard_input.just_pressed(KeyCode::Space) && game_state.name < name.len() - 1 && game_state.dialog < dialog.len() - 1 {
        game_state.name += 1;
        game_state.dialog += 1;
        game_state.image += 1;
        commands.spawn(SpriteBundle {
            texture: _asset_server.load(image[game_state.image]),
            transform: Transform::from_xyz(0., 0., game_state.image as f32),
            ..default()
        });
    }

    egui::TopBottomPanel::bottom("").frame(my_frame).show(egui_context.ctx_mut(), |ui| {
        ui.label(RichText::new(name[game_state.name]).heading().size(30.0).color(Color32::from_white_alpha(192)));
        ui.separator();
        ui.label(RichText::new(dialog[game_state.dialog]).size(20.0).color(Color32::from_white_alpha(192)));
    });
}