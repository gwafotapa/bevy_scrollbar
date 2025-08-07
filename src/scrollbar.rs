use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    input::mouse::MouseScrollUnit,
    prelude::*,
};

use crate::{ScrollSpeed, Scrollable, ScrollableLineHeight};

/// Component of a scrollbar `Node`.
///
/// Add this component to an entity to turn it into a scrollbar. Doing so will:
/// * add the `Node` component if it's not already present;
/// * add a `Relationship` between the scrollbar and the targeted [`scrollable`](Self::scrollable) node, inserting [`Scrollable`] into the target which typically has overflowing content;
/// * if the target does not have either `Node::overflow::x` or `Node::overflow::y` set to `OverflowAxis::Scroll`, then set its `Node::overflow::y` to `OverflowAxis::Scroll` and configure the scrollbar vertical;
/// * spawn an observer watching the target for mouse wheel `Scroll` triggers;
/// * spawn the _thumb_ of the scrollbar as its child;
/// * spawn an observer watching the thumb for `Drag`triggers;
///
/// The scroll speed of the mouse wheel can be configured by adding [`ScrollSpeed`] to the target. The color and drag speed of the thumb can be configured by adding [`ThumbColor`] and [`ThumbDragScale`] to the scrollbar.

#[derive(Component, Clone, Reflect, Debug)]
#[relationship(relationship_target = Scrollable)]
#[require(Node, ThumbColor, ThumbDragScale)]
#[component(immutable)]
#[component(on_add = spawn_thumb_and_observers)]
pub struct Scrollbar {
    /// The [`Scrollable`] entity of this scrollbar entity.
    pub scrollable: Entity,
}

/// Component of a [`Scrollbar`] configuring the color of its thumb.
///
/// This component is immutable to remind you it is only used at the spawning of the [`Scrollbar`]. If you want to change the color of the thumb afterwards, mutate its `Color` component directly.
#[derive(Component, Default, Copy, Clone, Reflect, Debug)]
#[component(immutable)]
pub struct ThumbColor(pub Color);

/// Component of a [`Scrollbar`] configuring how fast its thumb moves when dragged.
///
/// This is unrelated to how fast the content scrolls when scrolling the mouse. See [`ScrollSpeed`] for that.
#[derive(Component, Copy, Clone, Reflect, Debug)]
pub struct ThumbDragScale(pub f32);

