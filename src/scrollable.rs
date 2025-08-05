use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    input::mouse::MouseScrollUnit,
    prelude::*,
    text::LineHeight,
};

use crate::{scrollbar, Scrollbar};

/// Component of a scrollable node, which usually has children overflowing its content.
///
/// Adding this component to an entity will:
/// * add the `Node` component if it's not already present;
/// * set its `Node::overflow` to `Overflow::scroll_y`;
/// * set up this node as the `RelationshipTarget` of a [`Scrollbar`] node;
/// * have it watched by an observer for `Scroll` triggers.
///
/// Note: As `Children`, this component is not inserted directly. See [example 2](crate#example-2) for more information.
#[derive(Component, Clone, Debug)]
#[relationship_target(relationship = Scrollbar, linked_spawn)]
#[require(Node, ScrollableSettings)]
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

/// Settings of a [`Scrollable`] node.
///
/// Add this component to a [`Scrollable`] node to configure its `scroll speed`.
#[derive(Component, Copy, Clone, Debug)]
pub struct ScrollableSettings {
    /// How many pixels the [`Scrollable`] node should move per mouse pixel scrolled.
    pub scroll_speed: f32,
    /// Only used to compute mouse pixel scroll and only if `MouseScrollUnit::Line` is used.
    pub font_size: f32,
    /// Only used to compute mouse pixel scroll and only if `MouseScrollUnit::Line` is used.
    pub line_height: LineHeight,
}

impl Default for ScrollableSettings {
    fn default() -> Self {
        Self {
            scroll_speed: Self::SCROLL_SPEED_DEFAULT,
            font_size: TextFont::default().font_size,
            line_height: LineHeight::default(),
        }
    }
}

impl ScrollableSettings {
    /// Default value of [`scroll_speed`](Self::scroll_speed).
    pub const SCROLL_SPEED_DEFAULT: f32 = 1.0;

    /// Returns the number of pixels in the height of a line.
    fn line_height_px(&self) -> f32 {
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
    q_settings: Query<&ScrollableSettings>,
    mut commands: Commands,
) -> Result {
    let settings = q_settings.get(scroll.target())?;
    let dy = settings.scroll_speed
        * match scroll.unit {
            MouseScrollUnit::Line => scroll.y * settings.line_height_px(),
            MouseScrollUnit::Pixel => scroll.y,
        };
    commands.run_system_cached_with(scrollbar::scroll, (scroll.target(), dy));
    Ok(())
}
