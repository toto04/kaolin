use typed_floats::tf64::{Positive, PositiveFinite};

use crate::style::layout::Direction;

/// Represents the preferred sizing behavior for a specific Dimension.
#[derive(Clone, Copy, Debug)]
pub enum PreferredSize {
    // gravitates towards a fixed size
    Fixed(PositiveFinite),
    // no fixed size, grows indefenetly with a factor
    Grow(PositiveFinite),
}

impl Default for PreferredSize {
    fn default() -> Self {
        PreferredSize::Fixed(PositiveFinite::new(0.0).unwrap()) // gravitate to 0.0 for fit sizing
    }
}

/// Represents the sizing information for a UI element used while calculating layout.
#[derive(Clone, Copy, Debug)]
pub struct SizingDimensions {
    pub min: PositiveFinite,      // Minimum size
    pub preferred: PreferredSize, // Preferred size
    pub max: Positive,            // Maximum size
}

impl Default for SizingDimensions {
    fn default() -> Self {
        SizingDimensions {
            min: PositiveFinite::new(0.0).unwrap(),
            preferred: PreferredSize::default(),
            max: Positive::new(f64::INFINITY).unwrap(), // No maximum limit
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
    pub fn get_grow_factor(&self) -> f64 {
        match self.preferred {
            PreferredSize::Grow(factor) => factor.into(),
            _ => 0.0, // Default grow factor if not specified
        }
    }

    /// Returns the dimension value clamped between min and max.
    ///
    /// ```
    /// # use typed_floats::tf64::{Positive, PositiveFinite};
    /// # use kaolin::style::sizing::{SizingDimensions, PreferredSize};
    /// let sized = SizingDimensions {
    ///     min: PositiveFinite::new(100.0).unwrap(),
    ///     preferred: PreferredSize::Fixed(PositiveFinite::new(200.0).unwrap()),
    ///     max: Positive::new(300.0).unwrap(),
    /// };
    /// assert_eq!(sized.clamped(250.0), 250.0);
    /// assert_eq!(sized.clamped(50.0), 100.0);
    /// assert_eq!(sized.clamped(350.0), 300.0);
    ///
    /// assert_eq!(sized.clamped(f64::NAN), 100.0); // NaN defaults to min
    /// assert_eq!(sized.clamped(f64::INFINITY), 300.0);
    /// assert_eq!(sized.clamped(f64::NEG_INFINITY), 100.0);
    /// ```
    pub fn clamped(&self, value: f64) -> f64 {
        if value.is_nan() {
            return self.min.into(); // Default to min if value is NaN
        }
        value.clamp(self.min.into(), self.max.into())
    }

    pub fn max(&self) -> f64 {
        self.max.into()
    }

    pub fn min(&self) -> f64 {
        self.min.into()
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
        min: Option<PositiveFinite>,
        max: Option<Positive>,
    },
    Fixed(PositiveFinite),
    Grow {
        factor: Option<PositiveFinite>, // Growth factor
        min: Option<PositiveFinite>,
        max: Option<Positive>,
    },
}

impl From<Sizing> for SizingDimensions {
    fn from(sizing: Sizing) -> Self {
        match sizing {
            Sizing::Default => SizingDimensions::default(), // is actually just FIT with no limits
            Sizing::Fit { min, max } => SizingDimensions {
                min: min.unwrap_or_default(),
                preferred: PreferredSize::Fixed(min.unwrap_or_default()), // prefers to stay at the min i guess
                max: max.unwrap_or(Positive::new(f64::INFINITY).unwrap()),
            },
            Sizing::Fixed(size) => SizingDimensions {
                min: size,
                preferred: PreferredSize::Fixed(size),
                max: size.into(),
            },
            Sizing::Grow { factor, min, max } => SizingDimensions {
                min: min.unwrap_or_default(),
                preferred: PreferredSize::Grow(factor.unwrap_or(PositiveFinite::new(1.0).unwrap())),
                max: max.unwrap_or(Positive::new(f64::INFINITY).unwrap()),
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
            min: Some(typed_floats::tf64::PositiveFinite::new($min).unwrap()),
            max: Some(typed_floats::tf64::Positive::new($max).unwrap()),
        }
    };

    ($max:expr) => {
        $crate::style::sizing::Sizing::Fit {
            min: None,
            max: Some(typed_floats::tf64::Positive::new($max).unwrap()),
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
        $crate::style::sizing::Sizing::Fixed(
            typed_floats::tf64::PositiveFinite::new($size).unwrap(),
        )
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
            factor: Some(typed_floats::tf64::PositiveFinite::new($factor).unwrap()),
            min: Some(typed_floats::tf64::PositiveFinite::new($min).unwrap()),
            max: Some(typed_floats::tf64::Positive::new($max).unwrap()),
        }
    };

    ($factor:expr) => {
        $crate::style::sizing::Sizing::Grow {
            factor: Some(typed_floats::tf64::PositiveFinite::new($factor).unwrap()),
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
