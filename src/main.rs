use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_flycam::NoCameraPlayerPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct CameraMarker;

fn setup_cam(mut commands: Commands) {
    let mut camera_2d_bundle = Camera2dBundle::default();
    // For this example, let's make the screen/window height correspond to
    // 1600.0 world units. The width will depend on the aspect ratio.
    camera_2d_bundle.projection.scaling_mode = ScalingMode::FixedVertical(720.0);
    camera_2d_bundle.transform = Transform::from_xyz(0.0, 0.0, 0.0);

    commands.spawn((camera_2d_bundle, CameraMarker));
}


fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/mouse_icon.png"),
        // sprite: Sprite::new(Vec2::new(100.0, 100.0)),
        transform: Transform::from_xyz(0.0, 0.0, 101.0),
        ..default()
    });
}

fn spawn_squares(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(mesh_assets.add(Rectangle::new(20.0, 20.0)));
    let color = Color::hsl(0.5, 0.5, 0.5);

    for x in -10..10 {
        for y in -10..10 {
            commands.spawn(MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: materials.add(color),
                transform: Transform::from_xyz(x as f32 * 30., y as f32 * 30., 100.),
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
            .insert_resource(ClearColor(Color::rgb(223., 215., 181.)))
            .add_systems(
                Startup,
                (setup_cam, spawn_player, spawn_squares, add_people).chain(),
            )
            // chain allows us to specify the order of the systems running, otherwise they run in
            // parallel, with no guaranteed order
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
            synchronous_pipeline_compilation: true, 
        }))
        .add_plugins((NoCameraPlayerPlugin, HelloPlugin))
        .run();
}
