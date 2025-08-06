//! Bevy plugin providing a vertical scrollbar.
//!
//! # Getting started
//!
//! First add `bevy_scrollbar` to your dependencies and the [`ScrollbarPlugin`] to your app.
//!
//! # Making a scrollbar
//!
//! The two pieces of a scrollbar are referred to as the _track_ and the _thumb_.  You can turn an entity into a scrollbar (track) by adding [`Scrollbar { scrollable }`](Scrollbar) to it, where `scrollable` is the Id of an another node, usually overflowed. This adds `Node` to your entity if it's missing and spawns the thumb as its child.
//!
//! # Example 1
//!
//! ```no_run
//! use bevy::{
//!     ecs::spawn::{SpawnIter, SpawnWith},
//!     prelude::*,
//! };
//! use bevy_scrollbar::{Scrollbar, ScrollbarPlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((DefaultPlugins, ScrollbarPlugin))
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera2d);
//!
//!     commands.spawn((
//!         // Container node for the overflowed node and its scrollbar that are usually siblings.
//!         Node {
//!             width: Val::Percent(100.0),
//!             height: Val::Percent(100.0),
//!             justify_content: JustifyContent::Center,
//!             align_items: AlignItems::Center,
//!             ..default()
//!         },
//!         Children::spawn(SpawnWith(|container: &mut ChildSpawner| {
//!             // Overflowed node. No need to set the overflow field
//!             let scrollable = container
//!                 .spawn((
//!                     Node {
//!                         height: Val::Percent(80.0),
//!                         border: UiRect::all(Val::Px(5.0)).with_right(Val::Px(2.5)),
//!                         flex_direction: FlexDirection::Column,
//!                         ..default()
//!                     },
//!                     BorderColor(Color::BLACK),
//!                     Children::spawn(SpawnIter(
//!                         (0..100).map(|i| Text::new(format!("  Scrolling {i}!  "))),
//!                     )),
//!                 ))
//!                 .id();
//!
//!             // Scrollbar
//!             container.spawn((
//!                 Scrollbar { scrollable },
//!                 Node {
//!                     width: Val::Percent(1.5),
//!                     height: Val::Percent(80.0),
//!                     border: UiRect::all(Val::Px(5.0)).with_left(Val::Px(2.5)),
//!                     ..default()
//!                 },
//!                 BorderColor(Color::BLACK),
//!             ));
//!         })),
//!     ));
//! }
//! ```
//!
//! # The [`Scrollbar`] / [`Scrollable`] relationship
//!
//! The [`Scrollbar`] component implements `Relationship` with target [`Scrollable`]. This relates the [`Scrollbar`] node to the overflowed node to which [`Scrollable`] is added. This means [`Scrollable`] can be used to spawn a scrollbar (the same way `Children` can be used to spawn children). See [example 2](crate#example-2).
//!
//! # The [`Scrollable`] content
//!
//! When the [`Scrollable`] component is added to the overflowed node, it adds an observer watching the node for mouse wheel `Scroll`s. You can configure how fast the content scrolls by adding [`ScrollableSettings`] to the [`Scrollable`] node. See [example 2](crate#example-2).
//!
//! # Customization of the thumb
//!
//! Color and drag speed of the thumb can be configured by adding [`ThumbSettings`] to the [`Scrollbar`]. See [example 2](crate#example-2).
//!
//! # Example 2
//!
//!```no_run
//! use bevy::{ecs::spawn::SpawnIter, prelude::*};
//! use bevy_scrollbar::{Scrollable, ScrollableSettings, ScrollbarPlugin, ThumbSettings};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((DefaultPlugins, ScrollbarPlugin))
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera2d);
//!
//!     // Container of the scrolling content and its scrollbar
//!     let mut container = commands.spawn(Node {
//!         width: Val::Percent(100.0),
//!         height: Val::Percent(100.0),
//!         justify_content: JustifyContent::Center,
//!         align_items: AlignItems::Center,
//!         ..default()
//!     });
//!     let container_id = container.id();
//!
//!     // Spawn the scrollable content
//!     container.with_child((
//!         Node {
//!             width: Val::Percent(80.0),
//!             height: Val::Percent(80.0),
//!             border: UiRect::all(Val::Px(5.0)),
//!             flex_wrap: FlexWrap::Wrap,
//!             ..default()
//!         },
//!         BorderColor(Color::BLACK),
//!         Children::spawn(SpawnIter((0..100).map(|i| {
//!             (
//!                 Node {
//!                     width: Val::Percent(20.0),
//!                     height: Val::Percent(20.0),
//!                     justify_content: JustifyContent::Center,
//!                     align_items: AlignItems::Center,
//!                     ..default()
//!                 },
//!                 Children::spawn_one(Text::new(format!("Scroll {i}!"))),
//!             )
//!         }))),
//!         // Customize scroll speed of the content
//!         ScrollableSettings {
//!             scroll_speed: 2.0,
//!             ..default()
//!         },
//!         // Spawn the scrollbar
//!         Scrollable::spawn_one((
//!             Node {
//!                 width: Val::Percent(1.5),
//!                 height: Val::Percent(80.0),
//!                 margin: UiRect::left(Val::Px(5.0)),
//!                 border: UiRect::all(Val::Px(5.0)),
//!                 ..default()
//!             },
//!             BorderColor(Color::BLACK),
//!             BorderRadius::all(Val::Px(10.0)),
//!             // Add the scrollbar as a child of the container
//!             ChildOf(container_id),
//!             // Customize color and speed of the thumb
//!             ThumbSettings {
//!                 color: Color::srgb(0.0, 0.0, 1.0),
//!                 speed: 4.0,
//!             },
//!         )),
//!     ));
//! }
//!```

