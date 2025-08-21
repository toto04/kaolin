use crate::style::layout::Direction;

/// Represents the preferred sizing behavior for a specific Dimension.
#[derive(Clone, Copy, Debug)]
pub enum PreferredSize {
    // gravitates towards a fixed size
    Fixed(f32),
    // no fixed size, grows indefenetly with a factor
    Grow(f32),
}

impl Default for PreferredSize {
    fn default() -> Self {
        PreferredSize::Fixed(0.0) // gravitate to 0.0 for fit sizing
    }
}

/// Represents the sizing information for a UI element used while calculating layout.
#[derive(Clone, Copy, Debug)]
pub struct SizingDimensions {
    pub min: f32,                 // Minimum size
    pub preferred: PreferredSize, // Preferred size
    pub max: f32,                 // Maximum size
}

impl Default for SizingDimensions {
    fn default() -> Self {
        SizingDimensions {
            min: 0.0,
            preferred: PreferredSize::default(),
            max: f32::INFINITY,
        }
    }
}

impl SizingDimensions {
    /// True if the sizing is fixed and layout calculations will not change it.
    pub fn is_fixed(&self) -> bool {
        matches!(self.preferred, PreferredSize::Fixed(_))
    }

    /// True if the sizing is growable and can expand to fill available space.
    pub fn is_growable(&self) -> bool {
        matches!(self.preferred, PreferredSize::Grow(_))
    }

    /// True if the sizing is shrinkable and can be reduced in size when
    /// overflowing its container.
    pub fn is_shrinkable(&self) -> bool {
        self.max > self.min // Allow shrinking if not fixed
    }

    /// Returns the growth factor for the dimension.
    pub fn get_grow_factor(&self) -> f32 {
        match self.preferred {
            PreferredSize::Grow(factor) => factor,
            _ => 0.0, // Default grow factor if not specified
        }
    }

    /// Returns the dimension value clamped between min and max.
    pub fn clamped(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }
}

/// Represents the sizing behavior of a box.
#[derive(Default, Clone, Copy)]
pub struct BoxSizing {
    pub width: Sizing,
    pub height: Sizing,
}

impl BoxSizing {
    pub fn main(&mut self, dir: Direction) -> &mut Sizing {
        match dir {
            Direction::LeftToRight | Direction::RightToLeft => &mut self.width,
            Direction::TopToBottom | Direction::BottomToTop => &mut self.height,
        }
    }

    pub fn cross(&mut self, dir: Direction) -> &mut Sizing {
        match dir {
            Direction::LeftToRight | Direction::RightToLeft => &mut self.height,
            Direction::TopToBottom | Direction::BottomToTop => &mut self.width,
        }
    }
}

/// Defines the sizing behavior for a flex box.
///
/// - `sizing!(width, height)` will create a box sizing with the specified width and height behaviors.
/// - `sizing!(size)` will create a box sizing with the same behavior for both axes.
/// - `sizing!()` will define a box sizing behavior with default values (fit for both axes).
///
/// You can also use `sizing!(key: value)` to specify width or height behavior individually (where `key` is either `width` or `height`, duh).
///
/// Example:
/// ```ignore
/// // grow both width and height
/// FlexStyle::new()
///     .sizing(sizing!(grow!())),
///
/// // fit width (max 100.0) and fixed height (200.0)
/// FlexStyle::new()
///     .sizing(sizing!(fit!(100.0), fixed!(200.0))),
/// ```
#[macro_export]
macro_rules! sizing {
    ($width:expr, $height:expr) => {
        $crate::style::sizing::BoxSizing {
            width: $width,
            height: $height,
        }
    };

    ($size:expr) => {
        $crate::style::sizing::BoxSizing {
            width: $size,
            height: $size,
        }
    };
    (width: $width:expr, height: $height:expr) => {
        $crate::style::sizing::BoxSizing {
            width: $width,
            height: $height,
        }
    };
    ($key:ident : $value:expr$(,)?) => {
        $crate::style::sizing::BoxSizing {
            $key: $value,
            ..$crate::style::sizing::BoxSizing::default()
        }
    };
    ($($key:ident : $value:expr),* $(,)?) => {
        $crate::style::sizing::BoxSizing {
            $($key: $value,)*
        }
    };
    () => {
        $crate::style::sizing::BoxSizing::default()
    };
}

/// Represents the Sizing behavior for a dimension.
#[derive(Default, Clone, Copy)]
pub enum Sizing {
    #[default]
    Default,
    Fit {
        min: Option<f32>,
        max: Option<f32>,
    },
    Fixed(f32),
    Grow {
        factor: Option<f32>, // Growth factor
        min: Option<f32>,
        max: Option<f32>,
    },
}

impl From<Sizing> for SizingDimensions {
    fn from(sizing: Sizing) -> Self {
        match sizing {
            Sizing::Default => SizingDimensions::default(), // is actually just FIT with no limits
            Sizing::Fit { min, max } => SizingDimensions {
                min: min.unwrap_or(0.0),
                preferred: PreferredSize::Fixed(min.unwrap_or(0.0)), // prefers to stay at the min i guess
                max: max.unwrap_or(f32::INFINITY),
            },
            Sizing::Fixed(size) => SizingDimensions {
                min: size,
                preferred: PreferredSize::Fixed(size),
                max: size,
            },
            Sizing::Grow { factor, min, max } => SizingDimensions {
                min: min.unwrap_or(0.0),
                preferred: PreferredSize::Grow(factor.unwrap_or(1.0)),
                max: max.unwrap_or(f32::INFINITY),
            },
        }
    }
}

/// Defines a fit sizing behavior.
///
/// - `fit!()` will create a fit behavior with no constraints.
/// - `fit!(max)` will be interpreted as the maximum size, with no minimum constraint.
/// - `fit!(min, max)` will set both the minimum and maximum size.
#[macro_export]
macro_rules! fit {
    ($min:expr, $max:expr) => {
        $crate::style::sizing::Sizing::Fit {
            min: Some($min),
            max: Some($max),
        }
    };

    ($max:expr) => {
        $crate::style::sizing::Sizing::Fit {
            min: None,
            max: Some($max),
        }
    };

    () => {
        $crate::style::sizing::Sizing::Fit {
            min: None,
            max: None,
        }
    };
}

/// Defines a fixed sizing behavior.
///
/// - `fixed!(size)` will define a fixed size. It's fixed. The constraints are that it's fixed.
#[macro_export]
macro_rules! fixed {
    ($size:expr) => {
        $crate::style::sizing::Sizing::Fixed($size)
    };
}

/// Defines a grow sizing behavior.
///
/// - `grow!()` will create a grow behavior with a default factor of `1.0`.
/// - `grow!(factor)` will create a grow behavior with the specified factor, and
///   no constraints. Different factors are useful for giving different
///   proportions of space to sibling growable elements.
/// - `grow!(factor, min, max)` will create a grow behavior with the specified constraints.
#[macro_export]
macro_rules! grow {
    ($factor:expr, $min:expr, $max:expr) => {
        $crate::style::sizing::Sizing::Grow {
            factor: Some($factor),
            min: Some($min),
            max: Some($max),
        }
    };

    ($factor:expr) => {
        $crate::style::sizing::Sizing::Grow {
            factor: Some($factor),
            min: None,
            max: None,
        }
    };

    () => {
        $crate::style::sizing::Sizing::Grow {
            factor: None,
            min: None,
            max: None,
        }
    };
}
