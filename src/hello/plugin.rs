use bevy::prelude::*;

struct Person;
struct Name(String);

pub struct HelloPlugin;

fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zyna Nieves".to_string())));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>,
                mut timer: ResMut<GreetTimer>,
                query: Query<&Name, With<Person>>) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            .add_system(greet_people.system())
            .add_resource(GreetTimer(Timer::from_seconds(2.0, true)));
    }
}
