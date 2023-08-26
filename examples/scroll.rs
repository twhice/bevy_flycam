use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_flycam::prelude::*;

// From bevy examples:
// https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum ScrollType {
    #[default]
    MovementSpeed,
    Zoom,
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        //NoCameraPlayerPlugin as we provide the camera
        .add_plugins(NoCameraPlayerPlugin)
        // Setting initial state
        .add_state::<ScrollType>()
        .add_systems(Startup, setup)
        .add_systems(Update, switch_scroll_type)
        .add_systems(Update, scroll)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    },));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // add camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FpsCamera {
            ..Default::default()
        },
    ));

    info!("Press 'Z' to switch between Movement Speed and Zoom");
    info!("Changing the selected value by scrolling the mousewheel");
}

/// Listens for Z key being pressed and toggles between the two scroll-type states [`ScrollType`]
fn switch_scroll_type(
    scroll_type: Res<State<ScrollType>>,
    mut next_scroll_type: ResMut<NextState<ScrollType>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        let result = match scroll_type.get() {
            ScrollType::MovementSpeed => ScrollType::Zoom,
            ScrollType::Zoom => ScrollType::MovementSpeed,
        };

        println!("{:?}", result);
        next_scroll_type.set(result);
    }
}

/// Depending on the state, the mouse-scroll changes either the movement speed or the field-of-view of the camera
fn scroll(
    scroll_type: Res<State<ScrollType>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut FpsCamera, &mut Projection)>,
) {
    for event in mouse_wheel_events.iter() {
        if *scroll_type.get() == ScrollType::MovementSpeed {
            for (mut camera, project) in query.iter_mut() {
                if let Projection::Perspective(perspective) = project.into_inner() {
                    perspective.fov = (perspective.fov - event.y * 0.01).abs();
                    println!("FOV: {:?}", perspective.fov);
                }
                camera.speed *= Mat4::from_scale(Vec3::splat(event.y * 0.1));
                println!("Speed: {:?}", camera.speed);
            }
        } else {
            for (_camera, project) in query.iter_mut() {
                if let Projection::Perspective(perspective) = project.into_inner() {
                    perspective.fov = (perspective.fov - event.y * 0.01).abs();
                    println!("FOV: {:?}", perspective.fov);
                }
            }
        }
    }
}
