use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    input::mouse::MouseScrollUnit,
    prelude::*,
    text::LineHeight,
};

use crate::{scrollbar, Scrollbar};

/// Component of a scrollable node, which usually has overflowing content.
///
/// Adding this component to an entity will:
/// * add the `Node` component if it's not already present;
/// * set its `Node::overflow` to `Overflow::scroll_y`;
/// * set up this node as the `RelationshipTarget` of a [`Scrollbar`] node;
/// * have it watched by an observer for `Scroll` triggers.
///
/// Note: As `Children`, this component is not inserted directly. It is
/// * either automatically inserted when you spawn a [`Scrollbar`] (see [example 1](crate#example-1));
/// * or inserted via `SpawnRelated::spawn_one` (see [example 2](crate#example-2)).
#[derive(Component, Clone, Reflect, Debug)]
#[relationship_target(relationship = Scrollbar, linked_spawn)]
#[require(Node, ScrollableScrollScale, ScrollableLineHeight)]
#[component(on_add = configure_overflow_and_wheel_scroll)]
pub struct Scrollable {
    /// The [`Scrollbar`] entity of this scrollable entity.
    scrollbar: Entity,
}

impl Scrollable {
    /// Gets the [`Scrollbar`] entity of this scrollable entity.
    pub fn scrollbar(&self) -> Entity {
        self.scrollbar
    }
}

/// How many pixels the [`Scrollable`] node should move per mouse pixel scrolled.
#[derive(Component, Copy, Clone, Reflect, Debug)]
pub struct ScrollableScrollScale(pub f32);

impl Default for ScrollableScrollScale {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl ScrollableScrollScale {
    /// Default value of [`ScrollableScrollScale`].
    pub const DEFAULT: f32 = 1.0;
}

/// Used to compute line height for mouse pixel scroll. This is only used by vertical [`Scrollbar`]s using `MouseScrollUnit::Line`.
#[derive(Component, Copy, Clone, Reflect, Debug)]
pub struct ScrollableLineHeight {
    /// Font size.
    pub font_size: f32,
    /// Line height.
    pub line_height: LineHeight,
}

impl Default for ScrollableLineHeight {
    fn default() -> Self {
        Self {
            font_size: TextFont::default().font_size,
            line_height: LineHeight::default(),
        }
    }
}

impl ScrollableLineHeight {
    /// Returns the number of pixels in the height of a line.
    fn px(&self) -> f32 {
        match self.line_height {
            LineHeight::Px(px) => px,
            LineHeight::RelativeToFont(scale) => scale * self.font_size,
        }
    }
}

/// `on_add` hook of [`Scrollable`].
fn configure_overflow_and_wheel_scroll(
    mut world: DeferredWorld,
    HookContext { entity, .. }: HookContext,
) {
    world.commands().queue(
        move |world: &mut World| match world.get_entity_mut(entity) {
            Ok(mut entity) => {
                entity.get_mut::<Node>().unwrap().overflow = Overflow::scroll_y();
                entity.observe(scroll_on_wheel);
            }
            Err(err) => warn!(
                "Scrollable on_add hook could not configure entity {}. {err}",
                entity.index()
            ),
        },
    );
}

/// Observer watching a [`Scrollable`] node for `Scroll` triggers.
pub(super) fn scroll_on_wheel(
    scroll: Trigger<Pointer<Scroll>>,
    q_scroll_scale: Query<&ScrollableScrollScale>,
    q_line_height: Query<&ScrollableLineHeight>,
    mut commands: Commands,
) -> Result {
    let scrollable = scroll.target();
    let mouse_scroll = match scroll.unit {
        MouseScrollUnit::Line => {
            let line_height = q_line_height.get(scrollable)?;
            scroll.y * line_height.px()
        }
        MouseScrollUnit::Pixel => scroll.y,
    };
    let scroll_scale = q_scroll_scale.get(scrollable)?;
    let scroll = scroll_scale.0 * mouse_scroll;
    commands.run_system_cached_with(scrollbar::scroll, (scrollable, scroll));
    Ok(())
}
