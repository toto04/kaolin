/// Represents the padding values for a UI element.
///
/// Padding can be set individually for each side (left, right, top, bottom).
///
/// This struct implements different constructors for ease of use.
///
/// For more info on Styling, see [`kaolin::style`]
#[derive(Default, Clone, Copy)]
pub struct Padding {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

impl Padding {
    /// Creates a new `Padding` instance with the specified values.
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Self {
        Padding {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Creates a new `Padding` instance with the same value for all sides.
    pub fn all(value: f64) -> Self {
        Padding {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    /// Creates a new `Padding` instance with the same value for the horizontal sides (left and right).
    pub fn horizontal(value: f64) -> Self {
        Padding {
            left: value,
            right: value,
            top: 0.0,
            bottom: 0.0,
        }
    }

    /// Creates a new `Padding` instance with the same value for the vertical sides (top and bottom).
    pub fn vertical(value: f64) -> Self {
        Padding {
            left: 0.0,
            right: 0.0,
            top: value,
            bottom: value,
        }
    }

    /// Creates a new `Padding` with the same value for the horizontal sides (left and right) and the vertical sides (top and bottom).
    pub fn hor_ver(hor: f64, ver: f64) -> Self {
        Padding {
            left: hor,
            right: hor,
            top: ver,
            bottom: ver,
        }
    }

    /// Creates a new `Padding` with only the left value set, all others sides set to 0.
    pub fn left(value: f64) -> Self {
        Padding {
            left: value,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }

    /// Creates a new `Padding` with only the right value set, all others sides set to 0.
    pub fn right(value: f64) -> Self {
        Padding {
            left: 0.0,
            right: value,
            top: 0.0,
            bottom: 0.0,
        }
    }

    /// Creates a new `Padding` with only the top value set, all others sides set to 0.
    pub fn top(value: f64) -> Self {
        Padding {
            left: 0.0,
            right: 0.0,
            top: value,
            bottom: 0.0,
        }
    }

    /// Creates a new `Padding` with only the bottom value set, all others sides set to 0.
    pub fn bottom(value: f64) -> Self {
        Padding {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: value,
        }
    }

    /// Returns the total horizontal padding (left + right). Used internally to calculate layout positions.
    pub fn x(&self) -> f64 {
        self.left + self.right
    }

    /// Returns the total vertical padding (top + bottom). Used internally to calculate layout positions.
    pub fn y(&self) -> f64 {
        self.top + self.bottom
    }
}
