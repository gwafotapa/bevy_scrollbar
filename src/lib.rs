//! Bevy plugin providing a scrollbar.
//!
//! # Getting started
//!
//! First add `bevy_scrollbar` to your dependencies and the [`ScrollbarPlugin`] to your app.
//!
//! # Making a scrollbar
//!
//! The two pieces of a scrollbar are referred to as the _track_ and the _thumb_.  You can turn an entity into a scrollbar (track) by adding [`Scrollbar { scrollable }`](Scrollbar) to it, where `scrollable` is the entity Id of another node with overflowing content. This spawns the thumb as the child of the track along with three observers. See [`Scrollbar`] for more details.
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
//!                         // You can omit the overflow field for a vertical scrollbar
//!                         overflow: Overflow::scroll_y(),
//!                         flex_direction: FlexDirection::Column,
//!                         ..default()
//!                     },
//!                     BorderColor::all(Color::BLACK),
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
//!                 BorderColor::all(Color::BLACK),
//!             ));
//!         })),
//!     ));
//! }
//! ```
//!
//! # The [`Scrollbar`] / [`Scrollable`] relationship
//!
//! The [`Scrollbar`] component implements `Relationship` with target [`Scrollable`]. This relates the [`Scrollbar`] node to the overflowed node to which [`Scrollable`] is added. This means [`Scrollable`] can be used to spawn a scrollbar (the same way `Children` can be used to spawn children). See [example-2](crate#example-2).
//!
//! # The [`Scrollable`] content
//!
//! The [`Scrollable`] content responds to mouse `Scroll` triggers. You can configure how fast the content scrolls by adding [`ScrollSpeed`] to the [`Scrollable`] node. See [example-2](crate#example-2).
//!
//! # Thumb customization
//!
//! Color and `Drag` speed of the thumb can be configured by adding [`ThumbColor`] and [`DragSpeed`] to the [`Scrollbar`]. See [example-2](crate#example-2).
//!
//! # Example 2
//!
//!```no_run
//! use bevy::{ecs::spawn::SpawnIter, prelude::*};
//! use bevy_scrollbar::{
//!     Scrollable, ScrollSpeed, ScrollbarPlugin, ThumbColor, DragSpeed,
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
//!         BorderColor::all(Color::BLACK),
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
//!         ScrollSpeed(2.0),
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
//!             BorderColor::all(Color::BLACK),
//!             // The thumb will be spawned with the same border radius
//!             BorderRadius::all(Val::Px(10.0)),
//!             // Customize color of the thumb
//!             ThumbColor(Color::srgb(0.0, 0.0, 1.0)),
//!             // Customize drag speed of the thumb
//!             DragSpeed(4.0),
//!         )),
//!     ));
//! }
//!```

mod scrollable;
mod scrollbar;

use bevy::{prelude::*, ui::UiSystems};
pub use scrollable::{ScrollSpeed, Scrollable, ScrollableLineHeight};
pub use scrollbar::{DragSpeed, Scrollbar, ThumbColor};

/// Plugin scheduling [`ScrollbarSystems`] after `UiSystem::Layout` in `PostUpdate`.
pub struct ScrollbarPlugin;

/// `SystemSet` containing the system updating the thumb of a [`Scrollbar`].
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScrollbarSystems;

impl Plugin for ScrollbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            update_scroll_position_and_thumb
                .after(UiSystems::Layout)
                .in_set(ScrollbarSystems),
        );
    }
}

