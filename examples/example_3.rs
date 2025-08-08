//! Example showing how to spawn an horizontal scrollbar from a scrollable node.

use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_scrollbar::{DragSpeed, ScrollSpeed, Scrollable, ScrollbarPlugin, ThumbColor};

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
        // We want the scrollbar under the scrollable content
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    });
    let container_id = container.id();

    // Spawn the scrollable node
    container.with_child((
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(10.0),
            border: UiRect::all(Val::Px(5.0)),
            // Setting scroll on the x axis will spawn an horizontal scrollbar
            overflow: Overflow::scroll_x(),
            ..default()
        },
        BorderColor::all(Color::BLACK),
        Children::spawn(SpawnIter((0..100).map(|i| {
            (
                Node {
                    min_width: Val::Percent(5.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::BLACK),
                Children::spawn_one(Text::new(format!("{i}"))),
            )
        }))),
        // Customize scroll speed of the content
        ScrollSpeed(20.0),
        // Spawn the scrollbar
        Scrollable::spawn_one((
            // Add the scrollbar as a child of the container
            ChildOf(container_id),
            Node {
                // Spawn scrollbar as wide as the Scrollable content
                width: Val::Percent(80.0),
                height: Val::Percent(3.0),
                margin: UiRect::top(Val::Px(5.0)),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(Color::BLACK),
            // The thumb will be spawned with this same border radius
            BorderRadius::all(Val::Px(10.0)),
            // Customize color of the thumb
            ThumbColor(Color::srgb(0.0, 1.0, 0.0)),
            // Customize drag speed of the thumb
            DragSpeed(6.0),
        )),
    ));
}
