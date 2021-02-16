use bevy::prelude::*;

struct Position { x: i32, y: i32 }
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
        .run();
}

fn setup(cmd: &mut Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                TextSection {
                    value: "@",
                    style: TextStyle
                }
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Position { x: 40, y: 25 });

    for i in 0..10 {
        cmd.with(Position { x: i * 7, y: 20 })
    }
}

struct InputTimer(Timer);
fn input(time: Res<Time>,
         timer: ResMut<InputTimer>,
         keyboard_input: Res<Input<KeyCode>>,
         mut query: Query<(&Player, &mut Position)>) {

    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    for (_player, mut position) in query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            position -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            position += 1.0;
        }
    }
}

fn position_sync(query: Query<(&Position, &mut Transform)>) {
    for (position, transform) in query.iter() {
        transform.translation = Vec3::new(position.x * 20 as f32, position.y * 20 as f32, 0.0)
    }
}
