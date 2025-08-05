use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::Scrollable;

/// Component of a scrollbar node.
///
/// Adding this component to an entity will:
/// * add the `Node` component if it's not already present;
/// * add a `Relationship` between this entity and the targeted `scrollable` node, inserting [`Scrollable`] into the target. The target typically has children overflowing its content;
/// * spawn the _thumb_ of the scrollbar as a child of this entity and watched by an observer for `Drag` triggers. The thumb can be configured by adding [`ScrollbarSettings`] to this entity.
#[derive(Component, Clone, Debug)]
#[relationship(relationship_target = Scrollable)]
#[require(Node, ScrollbarSettings)]
#[component(on_add = spawn_thumb)]
pub struct Scrollbar {
    /// The [`Scrollable`] entity of this scrollbar entity.
    pub scrollable: Entity,
}

/// Settings of the thumb of a [`Scrollbar`].
///
/// This component is added to the [`Scrollbar`] to configure its thumb.
#[derive(Component, Copy, Clone, Debug)]
pub struct ScrollbarSettings {
    /// Color of the thumb.
    pub thumb_color: Color,
    /// How many pixels the thumb is moved per `Drag::delta` unit.
    pub thumb_speed: f32,
}

impl Default for ScrollbarSettings {
    fn default() -> Self {
        Self {
            thumb_color: Color::default(),
            thumb_speed: Self::THUMB_SPEED_DEFAULT,
        }
    }
}

impl ScrollbarSettings {
    /// Default value of [`thumb_speed`](Self::thumb_speed).
    pub const THUMB_SPEED_DEFAULT: f32 = 4.0;
}

/// `on_add` hook of [`Scrollbar`].
fn spawn_thumb(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let settings = *world.get::<ScrollbarSettings>(entity).unwrap();
    let border_radius = *world.get::<BorderRadius>(entity).unwrap();
    world
        .commands()
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::ZERO, // height is adjusted by update_thumb_height
                ..default()
            },
            ChildOf(entity),
            border_radius,
            BackgroundColor(settings.thumb_color),
        ))
        .observe(scroll_on_drag);
}

/// Observer watching the thumb of the [`Scrollbar`] for `Drag` triggers.
fn scroll_on_drag(
    drag: Trigger<Pointer<Drag>>,
    q_child_of: Query<&ChildOf>,
    q_scrollbar: Query<(&Scrollbar, &ScrollbarSettings)>,
    mut commands: Commands,
) -> Result {
    let thumb = drag.target();
    let scrollbar = q_child_of.get(thumb)?.parent();
    let (&Scrollbar { scrollable }, settings) = q_scrollbar.get(scrollbar)?;
    let dy = -settings.thumb_speed * drag.delta.y;
    commands.run_system_cached_with(scroll, (scrollable, dy));
    Ok(())
}

/// Scrolls the `scrollable` node by the given amount as well as its [`Scrollbar`] accordingly.
///
/// Helper function of [`scroll_on_drag`] and [`scroll_on_wheel`](super::scrollable::scroll_on_wheel).
pub(super) fn scroll(
    In((scrollable, dy)): In<(Entity, f32)>,
    mut q_scrollable: Query<(&mut ScrollPosition, &ComputedNode, &Scrollable)>,
    q_scrollbar: Query<(&ComputedNode, &Children)>,
    mut q_thumb: Query<(&mut Node, &ComputedNode)>,
) {
    let (mut scrollable_position, scrollable_cnode, scrollable) =
        q_scrollable.get_mut(scrollable).unwrap();
    scrollable_position.offset_y -= dy;

    let (track_cnode, children) = q_scrollbar.get(scrollable.scrollbar()).unwrap();
    let (mut thumb_node, thumb_cnode) = q_thumb.get_mut(children[0]).unwrap();
    let track_cnode_scroll_height = track_cnode.size.y
        - (track_cnode.border.top + track_cnode.border.bottom + thumb_cnode.size.y);
    let scrollable_cnode_scroll_height = scrollable_cnode.size.y - scrollable_cnode.content_size.y;
    let ratio = track_cnode_scroll_height / scrollable_cnode_scroll_height;
    let track_dy = ratio * dy;
    let top = &mut thumb_node.margin.top;
    let top_max = track_cnode.inverse_scale_factor * track_cnode_scroll_height;
    *top = Val::Px(0f32.max(top_max.min(px(*top) + track_dy)));
    debug!("track computed height: {}", track_cnode.size.y);
    debug!("thumb computed height: {}", thumb_cnode.size.y);
    debug!("track computed scroll height: {track_cnode_scroll_height}");
    debug!(
        "scaled margin top: {}\n",
        px(*top) / track_cnode.inverse_scale_factor
    );
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