mod scrollable;
mod scrollbar;

pub use scrollable::{Scrollable, ScrollableSettings};
pub use scrollbar::{Scrollbar, ThumbSettings};

use bevy::{prelude::*, ui::UiSystem};

/// Plugin scheduling [`ScrollbarSystem`] after `UiSystem::Layout` in `PostUpdate`.
pub struct ScrollbarPlugin;

/// `SystemSet` containing the system updating the height of the [`Scrollbar`].
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScrollbarSystem;

impl Plugin for ScrollbarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Scrollbar>()
            .register_type::<Scrollable>()
            .register_type::<ThumbSettings>()
            .register_type::<ScrollableSettings>();

        app.add_systems(
            PostUpdate,
            update_thumb_height
                .after(UiSystem::Layout)
                .in_set(ScrollbarSystem),
        );
    }
}

/// Updates the height of the thumb.
///
/// Bevy computes layout and `Transform` of UI nodes in `UiSystem::Layout`. This system runs in `PostUpdate` after `UiSystem::Layout` and uses change detection on the `ComputedNode` of the [`Scrollable`] content. Graphically, the height of the thumb is updated on the frame following the change. That tradeoff avoids computing the size of the [`Scrollable`] content.
fn update_thumb_height(
    q_changed_scrollable: Query<(&Scrollable, &ComputedNode), Changed<ComputedNode>>,
    q_children: Query<&Children>,
    mut q_node: Query<&mut Node>,
) {
    for (scrollable, scrollable_cnode) in &q_changed_scrollable {
        let thumb_id = q_children.get(scrollable.scrollbar()).unwrap()[0];
        let mut thumb = q_node.get_mut(thumb_id).unwrap();
        let ratio = scrollable_cnode.size.y / scrollable_cnode.content_size.y;
        thumb.height = Val::Percent(ratio * 100.0);
        debug!(
            "Thumb height = {} / {} = {}%",
            scrollable_cnode.size.y, scrollable_cnode.content_size.y, ratio
        );
    }
}
