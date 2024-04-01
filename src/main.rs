use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn spawn_cubes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mesh = mesh_assets.add(Cuboid::new(1.0, 1.0, 1.0));
    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32 * 2., 0., z as f32 * 2.)),
                ..Default::default()
            });
        }
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // only update one person
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello, {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, (setup_cam, add_people))
            // chain allows us to specify the order of the systems running, otherwise they run in
            // parallel, with no guaranteed order
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
