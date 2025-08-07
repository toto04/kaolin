#[derive(Default, Clone, Copy)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Padding {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn all(value: f32) -> Self {
        Padding {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn horizontal(value: f32) -> Self {
        Padding {
            left: value,
            right: value,
            top: 0.0,
            bottom: 0.0,
        }
    }

    pub fn vertical(value: f32) -> Self {
        Padding {
            left: 0.0,
            right: 0.0,
            top: value,
            bottom: value,
        }
    }

    pub fn hor_ver(hor: f32, ver: f32) -> Self {
        Padding {
            left: hor,
            right: hor,
            top: ver,
            bottom: ver,
        }
    }

    pub fn left(value: f32) -> Self {
        Padding {
            left: value,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }

    pub fn right(value: f32) -> Self {
        Padding {
            left: 0.0,
            right: value,
            top: 0.0,
            bottom: 0.0,
        }
    }

    pub fn top(value: f32) -> Self {
        Padding {
            left: 0.0,
            right: 0.0,
            top: value,
            bottom: 0.0,
        }
    }

    pub fn bottom(value: f32) -> Self {
        Padding {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: value,
        }
    }

    pub fn x(&self) -> f32 {
        self.left + self.right
    }

    pub fn y(&self) -> f32 {
        self.top + self.bottom
    }
}
