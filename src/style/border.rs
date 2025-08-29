use derive_default_constructor::DefaultConstructor;
use derive_setters::Setters;

/// Border for the flex container.
#[derive(Debug, Clone, Copy, PartialEq, Setters, DefaultConstructor)]
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
