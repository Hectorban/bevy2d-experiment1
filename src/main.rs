use bevy::{
    prelude::*,
    window::CursorMoved
};

struct Boundary {
    point_a: Vec2,
    point_b: Vec2,
    distance: i64
}

struct Ray {
    position: Vec2,
    direction: Vec2 
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(create_square)
        .add_system(print_mouse_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(100.0, 100.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn print_mouse_event_system(mut cursor_moved_events: EventReader<CursorMoved>) {
    for event in cursor_moved_events.iter() {
        info!("X: {:?}", event.position[0]);
        info!("Y: {:?}", event.position[1]);
    }
}

fn create_square(
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_cursor_location: EventReader<CursorMoved>
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
        info!("mouse location {:?}", mouse_cursor_location.latest())
    }
}