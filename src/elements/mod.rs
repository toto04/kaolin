pub mod flexbox;
pub mod text;

use flexbox::FlexBox;
use text::TextElement;
use typed_floats::tf64::Positive;

use crate::{
    commands::RenderCommand,
    style::sizing::{PreferredSize, SizingDimensions},
};
use uuid::Uuid;

pub enum KaolinElement<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    Flex(FlexBox<'frame, Color>),
    Text(TextElement<'frame, Color>),
}

pub(crate) struct KaolinNode<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    id: String,
    growable_width: bool,
    growable_height: bool,
    shrinkable: bool,
    element: KaolinElement<'frame, Color>,
    sizing: (SizingDimensions, SizingDimensions),
    current_width: f64,
    current_height: f64,
    x: f64,
    y: f64,
}

impl<'frame, Color> KaolinNode<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    fn new(element: KaolinElement<'frame, Color>, id: Option<String>) -> Self {
        let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        match &element {
            KaolinElement::Flex(flex_box) => {
                let style = &flex_box.style;
                let width = SizingDimensions::from(style.sizing.width);
                let height = SizingDimensions::from(style.sizing.height);

                KaolinNode {
                    id,
                    growable_width: width.is_growable(),
                    growable_height: height.is_growable(),
                    shrinkable: width.is_shrinkable(),
                    current_width: width.clamped(flex_box.fit_width_to_children()),
                    current_height: height.min.into(),
                    sizing: (width, height),
                    element,
                    x: 0.0,
                    y: 0.0,
                }
            }
            KaolinElement::Text(text_element) => {
                let (pref_width, pref_height) = text_element.get_preferred_size();
                let width = SizingDimensions {
                    min: text_element.get_minimum_size().0,
                    preferred: PreferredSize::Fixed(pref_width),
                    max: pref_width.into(),
                };
                let height = SizingDimensions {
                    min: pref_height,
                    preferred: PreferredSize::Fixed(pref_height),
                    max: Positive::new(f64::INFINITY).unwrap(),
                };

                KaolinNode {
                    id,
                    growable_width: false,
                    growable_height: false,
                    shrinkable: true,
                    element,
                    current_width: pref_width.into(),
                    current_height: pref_height.into(),
                    sizing: (width, height),
                    x: 0.0,
                    y: 0.0,
                }
            }
        }
    }

    /// Returns the growth factors for calculations during the growing phase.
    pub fn get_grow_factor(&self) -> (f64, f64) {
        let w_factor = match self.sizing.0.preferred {
            PreferredSize::Grow(factor) => factor.into(),
            _ => 0.0, // Default grow factor if not specified
        };
        let h_factor = match self.sizing.1.preferred {
            PreferredSize::Grow(factor) => factor.into(),
            _ => 0.0, // Default grow factor if not specified
        };
        (w_factor, h_factor)
    }

    /// Grows (or shrinks) the width of the element within the given limit.
    fn grow_width(&mut self, limit: f64) -> f64 {
        match self.element {
            KaolinElement::Flex(..) => {
                let grown;
                if limit > self.sizing.0.max() - self.current_width {
                    grown = self.sizing.0.max() - self.current_width;
                    self.growable_width = false; // No more growth possible
                } else if limit < self.sizing.0.min() - self.current_width {
                    grown = self.sizing.0.min() - self.current_width;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    grown = limit;
                }

                self.current_width += grown;
                self.current_width = self.current_width.min(self.sizing.0.max.into());
                grown
            }
            KaolinElement::Text(..) => {
                let shrunk;
                if limit > 0.0 {
                    shrunk = 0.0; // Text elements do not grow
                } else if limit < self.sizing.0.min() - self.current_width {
                    shrunk = self.sizing.0.min() - self.current_width;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    shrunk = limit;
                }
                self.current_width += shrunk;
                shrunk
            }
        }
    }

    /// Grows (or shrinks) the height of the element within the given limit.
    fn grow_height(&mut self, limit: f64) -> f64 {
        match self.element {
            KaolinElement::Flex(..) => {
                let grown;
                if limit > self.sizing.1.max() - self.current_height {
                    grown = self.sizing.1.max() - self.current_height;
                    self.growable_height = false; // No more growth possible
                } else if limit < self.sizing.1.min() - self.current_height {
                    grown = self.sizing.1.min() - self.current_height;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    grown = limit;
                }

                self.current_height += grown;
                self.current_height = self.current_height.min(self.sizing.1.max());
                grown
            }
            KaolinElement::Text(..) => {
                let shrunk;
                if limit > 0.0 {
                    shrunk = 0.0; // Text elements do not grow
                } else if limit < self.sizing.1.min() - self.current_height {
                    shrunk = self.sizing.1.min() - self.current_height;
                    self.shrinkable = false; // No more shrinking possible
                } else {
                    shrunk = limit;
                }
                self.current_height += shrunk;
                shrunk
            }
        }
    }

    /// Fits the height of the element to its content.
    fn fit_height(&mut self) {
        let height = self.sizing.1;
        if let KaolinElement::Flex(ref mut flex_box) = self.element {
            self.current_height = height
                .clamped(flex_box.fit_height_to_children(self.current_height, height.max.into()));
        }
    }

    /// Sets the position of the element, and propagates the change to its children.
    /// This is called after all sizing calculations are complete.
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        if let KaolinElement::Flex(ref mut flex_box) = self.element {
            flex_box.position_children((x, x + self.current_width, y, y + self.current_height));
        }
    }

    /// Renders the element and its children into a series of rendering commands.
    pub fn render(
        &self,
        inherited_color: Color,
    ) -> Box<dyn Iterator<Item = RenderCommand<Color>> + '_> {
        match &self.element {
            KaolinElement::Flex(flex_box) => Box::new(
                flex_box.render(
                    RenderCommand::DrawRectangle {
                        id: self.id.clone(),
                        x: self.x,
                        y: self.y,
                        width: self.current_width,
                        height: self.current_height,
                        color: flex_box
                            .style
                            .background_color
                            .unwrap_or(Color::default_background_color()),
                        corner_radius: flex_box.style.corner_radius,
                        border: flex_box.style.border,
                    },
                    inherited_color,
                ),
            ),
            KaolinElement::Text(text_element) => {
                Box::new(text_element.render(self.x, self.y, inherited_color))
            }
        }
    }
}

