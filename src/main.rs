use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rs Novel".to_string(),
                width: 1600.0,
                height: 900.0,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_system(dialog_text_system)
        .run();
}

#[derive(Component)]
struct DialogText;

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: _asset_server.load("textures/scene01.png"),
        ..default()
    });

    commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(70.0), Val::Px(150.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(50.0),
                    right: Val::Percent(15.0),
                    left: Val::Percent(15.0),
                    ..default()
                },
                border: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.6).into(),
            ..default()
        }
    )
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "ыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыыы",
                TextStyle { font: _asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 30.0, color: Color::rgba(1.0, 1.0, 1.0, 0.0) }, // TODO: create dialogs in the dialog box
                
            ),
            DialogText
        ));
    });
}

fn dialog_text_system(keyboard_input: Res<Input<KeyCode>>, /*time: Res<Time>,*/ mut query: Query<&mut Text, With<DialogText>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        for mut text in &mut query {
            // let seconds = time.elapsed_seconds(); // здесь должна быть имбовая реализация fade-in эффекта, но что-то пошло не так
            text.sections[0].style.color = Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
            fade_in_text(&text.sections[0].value);
        }
    }
}

fn fade_in_text(text: &str) {
    let timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    if timer.just_finished() {
        for char in text.chars() {
            print!("{}", char)
        }
    }
}
