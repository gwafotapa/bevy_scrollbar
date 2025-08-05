//! Example showing how to spawn a scrollbar from the scrollable entity.

use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_scrollbar::{Scrollable, ScrollableSettings, ScrollbarPlugin, ScrollbarSettings};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollbarPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Container of the scrolling content and its scrollbar
    let mut container = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    });
    let container_id = container.id();

    // Spawn the scrollable content
    container.with_child((
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(80.0),
            border: UiRect::all(Val::Px(5.0)),
            flex_wrap: FlexWrap::Wrap,
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
        ScrollableSettings {
            scroll_speed: 2.0,
            ..default()
        },
        // Spawn the scrollbar
        Scrollable::spawn_one((
            Node {
                width: Val::Percent(1.5),
                height: Val::Percent(80.0),
                margin: UiRect::left(Val::Px(5.0)),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.0)),
            // Add the scrollbar as a child of the container
            ChildOf(container_id),
            // Customize color and speed of the thumb
            ScrollbarSettings {
                thumb_color: Color::srgb(0.0, 0.0, 1.0),
                thumb_speed: 4.0,
            },
        )),
    ));
}
