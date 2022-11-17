use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
    .add_startup_system(add_people)
    .add_system(hello_world)
    .add_system(greet_people)
    .run();
}

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Caio Cesar".to_string())));
    commands.spawn((Person, Name("Castro Oliveira".to_string())));
    commands.spawn((Person, Name("Nicole Soares".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}