pub(crate) struct KaolinNodes<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub(crate) nodes: Vec<KaolinNode<'frame, Color>>,
}

impl<'frame, Color> KaolinNodes<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    fn new() -> Self {
        KaolinNodes { nodes: Vec::new() }
    }

    /// Add a new child
    fn push(&mut self, node: KaolinNode<'frame, Color>) {
        self.nodes.push(node);
    }

    /// Returns an array of mutable references of all growable children in the horizontal direction.
    fn get_growable_children_w(&mut self) -> Vec<&mut KaolinNode<'frame, Color>> {
        self.nodes.iter_mut().filter(|c| c.growable_width).collect()
    }

    /// Returns an array of mutable references of all growable children in the vertical direction.
    fn get_growable_children_h(&mut self) -> Vec<&mut KaolinNode<'frame, Color>> {
        self.nodes
            .iter_mut()
            .filter(|c| c.growable_height)
            .collect()
    }

    /// Returns an array of mutable references of all shrinkable children.
    fn get_shrinkable_children(&mut self) -> Vec<&mut KaolinNode<'frame, Color>> {
        self.nodes.iter_mut().filter(|c| c.shrinkable).collect()
    }

    /// Looks for the smallest and second smallest widths among the children.
    /// Important to make sure all children grow together.
    fn get_smallest_widths(children: &Vec<&mut KaolinNode<'frame, Color>>) -> (f64, f64) {
        let mut smallest = f64::INFINITY;
        let mut second_smallest = f64::INFINITY;
        for node in children {
            if node.current_width < smallest {
                second_smallest = smallest;
                smallest = node.current_width;
            } else if node.current_width < second_smallest && node.current_width > smallest {
                second_smallest = node.current_width;
            }
        }
        (smallest, second_smallest)
    }

    /// Looks for the smallest and second smallest heights among the children.
    /// Important to make sure all children grow together.
    fn get_smallest_heights(children: &Vec<&mut KaolinNode<'frame, Color>>) -> (f64, f64) {
        let mut smallest = f64::INFINITY;
        let mut second_smallest = f64::INFINITY;
        for node in children {
            if node.current_height < smallest {
                second_smallest = smallest;
                smallest = node.current_height;
            } else if node.current_height < second_smallest && node.current_height > smallest {
                second_smallest = node.current_height;
            }
        }
        (smallest, second_smallest)
    }

    /// Looks for the biggest and second biggest widths among the children.
    /// Important to make sure all children grow together.
    fn get_biggest_widths(children: &Vec<&mut KaolinNode<'frame, Color>>) -> (f64, f64) {
        let mut biggest = 0.0;
        let mut second_biggest = 0.0;
        for node in children {
            if node.current_width > biggest {
                second_biggest = biggest;
                biggest = node.current_width;
            } else if node.current_width > second_biggest && node.current_width < biggest {
                second_biggest = node.current_width;
            }
        }
        (biggest, second_biggest)
    }

    /// Returns the cumulative width of the child nodes.
    fn get_cumulative_width(&self) -> f64 {
        self.nodes.iter().map(|c| c.current_width).sum()
    }

    /// Returns the maximum width of the child nodes.
    fn get_max_width(&self) -> f64 {
        self.nodes
            .iter()
            .fold(0.0, |acc, c| acc.max(c.current_width))
    }

    /// Returns the cumulative height of the child nodes.
    fn get_cumulative_height(&self) -> f64 {
        self.nodes.iter().map(|c| c.current_height).sum()
    }

    /// Returns the maximum height of the child nodes.
    fn get_max_height(&self) -> f64 {
        self.nodes
            .iter()
            .fold(0.0, |acc, c| acc.max(c.current_height))
    }

    /// Returns an iterator over the child nodes.
    fn nodes(&mut self) -> impl Iterator<Item = &mut KaolinNode<'frame, Color>> {
        self.nodes.iter_mut()
    }

    /// Propagates the width growth of the element to its children.
    pub fn grow_w(&mut self) {
        self.nodes.iter_mut().for_each(|c| match c.element {
            KaolinElement::Flex(ref mut flex_box) => {
                flex_box.grow_children_width(c.current_width);
                c.fit_height(); // height fit depends on children (text elements will wrap upon width growth)
            }
            KaolinElement::Text(ref mut text_element) => {
                c.current_height = text_element.wrap_text(c.current_width); // height gets fit to the text itself
            }
        });
    }

    /// Propagates the height growth of the element to its children.
    pub fn grow_h(&mut self) {
        self.nodes.iter_mut().for_each(|c| {
            if let KaolinElement::Flex(ref mut flex_box) = c.element {
                flex_box.grow_children_height(c.current_height);
            }
        });
    }

    /// The number of children.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn render_nodes(
        &self,
        inherited_color: Color,
    ) -> Box<dyn Iterator<Item = RenderCommand<Color>> + '_> {
        Box::new(
            self.nodes
                .iter()
                .flat_map(move |node| node.render(inherited_color)),
        )
    }

    /// Returns the number of gaps that need to be accounted for.
    pub fn gaps(&self) -> u32 {
        if self.nodes.is_empty() {
            return 0;
        }
        self.nodes.len() as u32 - 1
    }
}
