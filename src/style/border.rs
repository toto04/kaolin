use derive_default_constructor::DefaultConstructor;
use derive_setters::Setters;

/// Border for the flex container.
#[derive(Debug, Clone, Copy, PartialEq, Setters, DefaultConstructor)]
pub struct Border<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    width: f32,
    color: Color,
}

impl<Color> Default for Border<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    fn default() -> Self {
        Border {
            width: 0.0,
            color: Color::default_foreground_color(),
        }
    }
}
