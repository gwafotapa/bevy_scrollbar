//! Bevy plugin providing a scrollbar.
//!
//! # Getting started
//!
//! First add `bevy_scrollbar` to your dependencies and the [`ScrollbarPlugin`] to your app.
//!
//! # Making a scrollbar
//!
//! The two pieces of a scrollbar are referred to as the _track_ and the _thumb_.  You can turn an entity into a scrollbar (track) by adding [`Scrollbar { scrollable }`](Scrollbar) to it, where `scrollable` is the entity Id of another node which is usually overflowed (i.e. with overflowing content). This spawns the thumb as the child of the track along with a couple observers. See [`Scrollbar`] for more details.
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
//!             // Overflowed node
//!             let scrollable = container
//!                 .spawn((
//!                     Node {
//!                         height: Val::Percent(80.0),
//!                         border: UiRect::all(Val::Px(5.0)).with_right(Val::Px(2.5)),
//!                         // You can omit the overflow field for a vertical scrollbar in which case
//!                         // it will be automatically set to Overflow::scroll_y()
//!                         overflow: Overflow::scroll_y(),
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
//!                     // Same height as the scrollable node
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
//! The [`Scrollable`] content responds to mouse wheel `Scroll` triggers. You can configure how fast the content scrolls by adding [`ScrollableScrollScale`] to the [`Scrollable`] node. See [example 2](crate#example-2).
//!
//! # Customization of the thumb
//!
//! Color and drag speed of the thumb can be configured by adding [`ThumbColor`] and [`ThumbDragScale`] to the [`Scrollbar`]. See [example 2](crate#example-2).
//!
//! # Example 2
//!
//!```no_run
//! use bevy::{ecs::spawn::SpawnIter, prelude::*};
//! use bevy_scrollbar::{
//!     Scrollable, ScrollableScrollScale, ScrollbarPlugin, ThumbColor, ThumbDragScale,
//! };
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
//!     // Container of the scrollable content and its scrollbar
//!     let mut container = commands.spawn(Node {
//!         width: Val::Percent(100.0),
//!         height: Val::Percent(100.0),
//!         justify_content: JustifyContent::Center,
//!         align_items: AlignItems::Center,
//!         ..default()
//!     });
//!     let container_id = container.id();
//!
//!     // Spawn the scrollable node
//!     container.with_child((
//!         Node {
//!             width: Val::Percent(80.0),
//!             height: Val::Percent(80.0),
//!             border: UiRect::all(Val::Px(5.0)),
//!             flex_wrap: FlexWrap::Wrap,
//!             // Ommitting the overflow field because the scrollbar is vertical
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
//!         ScrollableScrollScale(2.0),
//!         // Spawn the scrollbar
//!         Scrollable::spawn_one((
//!             // Add the scrollbar as a child of the container
//!             ChildOf(container_id),
//!             Node {
//!                 width: Val::Percent(1.5),
//!                 // Same height as the scrollable node
//!                 height: Val::Percent(80.0),
//!                 margin: UiRect::left(Val::Px(5.0)),
//!                 border: UiRect::all(Val::Px(5.0)),
//!                 ..default()
//!             },
//!             BorderColor(Color::BLACK),
//!             // The thumb will be spawned with the same border radius
//!             BorderRadius::all(Val::Px(10.0)),
//!             // Customize color of the thumb
//!             ThumbColor(Color::srgb(0.0, 0.0, 1.0)),
//!             // Customize drag speed of the thumb
//!             ThumbDragScale(4.0),
//!         )),
//!     ));
//! }
//!```

mod scrollable;
mod scrollbar;

pub use scrollable::{Scrollable, ScrollableLineHeight, ScrollableScrollScale};
pub use scrollbar::{Scrollbar, ThumbColor, ThumbDragScale};

use bevy::{prelude::*, ui::UiSystem};

/// Plugin scheduling [`ScrollbarSystem`] after `UiSystem::Layout` in `PostUpdate`.
pub struct ScrollbarPlugin;

/// `SystemSet` containing the system updating the length of the [`Scrollbar`].
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScrollbarSystem;

impl Plugin for ScrollbarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Scrollbar>()
            .register_type::<Scrollable>()
            .register_type::<ThumbColor>()
            .register_type::<ThumbDragScale>()
            .register_type::<ScrollableScrollScale>()
            .register_type::<ScrollableLineHeight>();

        app.add_systems(
            PostUpdate,
            update_thumb_length
                .after(UiSystem::Layout)
                .in_set(ScrollbarSystem),
        );
    }
}

/// Updates the length of the thumb.
///
/// Bevy computes layout and `Transform` of UI nodes in `UiSystem::Layout`. This system runs in `PostUpdate` after `UiSystem::Layout` and uses change detection on the `ComputedNode` of the [`Scrollable`] node. Graphically, the length of the thumb is updated on the frame following the change. That tradeoff avoids computing the size of the [`Scrollable`] content.
fn update_thumb_length(
    q_changed_scrollable: Query<(&Scrollable, &Node, &ComputedNode), Changed<ComputedNode>>,
    q_children: Query<&Children>,
    mut q_node: Query<&mut Node, Without<Scrollable>>,
) -> Result {
    for (scrollable, scrollable_node, scrollable_cnode) in &q_changed_scrollable {
        let thumb = q_children.get(scrollable.scrollbar())?[0];
        let mut thumb_node = q_node.get_mut(thumb)?;
        if scrollable_node.overflow.y == OverflowAxis::Scroll {
            let ratio = scrollable_cnode.size.y / scrollable_cnode.content_size.y;
            thumb_node.height = Val::Percent(ratio * 100.0);
            debug!(
                "Thumb height = {} / {} = {}%",
                scrollable_cnode.size.y, scrollable_cnode.content_size.y, ratio
            );
        } else if scrollable_node.overflow.x == OverflowAxis::Scroll {
            let ratio = scrollable_cnode.size.x / scrollable_cnode.content_size.x;
            thumb_node.width = Val::Percent(ratio * 100.0);
            debug!(
                "Thumb width = {} / {} = {}%",
                scrollable_cnode.size.x, scrollable_cnode.content_size.x, ratio
            );
        }
    }
    Ok(())
}
