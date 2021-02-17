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
        .add_system(input.system())
        .add_system(sync_logical_position.system())
        .insert_resource(InputTimer(Timer::from_seconds(0.5, true)))
        .run();
}

fn setup(cmd: &mut Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(OrthographicCameraBundle::new_2d())
        .spawn(Text2dBundle {
            text: Text::with_section(
                "@",
                TextStyle {
                    font: asset_server.load("MajorMonoDisplay-Regular.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE
                },
                Default::default()
            ),
            ..Default::default()
        })
        .with(Position { x: 2.0, y: 3.0 })
        .with(Player);

    for i in 0..10 {
        cmd.spawn(Text2dBundle {
            text: Text::with_section(
                "$",
                TextStyle {
                    font: asset_server.load("MajorMonoDisplay-Regular.ttf"),
                    font_size: 32.0,
                    color: Color::BLUE
                },
                Default::default()
            ),
            transform: Transform {
                translation: Vec3::new(i as f32 * 32.0, 10.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }).with(Position { x: i as f32 , y: 3.0 });
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
            println!("left!");
            position.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            println!("right!");
            position.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            println!("up!");
            position.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            println!("down!");
            position.y -= 1.0;
        }
    }
}

fn sync_logical_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = 32.0 * position.x;
        transform.translation.y = 32.0 * position.y;
    }
}
