use bevy::prelude::*;

struct Position { x: f32, y: f32 }
struct Renderable {
    glyph: String,
    fg: Color,
    bg: Color
}
struct Player;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(position_sync.system())
        .add_system(input.system())
        .add_resource(InputTimer(Timer::from_seconds(1.0, true)))
        .run();
}

fn setup(cmd: &mut Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            text: Text {
                value: "@".to_string(),
                font: asset_server.load("MajorMonoDisplay-Regular.ttf"),
                style: TextStyle {
                    color: Color::WHITE,
                    font_size: 60.0,
                    ..Default::default()
                }
            },
            ..Default::default()
        })
        .with(Position { x: 40.0, y: 25.0 });

    for i in 0..10 {
        cmd.spawn(TextBundle {
            text: Text {
                value: "%".to_string(),
                font: asset_server.load("MajorMonoDisplay-Regular.ttf"),
                style: TextStyle {
                    color: Color::BLUE,
                    font_size: 60.0,
                    ..Default::default()
                }
            },
            ..Default::default()
        }).with(Position { x: i as f32 * 7.0, y: 20.0 });
    }
}

struct InputTimer(Timer);
fn input(time: Res<Time>,
         mut timer: ResMut<InputTimer>,
         keyboard_input: Res<Input<KeyCode>>,
         mut query: Query<(&Player, &mut Position)>) {

    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    for (_player, mut position) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            position.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            position.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            position.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            position.y -= 1.0;
        }
    }
}

fn position_sync(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.x * 20 as f32, position.y * 20 as f32, 0.0)
    }
}
