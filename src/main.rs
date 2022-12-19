use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(dialog_text_system)
        .run();
}

#[derive(Component)]
struct DialogText;

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(90.0), Val::Px(150.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(50.0), 
                    right: Val::Percent(5.0),
                    left: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.6).into(),
            ..default()
        }
    )
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "ы",
                TextStyle { font: _asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 30.0, color: Color::rgba(1.0, 1.0, 1.0, 0.0) }, // TODO: create dialogs in the dialog box
                
            ),
            DialogText
        ));
    });
}

fn dialog_text_system(keyboard_input: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut Text, With<DialogText>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        for mut text in &mut query {
            let seconds = time.elapsed_seconds(); // здесь должна быть имбовая реализация fade-in эффекта, но что-то пошло не так

            text.sections[0].style.color = Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
        }
    }
}