impl Default for ThumbDragScale {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl ThumbDragScale {
    /// Default value of [`ThumbDragScale`].
    pub const DEFAULT: f32 = 4.0;
}

/// `on_add` hook of [`Scrollbar`].
fn spawn_thumb_and_observers(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let &Scrollbar { scrollable } = world.get::<Scrollbar>(entity).unwrap();
    world.commands().queue(move |world: &mut World| {
        let Ok(mut scrollable) = world.get_entity_mut(scrollable) else {
            warn!(
                "Scrollbar setup aborted. Scrollable entity {} does not exist anymore.",
                scrollable.index()
            );
            return;
        };

        let Some(mut node) = scrollable.get_mut::<Node>() else {
            warn!("Scrollbar setup aborted. Scrollable entity is missing the Node component.");
            return;
        };

        enum ScrollDirection {
            Vertical,
            Horizontal,
        }

        // Choose an overflowing axis on the scrollable node if none is set
        let direction = match (node.overflow.x, node.overflow.y) {
            (_, OverflowAxis::Scroll) => ScrollDirection::Vertical,
            (OverflowAxis::Scroll, _) => ScrollDirection::Horizontal,
            (_, _) => {
                node.overflow = Overflow::scroll_y();
                ScrollDirection::Vertical
            }
        };

        // Set line height on the scrollable node if none is set and the scrollbar is vertical
        if matches!(direction, ScrollDirection::Vertical)
            && !scrollable.contains::<ScrollableLineHeight>()
        {
            scrollable.insert(ScrollableLineHeight::default());
        }

        // Observe the scrollable node for mouse wheel `Scroll` triggers
        scrollable.observe(scroll_on_wheel);

        // Spawn the thumb and observe it for `Drag` triggers
        let node = match direction {
            ScrollDirection::Vertical => Node {
                width: Val::Percent(100.0),
                height: Val::ZERO,
                ..default()
            },
            ScrollDirection::Horizontal => Node {
                width: Val::ZERO,
                height: Val::Percent(100.0),
                ..default()
            },
        };
        let Ok(scrollbar) = world.get_entity(entity) else {
            warn!(
                "Scrollbar setup aborted. Scrollbar entity {} does not exist anymore.",
                entity.index()
            );
            return;
        };
        let border_radius = *scrollbar.get::<BorderRadius>().unwrap();
        let thumb_color = scrollbar.get::<ThumbColor>().unwrap().0;
        world
            .spawn((
                node,
                ChildOf(entity),
                border_radius,
                BackgroundColor(thumb_color),
            ))
            .observe(scroll_on_drag);
    });
}

/// Observer watching a [`Scrollable`] node for `Scroll` triggers.
fn scroll_on_wheel(
    scroll: Trigger<Pointer<Scroll>>,
    q_scrollable: Query<(&ScrollSpeed, Option<&ScrollableLineHeight>)>,
    mut commands: Commands,
) -> Result {
    let scrollable = scroll.target();
    let (scroll_speed, line_height) = q_scrollable.get(scrollable)?;
    let mouse_scroll = match (scroll.unit, line_height) {
        (MouseScrollUnit::Line, Some(line_height)) => scroll.y * line_height.px(),
        _ => scroll.y,
    };
    let scroll = scroll_speed.0 * mouse_scroll;
    commands.run_system_cached_with(self::scroll, (scrollable, scroll));
    Ok(())
}

/// Observer watching the thumb of the [`Scrollbar`] for `Drag` triggers.
fn scroll_on_drag(
    drag: Trigger<Pointer<Drag>>,
    q_child_of: Query<&ChildOf>,
    q_scrollbar: Query<(&Scrollbar, &ThumbDragScale)>,
    q_node: Query<&Node>,
    mut commands: Commands,
) -> Result {
    let thumb = drag.target();
    let scrollbar = q_child_of.get(thumb)?.parent();
    let (&Scrollbar { scrollable }, drag_scale) = q_scrollbar.get(scrollbar)?;
    let overflow = q_node.get(scrollable)?.overflow;
    let drag_delta = if overflow.y == OverflowAxis::Scroll {
        drag.delta.y
    } else if overflow.x == OverflowAxis::Scroll {
        drag.delta.x
    } else {
        return Ok(());
    };
    let drag = -drag_scale.0 * drag_delta;
    commands.run_system_cached_with(scroll, (scrollable, drag));
    Ok(())
}

/// Scrolls the `scrollable` node by `scroll` and moves the thumb of its [`Scrollbar`] proportionately.
///
/// Helper function of [`scroll_on_drag`] and [`scroll_on_wheel`].
fn scroll(
    In((scrollable, scroll)): In<(Entity, f32)>,
    mut q_scrollable: Query<(&mut ScrollPosition, &Node, &ComputedNode, &Scrollable)>,
    q_scrollbar: Query<(&ComputedNode, &Children)>,
    mut q_thumb: Query<(&mut Node, &ComputedNode), Without<Scrollable>>,
) {
    let (mut scrollable_position, scrollable_node, scrollable_cnode, scrollable) =
        q_scrollable.get_mut(scrollable).unwrap();
    let (track_cnode, track_children) = q_scrollbar.get(scrollable.scrollbar()).unwrap();
    let (mut thumb_node, thumb_cnode) = q_thumb.get_mut(track_children[0]).unwrap();

    if scrollable_node.overflow.y == OverflowAxis::Scroll {
        scrollable_position.offset_y -= scroll;
        let track_cnode_scroll_height = track_cnode.size.y
            - (track_cnode.border.top + track_cnode.border.bottom + thumb_cnode.size.y);
        let scrollable_cnode_scroll_height =
            scrollable_cnode.size.y - scrollable_cnode.content_size.y;
        let ratio = track_cnode_scroll_height / scrollable_cnode_scroll_height;
        let track_scroll = ratio * scroll;
        let top_margin = &mut thumb_node.margin.top;
        let top_margin_max = track_cnode.inverse_scale_factor * track_cnode_scroll_height;
        *top_margin = Val::Px(0f32.max(top_margin_max.min(px(*top_margin) + track_scroll)));

        debug!("track computed height: {}", track_cnode.size.y);
        debug!("thumb computed height: {}", thumb_cnode.size.y);
        debug!("track computed scroll height: {track_cnode_scroll_height}");
        debug!(
            "scaled top margin: {}\n",
            px(*top_margin) / track_cnode.inverse_scale_factor
        );
    } else if scrollable_node.overflow.x == OverflowAxis::Scroll {
        scrollable_position.offset_x -= scroll;
        let track_cnode_scroll_width = track_cnode.size.x
            - (track_cnode.border.left + track_cnode.border.right + thumb_cnode.size.x);
        let scrollable_cnode_scroll_width =
            scrollable_cnode.size.x - scrollable_cnode.content_size.x;
        let ratio = track_cnode_scroll_width / scrollable_cnode_scroll_width;
        let track_scroll = ratio * scroll;
        let left_margin = &mut thumb_node.margin.left;
        let left_margin_max = track_cnode.inverse_scale_factor * track_cnode_scroll_width;
        *left_margin = Val::Px(0f32.max(left_margin_max.min(px(*left_margin) + track_scroll)));

        debug!("track computed width: {}", track_cnode.size.x);
        debug!("thumb computed width: {}", thumb_cnode.size.x);
        debug!("track computed scroll width: {track_cnode_scroll_width}");
        debug!(
            "scaled left margin: {}\n",
            px(*left_margin) / track_cnode.inverse_scale_factor
        );
    }
}

/// Unwraps a `Val::Px` enum variant into its corresponding pixel value.
///
/// # Panics
///
/// Panics for all others variants.
fn px(val: Val) -> f32 {
    match val {
        Val::Px(px) => px,
        _ => panic!("Wrong variant '{val:?}'. Expected Val::Px"),
    }
}
