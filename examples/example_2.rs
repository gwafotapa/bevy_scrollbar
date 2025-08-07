//! Example showing how to spawn a vertical scrollbar from the scrollable node.

use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_scrollbar::{ScrollSpeed, Scrollable, ScrollbarPlugin, ThumbColor, ThumbDragScale};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollbarPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Container of the scrollable content and its scrollbar
    let mut container = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    });
    let container_id = container.id();

    // Spawn the scrollable node
    container.with_child((
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(80.0),
            border: UiRect::all(Val::Px(5.0)),
            flex_wrap: FlexWrap::Wrap,
            // Ommitting the overflow field because the scrollbar is vertical
            ..default()
        },
        BorderColor(Color::BLACK),
        Children::spawn(SpawnIter((0..100).map(|i| {
            (
                Node {
                    width: Val::Percent(20.0),
                    height: Val::Percent(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Children::spawn_one(Text::new(format!("Scroll {i}!"))),
            )
        }))),
        // Customize scroll speed of the content
        ScrollSpeed(2.0),
        // Spawn the scrollbar
        Scrollable::spawn_one((
            // Add the scrollbar as a child of the container
            ChildOf(container_id),
            Node {
                width: Val::Percent(1.5),
                // Same height as the scrollable node
                height: Val::Percent(80.0),
                margin: UiRect::left(Val::Px(5.0)),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor(Color::BLACK),
            // The thumb will be spawned with the same border radius
            BorderRadius::all(Val::Px(10.0)),
            // Customize color of the thumb
            ThumbColor(Color::srgb(0.0, 0.0, 1.0)),
            // Customize drag speed of the thumb
            ThumbDragScale(4.0),
        )),
    ));
}
