use derive_setters::Setters;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

impl Direction {
    /// Returns true if the direction is horizontal (LeftToRight or RightToLeft).
    #[inline]
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Direction::LeftToRight | Direction::RightToLeft)
    }
}

// /// Shorthand for layout directions.
// ///
// /// - `Layout::new().direction!(ltr)` for Left to Right
// /// - `Layout::new().direction!(ttb)` for Top to Bottom
// /// - `Layout::new().direction!(rtl)` for Right to Left
// /// - `Layout::new().direction!(btt)` for Bottom to Top
// #[macro_export]
// macro_rules! direction {
//     (ltr) => {
//         direction($crate::style::layout::Direction::LeftToRight)
//     };
//     (ttb) => {
//         direction($crate::style::layout::Direction::TopToBottom)
//     };
//     (rtl) => {
//         direction($crate::style::layout::Direction::RightToLeft)
//     };
//     (btt) => {
//         direction($crate::style::layout::Direction::BottomToTop)
//     };
// }

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
#[derive(Default, Clone, Copy, Setters)]
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

impl Layout {
    /// alias for `Layout::default()`
    pub fn new() -> Self {
        Layout::default()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::direction;

//     #[test]
//     fn test_layout_direction() {
//         let layout = Layout::new().direction!(ltr);
//         assert_eq!(layout.direction, Direction::LeftToRight);
//     }

//     #[test]
//     fn test_layout_alignment() {
//         let layout = Layout::new().alignment!(center);
//         assert_eq!(layout.alignment, Alignment::Center);
//     }

//     #[test]
//     fn test_layout_justification() {
//         let layout = Layout::new().justification!(space_between);
//         assert_eq!(layout.justification, Justification::SpaceBetween);
//     }

//     #[test]
//     fn test_layout_gap() {
//         let layout = Layout::new().gap!(10.0);
//         assert_eq!(layout.gap, 10.0);
//     }
// }
