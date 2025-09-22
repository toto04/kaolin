use derive_setters::Setters;

/// Border for the flex container.
#[derive(Debug, Clone, Copy, PartialEq, Setters)]
pub struct Border<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    pub width: f32,
    pub color: Color,
}

impl<Color> Default for Border<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    fn default() -> Self {
        Border {
            width: 0.0,
            color: Color::default_foreground_color(),
        }
    }
}

impl<Color> Border<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// alias for `Border::default()`
    pub fn new() -> Self {
        Border::default()
    }
}
