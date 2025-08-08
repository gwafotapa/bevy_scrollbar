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
/// * add a `Relationship` between the scrollbar and the `scrollable` entity, inserting [`Scrollable`] into the target which typically has overflowing content;
/// * if the target does not have either `Node::overflow::y` or `Node::overflow::x` set to `OverflowAxis::Scroll`, then set `Node::overflow::y` to `OverflowAxis::Scroll` for a vertical scrollbar;
/// * spawn the _thumb_ of the scrollbar as its child;
/// * spawn an observer watching the target for `Scroll` triggers;
/// * spawn an observer watching the thumb for `Drag` triggers;
/// * spawn an observer watching the scrollbar for `Click` triggers.
///
/// The scroll speed of the content can be configured by adding [`ScrollSpeed`] to the target. The color and drag speed of the thumb can be configured by adding [`ThumbColor`] and [`DragSpeed`] to the scrollbar.

#[derive(Component, Clone, Reflect, Debug)]
#[relationship(relationship_target = Scrollable)]
#[require(Node, ThumbColor, DragSpeed)]
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
pub struct DragSpeed(pub f32);

impl Default for DragSpeed {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl DragSpeed {
    /// Default value of [`DragSpeed`].
    pub const DEFAULT: f32 = 4.0;
}

/// `on_add` hook of [`Scrollbar`].
fn spawn_thumb_and_observers(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let &Scrollbar { scrollable } = world.get::<Scrollbar>(entity).unwrap();
    world.commands().queue(move |world: &mut World| {
        let Ok(mut scrollable) = world.get_entity_mut(scrollable) else {
            warn!(
                "Scrollbar setup aborted. Scrollable entity {} does not exist.",
                scrollable.index()
            );
            return;
        };

        let Some(mut node) = scrollable.get_mut::<Node>() else {
            warn!(
                "Scrollbar setup aborted. Scrollable entity {} is missing the Node component.",
                scrollable.id().index()
            );
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

        // Observe the scrollable node for mouse Scroll triggers
        scrollable.observe(scroll_content_on_mouse_scroll);

        let Ok(scrollbar) = world.get_entity_mut(entity) else {
            warn!(
                "Scrollbar setup aborted. Scrollbar entity {} does not exist.",
                entity.index()
            );
            return;
        };

        // Spawn the thumb and observe it for Drag triggers
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
        let border_radius = *scrollbar.get::<BorderRadius>().unwrap();
        let thumb_color = scrollbar.get::<ThumbColor>().unwrap().0;
        let thumb = world
            .spawn((
                node,
                ChildOf(entity),
                border_radius,
                BackgroundColor(thumb_color),
            ))
            .observe(scroll_content_on_thumb_drag)
            .id();

        // Observe both the scrollbar and the thumb for Click triggers. The thumb is observed to stop
        // trigger propagation to the track.
        let observer = Observer::new(jump_content_on_trough_click)
            .with_entity(entity)
            .with_entity(thumb);
        world.spawn(observer);
    });
}

/// Observer watching a [`Scrollable`] node for `Scroll` triggers.
fn scroll_content_on_mouse_scroll(
    scroll: Trigger<Pointer<Scroll>>,
    mut q_scrollable: Query<(
        &mut ScrollPosition,
        &Node,
        &ScrollSpeed,
        Option<&ScrollableLineHeight>,
    )>,
) -> Result {
    let scrollable = scroll.target();
    let (mut scroll_position, node, scroll_speed, line_height) =
        q_scrollable.get_mut(scrollable)?;
    let mouse_scroll = match (scroll.unit, line_height) {
        (MouseScrollUnit::Line, Some(line_height)) => scroll.y * line_height.px(),
        _ => scroll.y,
    };
    let scroll = scroll_speed.0 * mouse_scroll;
    if node.overflow.y == OverflowAxis::Scroll {
        scroll_position.offset_y -= scroll;
    } else if node.overflow.x == OverflowAxis::Scroll {
        scroll_position.offset_x -= scroll;
    };
    Ok(())
}

/// Observer watching the thumb of the [`Scrollbar`] for `Drag` triggers.
fn scroll_content_on_thumb_drag(
    drag: Trigger<Pointer<Drag>>,
    q_child_of: Query<&ChildOf>,
    q_scrollbar: Query<(&Scrollbar, &DragSpeed)>,
    mut q_scrollable: Query<(&mut ScrollPosition, &Node)>,
) -> Result {
    let thumb = drag.target();
    let scrollbar = q_child_of.get(thumb)?.parent();
    let (&Scrollbar { scrollable }, drag_speed) = q_scrollbar.get(scrollbar)?;
    let (mut scroll_position, node) = q_scrollable.get_mut(scrollable)?;
    if node.overflow.y == OverflowAxis::Scroll {
        scroll_position.offset_y += drag_speed.0 * drag.delta.y;
    } else if node.overflow.x == OverflowAxis::Scroll {
        scroll_position.offset_x += drag_speed.0 * drag.delta.x;
    };
    Ok(())
}

/// Observer watching both the [`Scrollbar`] and its thumb for `Click` triggers.
///
/// This observer handles clicking the trough (i.e. the region of the track not covered by the thumb). When clicked, the thumb jumps to that position. This is achieved by discarding clicks on the thumb before they propagate to the track. This system only adjusts the ScrollPosition of the content. update_thumb() will see the change and update the thumb position as a result.
fn jump_content_on_trough_click(
    mut click: Trigger<Pointer<Click>>,
    q_scrollbar: Query<(&Scrollbar, &ComputedNode, &Children)>,
    q_node: Query<(&Node, &ComputedNode)>,
    mut q_scroll_position: Query<&mut ScrollPosition>,
) -> Result {
    let Some(click_position) = click.hit.position else {
        warn!("Scrollbar Click observed but hit position is missing to move the thumb");
        return Ok(());
    };

    let scrollbar = click.target();
    let Ok((&Scrollbar { scrollable }, track_cnode, children)) = q_scrollbar.get(scrollbar) else {
        // Discard event because the thumb was clicked
        click.propagate(false);
        return Ok(());
    };

    let thumb = children[0];
    let (_, thumb_cnode) = q_node.get(thumb)?;
    let (scrollable_node, scrollable_cnode) = q_node.get(scrollable)?;
    let mut scroll_position = q_scroll_position.get_mut(scrollable)?;

    if scrollable_node.overflow.y == OverflowAxis::Scroll {
        let click_y = (thumb_cnode.size.y / 2.0)
            .max(click_position.y * track_cnode.size.y)
            .min(track_cnode.size.y - thumb_cnode.size.y / 2.0);
        let ratio =
            (click_y - thumb_cnode.size.y / 2.0) / (track_cnode.size.y - thumb_cnode.size.y);
        scroll_position.offset_y = track_cnode.inverse_scale_factor
            * ratio
            * (scrollable_cnode.content_size.y - scrollable_cnode.size.y);
    } else if scrollable_node.overflow.x == OverflowAxis::Scroll {
        let click_x = (thumb_cnode.size.x / 2.0)
            .max(click_position.x * track_cnode.size.x)
            .min(track_cnode.size.x - thumb_cnode.size.x / 2.0);
        let ratio =
            (click_x - thumb_cnode.size.x / 2.0) / (track_cnode.size.x - thumb_cnode.size.x);
        scroll_position.offset_x = track_cnode.inverse_scale_factor
            * ratio
            * (scrollable_cnode.content_size.x - scrollable_cnode.size.x);
    };
    Ok(())
}
