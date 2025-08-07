//! Example showing how to spawn a scrollbar directly.

use bevy::{
    ecs::spawn::{SpawnIter, SpawnWith},
    prelude::*,
};
use bevy_scrollbar::{Scrollbar, ScrollbarPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollbarPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        // Container node for the overflowed node and its scrollbar that are usually siblings.
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Children::spawn(SpawnWith(|container: &mut ChildSpawner| {
            // Overflowed node
            let scrollable = container
                .spawn((
                    Node {
                        height: Val::Percent(80.0),
                        border: UiRect::all(Val::Px(5.0)).with_right(Val::Px(2.5)),
                        // You can omit the overflow field for a vertical scrollbar
                        overflow: Overflow::scroll_y(),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    Children::spawn(SpawnIter(
                        (0..100).map(|i| Text::new(format!("  Scrolling {i}!  "))),
                    )),
                ))
                .id();

            // Scrollbar
            container.spawn((
                Scrollbar { scrollable },
                Node {
                    width: Val::Percent(1.5),
                    // Same height as the scrollable node
                    height: Val::Percent(80.0),
                    border: UiRect::all(Val::Px(5.0)).with_left(Val::Px(2.5)),
                    ..default()
                },
                BorderColor(Color::BLACK),
            ));
        })),
    ));
}
