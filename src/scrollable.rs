use bevy::{prelude::*, text::LineHeight};

use crate::Scrollbar;

/// Component of a `Node` with overflowing content linking it to a [`Scrollbar`].
///
/// Adding this component to an entity makes it the `RelationshipTarget` of a [`Scrollbar`] entity. Despawning this entity will also despawn that [`Scrollbar`] entity. See [`Scrollbar`] for more information.
///
/// Note: As `Children`, this component is not inserted directly. It is
/// * either automatically inserted when you spawn a [`Scrollbar`] (see [example 1](crate#example-1));
/// * or inserted via `SpawnRelated::spawn_one` (see [example 2](crate#example-2)).
#[derive(Component, Clone, Reflect, Debug)]
#[relationship_target(relationship = Scrollbar, linked_spawn)]
#[require(Node, ScrollableScrollScale)]
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

/// Component of a [`Scrollable`] node configuring how fast its content scrolls when scrolling the mouse.
///
/// This is unrelated to how fast the content scrolls when dragging the thumb of the [`Scrollbar`]. See [`ThumbDragScale`](super::ThumbDragScale) for that.
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

/// Component of a [`Scrollable`] node used to compute line height for mouse scroll.
///
/// Only used by vertical [`Scrollbar`]s using `MouseScrollUnit::Line`.
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
    pub(super) fn px(&self) -> f32 {
        match self.line_height {
            LineHeight::Px(px) => px,
            LineHeight::RelativeToFont(scale) => scale * self.font_size,
        }
    }
}
