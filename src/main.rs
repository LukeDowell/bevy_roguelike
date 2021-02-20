use bevy::prelude::*;
use rand::*;

mod state;

use state::*;
use rand::distributions::Uniform;

fn main() {
    let game_settings = GameSettings {
        tile_size: 20.0,
        map_height: 60,
        map_width: 35
    };

    let window_descriptor = WindowDescriptor {
        title: "Rusty Roguelike".to_string(),
        width: game_settings.tile_size * game_settings.map_width as f32,
        height: game_settings.tile_size * game_settings.map_height as f32,
        ..Default::default()
    };

    App::build()
        .insert_resource(game_settings)
        .insert_resource(window_descriptor)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(input.system())
        .add_system(sync_logical_position.system())
        .insert_resource(InputTimer::default())
        .run();
}

fn setup(cmd: &mut Commands, asset_server: Res<AssetServer>, game_settings: Res<GameSettings>) {
    cmd.spawn(OrthographicCameraBundle::new_2d())
        .spawn(Text2dBundle {
            text: Text::with_section(
                "@",
                TextStyle {
                    font: asset_server.load("MajorMonoDisplay-Regular.ttf"),
                    font_size: game_settings.tile_size,
                    color: Color::WHITE
                },
                Default::default()
            ),
            ..Default::default()
        })
        .with(Position { x: 0., y: 0. })
        .with(Player);

    let map = new_map(game_settings.map_width, game_settings.map_height);
}

fn sync_logical_position(mut query: Query<(&mut Transform, &Position)>, game_settings: Res<GameSettings>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = game_settings.tile_size * position.x;
        transform.translation.y = game_settings.tile_size * position.y;
    }
}

pub fn input(mut timer: ResMut<InputTimer>,
             keyboard_input: Res<Input<KeyCode>>,
             mut query: Query<(&Player, &mut Position)>) {

    let seconds = timer.last_moved.elapsed().as_secs() as f64;
    let sub_seconds = timer.last_moved.elapsed().subsec_nanos() as f64 * 1e-9;
    if seconds + sub_seconds <= timer.move_cooldown {
        return
    }

    for (_player, mut position) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            position.x -= 1.0;
            timer.last_moved = Instant::now();
        }

        if keyboard_input.pressed(KeyCode::Right) {
            position.x += 1.0;
            timer.last_moved = Instant::now();
        }

        if keyboard_input.pressed(KeyCode::Up) {
            position.y += 1.0;
            timer.last_moved = Instant::now();
        }

        if keyboard_input.pressed(KeyCode::Down) {
            position.y -= 1.0;
            timer.last_moved = Instant::now();
        }
    }
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map(map_width: i32, map_height: i32) -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    // Walls
    for x in 0..map_width + 1 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..map_height + 1 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = rand::thread_rng();

    for _i in 0..400 {
        let x = rng.sample(Uniform::from(1..map_width));
        let y = rng.sample(Uniform::from(1..map_height));
        let idx = xy_idx(x, y);
        if idx != xy_idx(0, 0) {
            map[idx] = TileType::Wall;
        }
    }

    map
}
