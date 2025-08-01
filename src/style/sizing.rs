use crate::style::layout::Direction;

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
    pub fn is_fixed(&self) -> bool {
        matches!(self.preferred, PreferredSize::Fixed(_))
    }

    pub fn is_growable(&self) -> bool {
        matches!(self.preferred, PreferredSize::Grow(_))
    }

    pub fn is_shrinkable(&self) -> bool {
        self.max > self.min // Allow shrinking if not fixed
    }

    pub fn get_grow_factor(&self) -> f32 {
        match self.preferred {
            PreferredSize::Grow(factor) => factor,
            _ => 0.0, // Default grow factor if not specified
        }
    }

    pub fn clamped(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }
}

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

#[macro_export]
macro_rules! sizing {
    ($width:expr, $height:expr) => {
        kaolin::style::sizing::BoxSizing {
            width: $width,
            height: $height,
        }
    };

    ($size:expr) => {
        kaolin::style::sizing::BoxSizing {
            width: $size,
            height: $size,
        }
    };
    (width: $width:expr, height: $height:expr) => {
        kaolin::style::sizing::BoxSizing {
            width: $width,
            height: $height,
        }
    };
    ($key:ident : $value:expr$(,)?) => {
        kaolin::style::sizing::BoxSizing {
            $key: $value,
            ..kaolin::style::sizing::BoxSizing::default()
        }
    };
    ($($key:ident : $value:expr),* $(,)?) => {
        kaolin::style::sizing::BoxSizing {
            $($key: $value,)*
        }
    };
    () => {};
}

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

#[macro_export]
macro_rules! fit {
    ($min:expr, $max:expr) => {
        kaolin::style::sizing::Sizing::Fit {
            min: Some($min),
            max: Some($max),
        }
    };

    ($max:expr) => {
        kaolin::style::sizing::Sizing::Fit {
            min: None,
            max: Some($max),
        }
    };

    () => {
        kaolin::style::sizing::Sizing::Fit {
            min: None,
            max: None,
        }
    };
}

#[macro_export]
macro_rules! fixed {
    ($size:expr) => {
        kaolin::style::sizing::Sizing::Fixed($size)
    };
}

#[macro_export]
macro_rules! grow {
    ($factor:expr, $min:expr, $max:expr) => {
        kaolin::style::sizing::Sizing::Grow {
            factor: Some($factor),
            min: Some($min),
            max: Some($max),
        }
    };

    ($factor:expr) => {
        kaolin::style::sizing::Sizing::Grow {
            factor: Some($factor),
            min: None,
            max: None,
        }
    };

    () => {
        kaolin::style::sizing::Sizing::Grow {
            factor: None,
            min: None,
            max: None,
        }
    };
}
