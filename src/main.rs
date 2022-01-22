use bevy::{
    input::mouse::{MouseButtonInput},
    prelude::*,
    window::CursorMoved
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(print_mouse_event_system)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn print_mouse_event_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in mouse_button_input_events.iter() {
        info!("mouse button input: {:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!("cursor moved to: {:?}", event.position);
    }
}