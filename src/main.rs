use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break // only update one person
        }
    }

}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello, {}!", name.0);
    }
}


fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        // chain allows us to specify the order of the systems running, otherwise they run in
        // parallel, with no guaranteed order
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
