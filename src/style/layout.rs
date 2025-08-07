use derive_default_constructor::DefaultConstructor;
use derive_setters::Setters;

#[derive(Default, Clone, Copy)]
pub enum Direction {
    #[default]
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop,
}

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
    #[default]
    Start,
    End,
    Center,
    Stretch,
}

#[derive(Default, Clone, Copy)]
pub enum Justification {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
}

#[derive(Default, DefaultConstructor, Clone, Copy, Setters)]
pub struct Layout {
    pub direction: Direction,
    pub alignment: Alignment,
    pub justification: Justification,
    pub gap: f32, // Gap between items
}
