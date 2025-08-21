use derive_default_constructor::DefaultConstructor;
use derive_setters::Setters;

#[derive(Default, Clone, Copy)]
pub enum Direction {
    /// Default. Lays out elements from left to right.
    #[default]
    LeftToRight,
    /// Lays out elements from top to bottom.
    TopToBottom,
    /// Lays out elements from right to left.
    RightToLeft,
    /// Lays out elements from bottom to top.
    BottomToTop,
}

/// Shorthand for layout directions.
///
/// - `direction!(ltr)` for Left to Right
/// - `direction!(ttb)` for Top to Bottom
/// - `direction!(rtl)` for Right to Left
/// - `direction!(btt)` for Bottom to Top
#[macro_export]
macro_rules! direction {
    (ltr) => {
        $crate::style::layout::Direction::LeftToRight
    };
    (ttb) => {
        $crate::style::layout::Direction::TopToBottom
    };
    (rtl) => {
        $crate::style::layout::Direction::RightToLeft
    };
    (btt) => {
        $crate::style::layout::Direction::BottomToTop
    };
}

#[derive(Default, Clone, Copy)]
pub enum Alignment {
    /// Aligns the elements at the start of the cross axis (top with horizontal layout, left with vertical layout).
    #[default]
    Start,
    /// Aligns the elements at the end of the cross axis (bottom with horizontal layout, right with vertical layout).
    End,
    /// Aligns the elements at the center of the cross axis.
    Center,
    /// Stretches the elements to fill the cross axis.
    Stretch,
}

#[derive(Default, Clone, Copy)]
pub enum Justification {
    /// Aligns the elements at the start of the main axis.
    #[default]
    Start,
    /// Aligns the elements at the end of the main axis.
    End,
    /// Aligns the elements at the center of the main axis.
    Center,
    /// Distributes the elements evenly, with the first element at the start and the last at the end.
    SpaceBetween,
    /// Distributes the elements evenly, with equal space around them.
    SpaceAround,
}

/// Layout properties for the flex container (a.k.a, where do we put its children?)
#[derive(Default, DefaultConstructor, Clone, Copy, Setters)]
pub struct Layout {
    /// The direction in which child elements are laid out.
    pub direction: Direction,
    /// The alignment of child elements along the cross axis.
    pub alignment: Alignment,
    /// The justification of child elements along the main axis.
    pub justification: Justification,
    /// The gap between child elements.
    pub gap: f64,
}
