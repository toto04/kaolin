#[derive(Default, Clone, Copy)]
pub enum Direction {
    #[default]
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop,
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

#[derive(Default, Clone, Copy)]
pub struct Layout {
    pub direction: Direction,
    pub alignment: Alignment,
    pub justification: Justification,
    pub gap: f32, // Gap between items
}