/// Clamps [`ScrollPosition`] and updates the length and position of the thumb.
///
/// Bevy computes layout and `Transform` of UI nodes in `UiSystems::Layout`. This system runs in `PostUpdate` after `UiSystems::Layout` and uses change detection on the [`Scrollable`] node. Graphically, the thumb is updated on the frame following the change. This allows us to use the computation done by `UiSystems::Layout`.
fn update_scroll_position_and_thumb(
    q_changed_scrollable: Query<
        (&Scrollable, &Node, Ref<ComputedNode>),
        Or<(Changed<ComputedNode>, Changed<ScrollPosition>)>,
    >,
    q_children: Query<&Children>,
    mut q_node: Query<&mut Node, Without<Scrollable>>,
    mut commands: Commands,
) -> Result {
    for (scrollable, scrollable_node, scrollable_cnode) in &q_changed_scrollable {
        let thumb = q_children.get(scrollable.scrollbar())?[0];
        commands.run_system_cached_with(update_scroll_and_thumb_positions, thumb);

        // Recompute thumb length only if the content changed, not if it was merely scrolled
        if scrollable_cnode.is_changed() {
            let mut thumb_node = q_node.get_mut(thumb)?;
            if scrollable_node.overflow.y == OverflowAxis::Scroll {
                let ratio = scrollable_cnode.size.y / scrollable_cnode.content_size.y;
                thumb_node.height = Val::Percent(ratio * 100.0);
            } else if scrollable_node.overflow.x == OverflowAxis::Scroll {
                let ratio = scrollable_cnode.size.x / scrollable_cnode.content_size.x;
                thumb_node.width = Val::Percent(ratio * 100.0);
            }
        }
    }
    Ok(())
}

/// Clamps [`ScrollPosition`] and updates the position of the thumb.
fn update_scroll_and_thumb_positions(
    In(thumb): In<Entity>,
    mut q_thumb: Query<(&mut Node, &ComputedNode, &ChildOf), Without<Scrollable>>,
    q_scrollbar: Query<(&Scrollbar, &ComputedNode)>,
    mut q_scrollable: Query<(&mut ScrollPosition, &Node, &ComputedNode), With<Scrollable>>,
) -> Result {
    let (mut thumb_node, thumb_cnode, child_of) = q_thumb.get_mut(thumb)?;
    let scrollbar = child_of.parent();
    let (&Scrollbar { scrollable }, track_cnode) = q_scrollbar.get(scrollbar)?;
    let (mut scroll_position, scrollable_node, scrollable_cnode) =
        q_scrollable.get_mut(scrollable)?;

    if scrollable_node.overflow.y == OverflowAxis::Scroll {
        let scaled_scroll_length = scrollable_cnode.content_size.y - scrollable_cnode.size.y;
        let scroll_length = scrollable_cnode.inverse_scale_factor * scaled_scroll_length;
        scroll_position.y = scroll_position.y.clamp(0.0, scroll_length);
        thumb_node.margin.top = if scroll_length <= 0.0 {
            Val::ZERO
        } else {
            let ratio = scroll_position.y / scroll_length;
            let scaled_drag_length = track_cnode.size.y
                - (track_cnode.border.top + track_cnode.border.bottom + thumb_cnode.size.y);
            let drag_length = track_cnode.inverse_scale_factor * scaled_drag_length;
            Val::Px(ratio * drag_length)
        };
        debug!("scrollable node size: {}", scrollable_cnode.size.y);
        debug!(
            "scrollable content size: {}",
            scrollable_cnode.content_size.y,
        );
        debug!("thumb top margin: {:?}\n", thumb_node.margin.top);
    } else if scrollable_node.overflow.x == OverflowAxis::Scroll {
        let scaled_scroll_length = scrollable_cnode.content_size.x - scrollable_cnode.size.x;
        let scroll_length = scrollable_cnode.inverse_scale_factor * scaled_scroll_length;
        scroll_position.x = scroll_position.x.clamp(0.0, scroll_length);
        thumb_node.margin.left = if scroll_length <= 0.0 {
            Val::ZERO
        } else {
            let ratio = scroll_position.x / scroll_length;
            let scaled_drag_length = track_cnode.size.x
                - (track_cnode.border.left + track_cnode.border.right + thumb_cnode.size.x);
            let drag_length = track_cnode.inverse_scale_factor * scaled_drag_length;
            Val::Px(ratio * drag_length)
        };
    }
    Ok(())
}
